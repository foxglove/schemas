// use std::io::{ErrorKind, Read};
use std::path::Path;
// use std::process::Command;

fn compile_protos() {
    let proto_path = Path::new("../../schemas/proto");
    let out_dir = Path::new("python/foxglove/_protobuf");

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(out_dir).unwrap();

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

    /*
    // Compile protos
    let mut command = Command::new("protoc");
    command
        .arg(format!("--proto_path={}", proto_path.display()))
        .arg(format!("--python_out={}", out_dir.display()))
        .arg(format!("--pyi_out={}", out_dir.display()));

    // Add all proto files to the command
    for proto_file in &proto_files {
        println!("cargo:rerun-if-changed={}", proto_file.display());
        command.arg(proto_file);
    }

    let status = command
        .status()
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => {
                panic!("The command `protoc` was not found. Probably you need to install it.")
            }
            _ => panic!("protoc failed: {}", e),
        })
        .unwrap();
    if !status.success() {
        panic!("protoc failed!");
    }

    // Fix imports in generated .py files
    for proto_file in proto_files {
        // python name will look like ./foxglove/ProtoName_pb2.py
        let py_file = out_dir.join("foxglove").join(format!(
            "{}_pb2.py",
            proto_file.file_stem().unwrap().to_str().unwrap()
        ));

        let mut file = std::fs::File::open(&py_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let new_contents = contents.replace("from foxglove import ", "from . import ");
        std::fs::write(py_file, new_contents).unwrap();
    }
    */

    // Write __init__.py
    let init_path = out_dir.join("__init__.py");
    std::fs::write(init_path, init_imports.join("\n") + "\n").unwrap();
}

fn main() {
    compile_protos();
}
