use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const BB_DOWNLOAD_SCRIPT: &str = include_str!("./download_bb.sh");

fn main() {
    // Notify Cargo to rerun this build script if `build.rs` changes.
    println!("cargo:rerun-if-changed=build.rs");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    // Try to list contents of the target directory
    let bb_path = Path::new(&out_dir).join(Path::new("bb"));
    // If the rapidsnark repo is not downloaded, download it
    if !bb_path.exists() {
        let bb_script_path = Path::new(&out_dir).join(Path::new("download_bb.sh"));
        fs::write(&bb_script_path, BB_DOWNLOAD_SCRIPT).expect("Failed to write build script");
        let child_process = Command::new("sh")
            .arg(bb_script_path.to_str().unwrap())
            .spawn();
        if let Err(e) = child_process {
            panic!("Failed to spawn bb download: {}", e);
        }
        let status = child_process.unwrap().wait();
        if let Err(e) = status {
            panic!("Failed to wait for bb download: {}", e);
        } else if !status.unwrap().success() {
            panic!("Failed to wait for bb download");
        }
    }
    let absolute_lib_path = if bb_path.join(&target_os).exists() {
        bb_path.join(&target_os)
    } else {
        bb_path.join(&target_os)
    };

    // Add the library search path for Rust to find during linking.
    let lib_dir;
    if !(target_os == "macos" || target_os == "ios" || target_os == "android" || target_os == "linux") {
        panic!("Unsupported target OS: {}", target_os);
    }
    lib_dir = absolute_lib_path.join("lib");
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

    // refer to https://github.com/bbqsrc/cargo-ndk to see how to link the libc++_shared.so file in Android
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
}


fn android() {
    println!("cargo:rustc-link-lib=c++_shared");

    if let Ok(output_path) = env::var("CARGO_NDK_OUTPUT_PATH") {
        let sysroot_libs_path = PathBuf::from(env::var_os("CARGO_NDK_SYSROOT_LIBS_PATH").unwrap());
        let lib_path = sysroot_libs_path.join("libc++_shared.so");
        assert!(
            lib_path.exists(),
            "Error: Source file {:?} does not exist",
            lib_path
        );
        let dest_dir = Path::new(&output_path).join(env::var("CARGO_NDK_ANDROID_TARGET").unwrap());
        println!("cargo:rerun-if-changed={}", dest_dir.display());
        if !dest_dir.exists() {
            fs::create_dir_all(&dest_dir).unwrap();
        }
        fs::copy(lib_path, Path::new(&dest_dir).join("libc++_shared.so")).unwrap();
    }
}