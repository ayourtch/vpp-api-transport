extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the VPP client library
    println!("cargo:rustc-flags=-L../../vpp/build-root/install-vpp-native/vpp/lib/ -lvppapiclient");

    let bindings = bindgen::Builder::default()
        .header("src/shmem_wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_file_name = out_path.join("bindings.rs");
    bindings
        .write_to_file(out_file_name.clone())
        .expect("Couldn't write bindings!");
}
