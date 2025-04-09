// use super::{
//     bindgen,
//     models::Fr,
//     traits::{DeserializeBuffer, SerializeBuffer},
// };

// pub unsafe fn poseidon_hash(inputs: &[Fr]) -> Fr {
//     let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
//     bindgen::poseidon_hash(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());
//     Fr::from_buffer(output)
// }

// pub unsafe fn poseidon_hashes(inputs: &[Fr]) -> Fr {
//     let mut output: <Fr as DeserializeBuffer>::Slice = [0; 32];
//     bindgen::poseidon_hashes(inputs.to_buffer().as_slice().as_ptr(), output.as_mut_ptr());
//     Fr::from_buffer(output)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_poseidon_hash() {
//         let fr1 = Fr { data: [0; 32] }; // Initialize with zeros
//         let fr2 = Fr { data: [1; 32] }; // Initialize with zeros
//         let fr3 = Fr { data: [2; 32] }; // Initialize with zeros
//         let inputs = vec![fr1, fr2, fr3];
//         let hash = unsafe { poseidon_hash(&inputs) };
//         println!("hash: {:?}", hash);
//     }
// }
