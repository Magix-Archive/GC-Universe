use std::error::Error;

fn main() {
    make_protobuf().unwrap();
}

fn make_protobuf() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/protos/PacketHead.proto");

    // Run definition code generation.
    protobuf_codegen::Codegen::new()
        .protoc()
        .protoc_path(&protoc_bin_vendored::protoc_bin_path()?)

        .includes(&["src/protos"])
        .input("src/protos/PacketHead.proto")
        .cargo_out_dir("protos_target")

        .run_from_script();

    Ok(())
}
