use std::ffi::{c_char, CStr};
use tracing::debug;

pub mod barretenberg_api;

#[no_mangle]
extern "C" fn logstr(char_ptr: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(char_ptr) };
    debug!("{}", c_str.to_str().unwrap());
}

#[no_mangle]
extern "C" fn env_hardware_concurrency() -> u32 {
    std::thread::available_parallelism()
        .map(|p| p.get() as u32)
        .unwrap_or(1)
}