use std::{fs, path::Path};

// Deprecated. PB serialization will happen on the Rust side.
fn compile_protos() {
    let proto_path = Path::new("../../schemas/proto");
    let out_dir = Path::new("python/foxglove/_protobuf");

    // Create output directory if it doesn't exist
    fs::create_dir_all(out_dir).unwrap();

    // Find all .proto files
    let mut proto_files = Vec::new();
    for entry in walkdir::WalkDir::new(proto_path) {
        let entry = entry.unwrap();
        let path = entry.path().to_owned();
        if path.extension().and_then(|s| s.to_str()) == Some("proto") {
            proto_files.push(path);
        }
    }
    proto_files.sort();

    // Generate __init__.py imports
    let mut init_imports = Vec::new();
    for path in &proto_files {
        let proto_name = path.file_stem().unwrap().to_str().unwrap();
        init_imports.push(format!(
            "from .foxglove.{}_pb2 import {}",
            proto_name, proto_name
        ));
    }

    // Write __init__.py
    let init_path = out_dir.join("__init__.py");
    fs::write(init_path, init_imports.join("\n") + "\n").unwrap();
}

fn main() {
    compile_protos();
}
