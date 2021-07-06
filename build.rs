//! This build script's primary purpose
//! is to generate the bindings necessary
//! for use in Linux's `perf_event_open()`
//! system call. What it does may evolve.
extern crate bindgen;

use std::fs::create_dir;
use std::path::{Path, PathBuf};

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrappers/perf_event.h");

    if !Path::new("./src/bindings/").exists() {
        match create_dir("./src/bindings") {
            Err(err) => eprintln!("{:?}", err),
            Ok(_) => (),
        }
    }
    let perf_bindings = bindgen::Builder::default()
        .header("./wrappers/perf_event.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .derive_default(true)
        .generate()
        .expect("Unable to generate perf_event bindings");

    let out_path = PathBuf::from("./src/bindings");

    perf_bindings
        .write_to_file(out_path.join("perf_event.rs"))
        .expect("Unable to write perf_event bindings to ./src/bindings/perf_event.rs");
}
