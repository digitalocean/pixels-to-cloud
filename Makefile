# Variables
IMAGE_PATH = ./images/diabhey.png

# Default target
all: server client

# Server and Client Build Targets
server:
	cargo build --release --bin server

client:
	cargo build --release --bin client

# Run the server
run:
	cargo run --release --bin server

# Upload an image using the client
upload:
	cargo run --release --bin client upload --imgpath $(IMAGE_PATH)

# Clean build artifacts using Cargo
clean:
	cargo clean

# Clean all artifacts, edited images
clean-all:
	cargo clean
	rm -rf images/edited/*

.PHONY: all server client run upload clean clean-all
