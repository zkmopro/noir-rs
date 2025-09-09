use crate::utils::proof_utils::*;
use crate::witness::from_vec_to_witness_map;
use crate::backends::barretenberg::{
    srs::setup_srs_from_bytecode,
    prove::prove_ultra_honk_keccak,
    verify::{verify_ultra_honk_keccak, get_ultra_honk_keccak_verification_key},
};

// Multiplier2 circuit bytecode
// This circuit multiplies x * y == result with result as public input
const MULTIPLIER2_BYTECODE: &str = "H4sIAAAAAAAA/62QQQqAMAwErfigpEna5OZXLLb/f4KKLZbiTQdCQg7Dsm66mc9x00O717rhG9ico5cgMOfoMxJu4C2pAEsKioqisnslysoaLVkEQ6aMRYxKFc//ZYQr29L10XfhXv4jB52E+OpMAQAA";

#[test]
fn test_get_num_public_inputs_multiplier2() {
    let num_public_inputs = get_num_public_inputs_from_circuit(MULTIPLIER2_BYTECODE).unwrap();
    
    // Multiplier2 circuit has 1 public input: result
    assert_eq!(num_public_inputs, 1, "Multiplier2 circuit should have 1 public inputs");
}

#[test]
fn test_prove_and_verify_ultra_honk_keccak_multiplier2() {
    let num_public_inputs = get_num_public_inputs_from_circuit(MULTIPLIER2_BYTECODE).unwrap();
    assert_eq!(num_public_inputs, 1, "Multiplier2 circuit should have 1 public inputs");

    setup_srs_from_bytecode(MULTIPLIER2_BYTECODE, None, false).unwrap();

    let initial_witness = from_vec_to_witness_map(vec![3 as u128, 5 as u128, 15 as u128]).unwrap();
    let vk = get_ultra_honk_keccak_verification_key(MULTIPLIER2_BYTECODE, false, false).unwrap();
    let proof = prove_ultra_honk_keccak(MULTIPLIER2_BYTECODE, initial_witness, vk.clone(), false, false).unwrap();

    // Parse the proof into separated components
    let proof_with_public_inputs = parse_proof_with_public_inputs(&proof, num_public_inputs).unwrap();

    // Combine the proof and public inputs back into a single byte vector
    let combined_proof = combine_proof_and_public_inputs(proof_with_public_inputs.proof, proof_with_public_inputs.public_inputs);

    let verdict = verify_ultra_honk_keccak(combined_proof, vk, false).unwrap();
    assert_eq!(verdict, true, "Multiplier2 circuit should be valid");
}
