use std::{env, path::PathBuf};

use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        //.proto_path("C:\\Users\\kevin\\.local\\bin")
        .build_client(true)
        .build_server(true)
        //.out_dir("./proto/") // if prefer to check-in generated artefacts
        .file_descriptor_set_path(out_dir.join("auction_v1_descriptor.bin"))
        .compile(
            &[
                "./proto/representation.proto",
                "./proto/extract.service.proto",
                "./proto/transaction.service.proto",
            ],
            &["proto"], // location to search proto dependencies
        )?;

    Ok(())
}