extern crate cc;
extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    // Automatically generate the binding.rs file
    let bindings = bindgen::Builder::default()
        .header("src/fibonacci.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("binding.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("src/fibonacci.c")
        .compile("libfibonacci.a");
}
