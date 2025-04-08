use std::{env, path::PathBuf, process::Command};

fn main() {
    // Notify Cargo to rerun this build script if `build.rs` changes.
    println!("cargo:rerun-if-changed=build.rs");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Add the library search path for Rust to find during linking.
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let lib_dir;
    if !(target_os == "macos" || target_os == "ios") {
        panic!("Unsupported target OS: {}", target_os);
    }
    lib_dir = manifest_dir.join(target_os).join("lib");
    println!("cargo:rustc-link-search={}", lib_dir.display());

    // Link the `barretenberg` static library.
    println!("cargo:rustc-link-lib=static=barretenberg");

    // Link the C++ standard library.
    if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }

    // Copy the headers to the build directory.
    // Fix an issue where the headers are not included in the build.
    Command::new("sh")
        .args(&["copy-headers.sh", &"./include"])
        .output()
        .unwrap();
}
