extern crate bindgen;

use std::env;
use std::path::PathBuf;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-search=native=cspice_linux_gcc_64bit/lib");
    println!("cargo:rustc-link-lib=static=cspice");
    //println!("cargo:rustc-flags=-L cspice_linux_gcc_64bit/lib");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("spice_wrapper.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");


    let out_path = PathBuf::from(out_dir.clone());
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("spice_bindings.rs"))
        .expect("Couldn't write bindings!");
}
