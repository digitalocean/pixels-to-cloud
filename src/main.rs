use tonic::transport::Server;

use pixbox::storage_server::StorageServer;
use server::PixBoxStorage;

// Public modules for the gRPC service and server implementation.
pub mod pixbox;
pub mod server;

mod store_proto {
    // Include the generated Rust code from the protobuf definition.
    include!("pixbox.rs");

    // Encoded file descriptor set for the protobuf definitions.
    // This is used for server reflection.
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("pixbox_descriptor");
}

/// The entry point for the PixBox storage server.
///
/// This function sets up and runs the gRPC server for the PixBox storage service.
/// It includes a reflection service for dynamic service discovery.
///
/// # Errors
/// Returns an error if the server fails to bind to the address or if there is an issue
/// starting the server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Server address to bind to.
    let addr = "127.0.0.1:9001".parse()?;
    // Initialize the storage service.
    let storage = PixBoxStorage::default();

    // Setup the reflection service for gRPC.
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(store_proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    // Build and run the server with the storage and reflection services.
    Server::builder()
        .add_service(StorageServer::new(storage))
        .add_service(reflection_service)
        .serve(addr)
        .await?;
    Ok(())
}
