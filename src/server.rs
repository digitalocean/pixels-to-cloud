use crate::pixbox::storage_server::Storage;
use crate::pixbox::{Image, ImageRequest, StorageResponse};
use chrono::Local;
use rand::seq::SliceRandom;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

// -----------------------------------------------------------------------------
// Error Messages
// -----------------------------------------------------------------------------

/// Error message for when an empty or invalid image is provided.
const EMPTY_IMAGE_ERR: &str = "provided image was invalid";

// -----------------------------------------------------------------------------
// StorageServer Implementation
// -----------------------------------------------------------------------------

/// Represents the storage server for PixBox.
///
/// This struct holds the storage mechanism (an in-memory image) that can be
/// accessed and modified concurrently.
#[derive(Debug)]
pub struct PixBoxStorage {
    storage: Arc<Mutex<Image>>,
}

/// Provides default initialization for `PixBoxStorage`.
impl Default for PixBoxStorage {
    /// Creates a new `PixBoxStorage` instance with default values.
    ///
    /// Initializes the storage with an empty `Image`.
    fn default() -> Self {
        PixBoxStorage {
            storage: Arc::new(Mutex::new(Image {
                name: String::new(),
                data: vec![],
            })),
        }
    }
}

/// Implementation of the gRPC service trait `Storage` for `PixBoxStorage`.
#[tonic::async_trait]
impl Storage for PixBoxStorage {
    /// Asynchronously handles the upload of an image, applying a filter and saving the edited image.
    ///
    /// Parameters:
    /// - `request`: A `Request` wrapping a stream of `Image` chunks to be uploaded.
    ///
    /// Returns:
    /// - A `Result` wrapping a `Response<StorageResponse>` indicating the outcome of the upload operation.
    ///   On success, it contains a message indicating the image was saved. On failure, it returns a `Status`
    ///   with an appropriate error message.
    ///
    /// This method reads the streamed image data, checks for validity, applies a filter to the image,
    /// and saves the edited image to disk.
    async fn upload(&self, request: Request<Image>) -> Result<Response<StorageResponse>, Status> {
        let image = request.into_inner();

        let mut img = self.storage.lock().await;
        img.name = image.name;
        img.data = image.data;

        if img.data.is_empty() {
            return Err(Status::invalid_argument(EMPTY_IMAGE_ERR));
        }

        let mut image = photon_rs::native::open_image_from_bytes(&img.data)
            .expect("can't open image from raw data");

        let filters = [
            "oceanic",
            "islands",
            "marine",
            "seagreen",
            "flagblue",
            "liquid",
            "diamante",
            "radio",
            "twenties",
            "rosetint",
            "mauve",
            "bluechrome",
            "vintage",
            "perfume",
            "serenity",
        ];

        let mut rng = rand::thread_rng(); // Create a random number generator
        let chosen_filter = filters.choose(&mut rng).expect("Array should not be empty");

        photon_rs::filters::filter(&mut image, &chosen_filter);

        let base_path = "./images/edited/";
        let edited_image = format!("{}{}-{}", base_path, chosen_filter, img.name);
        println!(
            "{} - INFO - Applied filter: {} to {}",
            Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            chosen_filter,
            edited_image
        );
        photon_rs::native::save_image(image, &edited_image).expect("failed to save edited image");

        Ok(Response::new(StorageResponse {
            status: "Image saved".into(),
        }))
    }

    /// Asynchronously handles the download of an image.
    ///
    /// This function takes an `ImageRequest` as a parameter and returns a `Result` containing a `Response<Image>` and a `Status`.
    /// It opens the image specified by the `image_id` in the `ImageRequest` from the local storage, creates an `Image` object with the data and name,
    /// and sends it to the client.
    ///
    /// # Parameters
    ///
    /// * `request`: A `Request` wrapping an `ImageRequest` containing the ID of the image to be downloaded.
    ///
    /// # Returns
    ///
    /// * `Result`: A `Result` containing a `Response<Image>` and a `Status`.
    ///   - On success, it contains a `Response` wrapping an `Image` object representing the downloaded image.
    ///   - On failure, it returns a `Status` with an appropriate error message.
    ///
    async fn download(&self, request: Request<ImageRequest>) -> Result<Response<Image>, Status> {
        let base_path = "./images/edited/";
        let edited_image_path = format!("{}{}", base_path, request.into_inner().image_id);
        let e_image = photon_rs::native::open_image(&edited_image_path).expect("can't open image");
        let d_image = Image {
            name: edited_image_path.to_string(),
            data: e_image.get_bytes(),
        };
        println!(
            "{} - INFO - Image retrieved from {}",
            Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
            edited_image_path,
        );
        Ok(Response::new(d_image))
    }
}
