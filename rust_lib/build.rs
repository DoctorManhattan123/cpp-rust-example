use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("../include/my_cpp_lib.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("generated");
    fs::create_dir_all(&out_path).expect("Failed to create 'generated' directory");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

