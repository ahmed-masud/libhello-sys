extern crate autotools;
extern crate bindgen;

use std::path::PathBuf;
use std::env::var;

fn main() {
    
    let dst = autotools::Config::new("libhello")
        .reconf("-vi")
        .fast_build(true)
        .enable_static()
        .disable_shared()
        .build();

    // Simply link the library without using pkg-config
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=hello");
    println!("cargo:rustc-link-lib=c");

    // generate bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header("libhello/hello.h")
        .generate()
        .expect("unable to generate bindings");

    // setup the path to write bindings into
    let out_path = PathBuf::from(var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}

