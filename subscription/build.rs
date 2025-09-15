use std::error::Error;

fn build_subscription_service() -> Result<(), Box<dyn Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/api/gen")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .file_descriptor_set_path("src/api/gen/description.bin")
        .protoc_arg("--experimental_allow_proto3_optional")
        .compile_protos(&["proto/api.proto"], &["proto", "../"])?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=proto/api.proto");
    println!("cargo:rerun-if-changed=proto/");
    build_subscription_service()?;
    Ok(())
}
