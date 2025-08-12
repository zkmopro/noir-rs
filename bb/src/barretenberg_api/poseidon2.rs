use super::{
    bindgen,
    models::Fr,
    traits::{DeserializeBuffer, SerializeBuffer},
};

pub unsafe fn poseidon2_hash(inputs: &[Fr]) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    bindgen::poseidon2_hash(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());
    Fr::from_buffer(output)
}

pub unsafe fn poseidon2_hashes(inputs: &[Fr]) -> Fr {
    let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
    bindgen::poseidon2_hashes(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());
    Fr::from_buffer(output)
}
