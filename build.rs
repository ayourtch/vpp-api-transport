extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn find_vpp_lib_dir() -> String {
    /*
     * In the future there's more cleverness possibly to be added.
     * For now this will do.
     */
    let path = "/usr/lib/x86_64-linux-gnu/".to_string();
    path
}

fn git_version() -> String {
    use std::process::Command;

    let describe_output = Command::new("git")
        .arg("describe")
        .arg("--all")
        .arg("--long")
        .output()
        .unwrap();

    let mut describe = String::from_utf8_lossy(&describe_output.stdout).to_string();
    describe.pop();
    describe
}

fn main() {
    let vpp_lib_dir = match env::var("VPP_LIB_DIR") {
        Ok(val) => val,
        Err(_e) => find_vpp_lib_dir(),
    };
    if !std::path::Path::new(&format!("{}/libvppapiclient.so", &vpp_lib_dir)).exists() {
        panic!("Can not find libvppapiclient.so at {}, please install python3-vpp-api or define VPP_LIB_DIR accordingly", vpp_lib_dir)
    }
    let flags = format!("cargo:rustc-flags=-L{} -lvppapiclient", &vpp_lib_dir);

    // Tell cargo to tell rustc to link the VPP client library
    println!("{}", flags);

    println!("cargo:rustc-env=GIT_VERSION=version {}", &git_version());

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
