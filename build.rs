use std::env;
use std::path::PathBuf;

/// This is the main function of the program.
/// It compiles the protobuf files and generates the necessary Rust code for client and server.
///
/// # Arguments
///
/// * `proto_file` - A string representing the path to the protobuf file to be compiled.
/// * `out_dir` - A `PathBuf` representing the output directory where the generated code will be placed.
///
/// # Returns
///
/// * `Result<(), Box<dyn std::error::Error>>` - Returns a `Result` indicating success or failure.
///   On success, it returns `Ok(())`. On failure, it returns an error wrapped in a `Box`.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_file = "./proto/pixbox.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Configure the tonic_build with experimental_allow_proto3_optional flag for older systems
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_client(true) // Generate client code
        .build_server(true) // Generate server code
        .file_descriptor_set_path(out_dir.join("pixbox_descriptor.bin")) // Output file for file descriptor set
        .out_dir("./src") // Output directory for generated code
        .compile(&[proto_file], &["proto"])?; // Compile the protobuf file

    Ok(())
}
