use crate::backends::barretenberg::{
    prove::prove_ultra_honk_keccak,
    srs::setup_srs_from_bytecode,
    verify::{get_ultra_honk_keccak_verification_key, verify_ultra_honk_keccak},
};
use crate::utils::proof_utils::*;
use crate::witness::from_vec_to_witness_map;

use super::load_product_bytecode;

// Product circuit: a * b == result, with `result` as public input

#[test]
fn test_get_num_public_inputs_product() {
    let bytecode = load_product_bytecode();
    let num_public_inputs = get_num_public_inputs_from_circuit(&bytecode).unwrap();

    // Product circuit has 1 public input: result
    assert_eq!(
        num_public_inputs, 1,
        "Product circuit should have 1 public input"
    );
}

#[test]
fn test_prove_and_verify_ultra_honk_keccak_product() {
    let bytecode = load_product_bytecode();
    let num_public_inputs = get_num_public_inputs_from_circuit(&bytecode).unwrap();
    assert_eq!(
        num_public_inputs, 1,
        "Product circuit should have 1 public input"
    );

    setup_srs_from_bytecode(&bytecode, None, false).unwrap();

    // a=3, b=5, result=15
    let initial_witness = from_vec_to_witness_map(vec![3 as u128, 5 as u128, 15 as u128]).unwrap();
    let vk = get_ultra_honk_keccak_verification_key(&bytecode, false, false).unwrap();
    let proof =
        prove_ultra_honk_keccak(&bytecode, initial_witness, vk.clone(), false, false).unwrap();

    // Parse the proof into separated components
    let proof_with_public_inputs =
        parse_proof_with_public_inputs(&proof, num_public_inputs).unwrap();

    // Combine the proof and public inputs back into a single byte vector
    let combined_proof = combine_proof_and_public_inputs(
        proof_with_public_inputs.proof,
        proof_with_public_inputs.public_inputs,
    );

    let verdict = verify_ultra_honk_keccak(combined_proof, vk, false).unwrap();
    assert_eq!(verdict, true, "Product circuit should verify");
}
