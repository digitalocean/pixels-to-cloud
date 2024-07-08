#!/bin/bash

# Function to print messages with a timestamp
log_message() {
  local MESSAGE=$1
  echo "[$(date '+%Y-%m-%d')][$(date '+%H:%M:%S')] - INFO - $MESSAGE"
}

# Ensure rclone is configured correctly
rclone config show | grep -q 'pixbox-ams' || { log_message "Rclone configuration for pixbox-ams is missing"; exit 1; }
rclone config show | grep -q 'pixbox-syd' || { log_message "Rclone configuration for pixbox-syd is missing"; exit 1; }
rclone config show | grep -q 'pixbox-nyc' || { log_message "Rclone configuration for pixbox-nyc is missing"; exit 1; }

# Check if directory to watch is provided as an argument
if [ -z "$1" ]; then
  log_message "Usage: $0 <directory_to_watch>"
  exit 1
fi

WATCH_DIR=$1

# Check if the directory exists
if [ ! -d "$WATCH_DIR" ]; then
  log_message "The directory $WATCH_DIR does not exist."
  exit 1
fi

# Function to get relative path
get_relative_path() {
  local FILE=$1
  local DIR=$2
  local RELATIVE_PATH=$(echo "$FILE" | sed "s|^$DIR/||")
  echo "$RELATIVE_PATH"
}

# Function to upload a file to the three buckets
upload_file() {
  local FILE=$1
  local RELATIVE_FILE=$(get_relative_path "$FILE" "$WATCH_DIR")
  log_message "Uploading $WATCH_DIR to DigitalOcean Spaces pixbox"
  rclone copy "$FILE" pixbox-ams:pixbox
}

# Function to synchronize all three buckets
sync_buckets() {
  log_message "Synchronizing pixbox-ams to pixbox-syd..."
  rclone sync pixbox-ams:pixbox pixbox-syd:pixbox-syd1
  
  log_message "Synchronizing pixbox-ams to pixbox-nyc..."
  rclone sync pixbox-ams:pixbox pixbox-nyc:pixbox-nyc3
}

# Initial synchronization
sync_buckets

# Watch the directory for new files and upload them
fswatch -0 -e ".*" -i "\\.*/[^/]*$" "$WATCH_DIR" | while read -d "" NEW_FILE
do
  RELATIVE_FILE=$(get_relative_path "$NEW_FILE" "$WATCH_DIR")
  log_message "New file detected: $WATCH_DIR"
  upload_file "$NEW_FILE"
  sync_buckets
done
