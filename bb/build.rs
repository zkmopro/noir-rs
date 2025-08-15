use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const BB_DOWNLOAD_SCRIPT: &str = include_str!("./download_bb.sh");
const TARGET_LIST: [&str; 8] = [
    "aarch64-apple-darwin",
    "aarch64-apple-ios",
    "aarch64-apple-ios-sim",
    "aarch64-linux-android",
    "x86_64-apple-darwin",
    "x86_64-apple-ios",
    "x86_64-linux-android",
    "x86_64-unknown-linux-gnu",
];

fn main() {
    // Notify Cargo to rerun this build script if `build.rs` changes.
    println!("cargo:rerun-if-changed=build.rs");

    let target = env::var("TARGET").expect("TARGET not set");
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let arch = target
        .split('-')
        .next()
        .expect("Architecture cannot be parsed");

    // See: https://github.com/zkmopro/chkstk_stub for more details
    chkstk_stub::build();

    // Try to list contents of the target directory
    let bb_path = Path::new(&out_dir).join(Path::new("bb"));
    // If the bb path is not downloaded, download it
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
    let absolute_lib_path = if bb_path.join(&target).exists() {
        bb_path.join(&target)
    } else {
        bb_path.join(&arch)
    };

    // Add the library search path for Rust to find during linking.
    let lib_dir;
    if !(TARGET_LIST.contains(&target.as_str()) || TARGET_LIST.contains(&arch)) {
        panic!("Unsupported target: {}", target);
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
}
