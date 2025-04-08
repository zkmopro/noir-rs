use super::{
    bindgen,
    models::Fr,
    traits::{DeserializeBuffer, SerializeBuffer},
};

pub fn blake2s(inputs: &[u8]) -> [u8; 32] {
    let mut output = [0; 32];
    unsafe { bindgen::blake2s(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr()) };
    output
}

pub fn blake2s_to_field(inputs: &[u8]) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    unsafe {
        bindgen::blake2s_to_field_(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr())
    };
    Fr::from_buffer(output)
}
