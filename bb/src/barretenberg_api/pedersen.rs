use super::{
    bindgen,
    models::{Fr, Point},
    traits::SerializeBuffer,
};
use crate::barretenberg_api::traits::DeserializeBuffer;

pub unsafe fn pedersen_commit(inputs: &[Fr]) -> Point {
    let mut output: <Point as DeserializeBuffer>::Slice = [0; 64];
    let offset: u32 = 0;
    bindgen::pedersen_commit(
        inputs.to_buffer().as_slice().as_ptr(),
        offset.to_be_bytes().as_ptr() as *const u32,
        output.as_mut_ptr(),
    );
    Point::from_buffer(output)
}

pub unsafe fn pedersen_hash(inputs: &[Fr], hash_index: u32) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    bindgen::pedersen_hash(
        inputs.to_buffer().as_slice().as_ptr(),
        hash_index.to_be_bytes().as_ptr() as *const u32,
        output.as_mut_ptr(),
    );
    Fr::from_buffer(output)
}

pub unsafe fn pedersen_hashes(inputs: &[Fr], hash_index: u32) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    bindgen::pedersen_hashes(
        inputs.to_buffer().as_slice().as_ptr(),
        hash_index.to_be_bytes().as_ptr() as *const u32,
        output.as_mut_ptr(),
    );
    Fr::from_buffer(output)
}

pub unsafe fn pedersen_hash_buffer(inputs: &[u8], hash_index: u32) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    bindgen::pedersen_hashes(
        inputs.as_ptr(),
        hash_index.to_be_bytes().as_ptr() as *const u32,
        output.as_mut_ptr(),
    );
    Fr::from_buffer(output)
}
