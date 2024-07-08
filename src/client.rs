pub mod pixbox;

use chrono::Local;
use clap::Parser;
use photon_rs::native::{open_image, open_image_from_bytes, save_image};
use pixbox::storage_client::StorageClient;
use pixbox::Image;
use tonic::Request;

use crate::pixbox::ImageRequest;

// -----------------------------------------------------------------------------
// Base Command
// -----------------------------------------------------------------------------

/// Represents the command-line options.
#[derive(Debug, Parser)]
struct Options {
    /// The specific command to execute.
    #[clap(subcommand)]
    command: Command,
}

/// Defines the available commands.
#[derive(Debug, Parser)]
enum Command {
    /// Handles the upload of an image.
    ///
    /// Requires the path to the image as an argument.
    Upload(ReadImgPath),
    Download(GetImg),
}

// -----------------------------------------------------------------------------
// Upload Command
// -----------------------------------------------------------------------------

/// Stores the path to the image to be uploaded.
#[derive(Debug, Parser)]
struct ReadImgPath {
    /// The path to the image file.
    #[clap(long)]
    imgpath: String,
}

/// Retrives the edited image from the server.
#[derive(Debug, Parser)]
struct GetImg {
    /// The naeme of the image to be downloaded.
    #[clap(long)]
    img_id: String,
}

/// Uploads an image to the server.
///
/// This function connects to the storage server, opens the specified image,
/// converts it to a byte stream, and uploads it.
///
/// # Arguments
///
/// * `opts` - A `ReadImgPath` struct containing the path to the image.
///
/// # Returns
///
/// This function returns `Ok(())` if the image is successfully uploaded,
/// otherwise it returns an error wrapped in `Box<dyn std::error::Error>`.
async fn upload(
    client: &mut StorageClient<tonic::transport::Channel>,
    opts: ReadImgPath,
) -> Result<(), Box<dyn std::error::Error>> {
    let name = opts
        .imgpath
        .split('/')
        .last() // Get the last segment after the last '/'
        .unwrap(); // Unwrap the Option, assuming there's always a '/'

    let img = open_image(&opts.imgpath)?;
    let image = Image {
        name: name.to_string(),
        data: img.get_bytes(),
    };
    println!(
        "{} - INFO - Loading Image {}",
        Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
        image.name
    );

    let response = client.upload(image).await?;

    assert_eq!(response.into_inner().status, "Image saved");
    println!(
        "{} - INFO - Saving Edited Image",
        Local::now().format("[%Y-%m-%d][%H:%M:%S]")
    );
    Ok(())
}

async fn download(
    client: &mut StorageClient<tonic::transport::Channel>,
    opts: GetImg,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .download(Request::new(ImageRequest {
            image_id: opts.img_id.clone(),
        }))
        .await?;
    println!(
        "{} - INFO - Received the edited image from the server",
        Local::now().format("[%Y-%m-%d][%H:%M:%S]")
    );
    let image = response.into_inner();

    let img = open_image_from_bytes(&image.data).expect("File should open");
    let base_path = "./data/test-output/";
    let edited_image_path = format!("{}{}", base_path, opts.img_id);
    // Save the image at the given path.
    save_image(img, &edited_image_path).expect("Save failed");
    println!(
        "{} - INFO - Saved the edited image from the server to {}",
        Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
        edited_image_path
    );
    Ok(())
}

// -----------------------------------------------------------------------------
// Main
// -----------------------------------------------------------------------------

/// The entry point of the application.
///
/// Parses command-line arguments and executes the corresponding command.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Options::parse();

    // Establish a connection to the server
    let mut client = StorageClient::connect("http://127.0.0.1:9001").await?;
    println!(
        "{} - INFO - Connected to the PixStorage Server",
        Local::now().format("[%Y-%m-%d][%H:%M:%S]")
    );

    use Command::*;
    match opts.command {
        Upload(opts) => upload(&mut client, opts).await?,
        Download(opts) => download(&mut client, opts).await?,
    };
    Ok(())
}
