use crate::backends::barretenberg::{
    api::get_slow_low_memory,
    prove::{prove_ultra_honk, prove_ultra_honk_keccak},
    srs::{setup_srs, setup_srs_from_bytecode},
    utils::compute_subgroup_size,
    verify::{
        get_ultra_honk_keccak_verification_key, get_ultra_honk_verification_key, verify_ultra_honk,
        verify_ultra_honk_keccak,
    },
};
use tracing::info;

use crate::{circuit, witness};
mod proof_utils;
use serde_json;
use std::env;
use std::path::PathBuf;

/// Load the product circuit bytecode from the compiled JSON artifact.
/// Run `cd circuits && nargo compile --package product` to regenerate.
fn load_product_bytecode() -> String {
    let circuit_path = get_circuit_path("product.json");
    let circuit_txt = std::fs::read_to_string(&circuit_path)
        .unwrap_or_else(|_| panic!("Failed to read circuit file at: {:?}", circuit_path));
    let circuit: serde_json::Value = serde_json::from_str(&circuit_txt).unwrap();
    circuit["bytecode"].as_str().unwrap().to_string()
}

/// Helper function to construct robust paths to circuit files
fn get_circuit_path(filename: &str) -> PathBuf {
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("..");
        path.push("circuits");
        path.push("target");
        path.push(filename);
        if path.exists() {
            return path;
        }
    }

    if let Ok(current_dir) = env::current_dir() {
        let mut path = current_dir.clone();
        while path.parent().is_some() {
            let test_path = path.join("circuits").join("target").join(filename);
            if test_path.exists() {
                return test_path;
            }
            path = path.parent().unwrap().to_path_buf();
        }
    }

    let relative_paths = [
        format!("../circuits/target/{}", filename),
        format!("../../circuits/target/{}", filename),
        format!("circuits/target/{}", filename),
    ];

    for relative_path in relative_paths.iter() {
        let path = PathBuf::from(relative_path);
        if path.exists() {
            return path;
        }
    }

    PathBuf::from(format!("circuits/target/{}", filename))
}

#[test]
fn test_acir_get_circuit_size() {
    use crate::backends::barretenberg::utils::get_circuit_size;
    let bytecode = load_product_bytecode();
    let circuit_size = get_circuit_size(&bytecode, false);
    assert!(circuit_size > 0, "Circuit size should be > 0");
}

#[test]
fn test_prove_and_verify_ultra_honk() {
    let _ = tracing_subscriber::fmt::try_init();

    let bytecode = load_product_bytecode();

    // Setup SRS
    setup_srs_from_bytecode(&bytecode, None, false).unwrap();

    // Get the witness map (product circuit: a * b == result, i.e. 5 * 6 == 30)
    let initial_witness =
        witness::from_vec_to_witness_map(vec![5 as u128, 6 as u128, 30 as u128]).unwrap();

    let start = std::time::Instant::now();
    let vk = get_ultra_honk_verification_key(&bytecode, false).unwrap();

    let proof = prove_ultra_honk(&bytecode, initial_witness, vk.clone(), false).unwrap();
    info!("ultra honk proof generation time: {:?}", start.elapsed());

    let verdict = verify_ultra_honk(proof, vk).unwrap();
    assert!(verdict, "Ultra Honk proof verification should succeed");
}

#[test]
fn test_ultra_honk_keccak() {
    let _ = tracing_subscriber::fmt::try_init();

    let keccak_circuit_path = get_circuit_path("keccak.json");
    let keccak_circuit_txt = std::fs::read_to_string(&keccak_circuit_path)
        .unwrap_or_else(|_| panic!("Failed to read circuit file at: {:?}", keccak_circuit_path));
    let keccak_circuit: serde_json::Value = serde_json::from_str(&keccak_circuit_txt).unwrap();
    let keccak_circuit_bytecode = keccak_circuit["bytecode"].as_str().unwrap();

    setup_srs_from_bytecode(keccak_circuit_bytecode, None, false).unwrap();

    let initial_witness = witness::from_vec_to_witness_map(vec![
        2 as u128, 5 as u128, 10 as u128, 15 as u128, 20 as u128,
    ])
    .unwrap();

    let start = std::time::Instant::now();
    let vk = get_ultra_honk_keccak_verification_key(keccak_circuit_bytecode, false, false).unwrap();

    let proof = prove_ultra_honk_keccak(
        keccak_circuit_bytecode,
        initial_witness,
        vk.clone(),
        false,
        false,
    )
    .unwrap();
    info!(
        "ultra honk keccak proof generation time: {:?}",
        start.elapsed()
    );

    let verdict = verify_ultra_honk_keccak(proof, vk, false).unwrap();
    assert!(
        verdict,
        "Ultra Honk Keccak proof verification should succeed"
    );
}

#[test]
fn test_ultra_honk_low_memory() {
    let _ = tracing_subscriber::fmt::try_init();

    let circuit_path = get_circuit_path("keccak_large.json");
    let circuit_txt = std::fs::read_to_string(&circuit_path)
        .unwrap_or_else(|_| panic!("Failed to read circuit file at: {:?}", circuit_path));
    let circuit: serde_json::Value = serde_json::from_str(&circuit_txt).unwrap();
    let circuit_bytecode = circuit["bytecode"].as_str().unwrap();

    setup_srs_from_bytecode(circuit_bytecode, None, false).unwrap();

    let initial_witness = witness::from_vec_to_witness_map(vec![
        2 as u128, 5 as u128, 10 as u128, 15 as u128, 20 as u128,
    ])
    .unwrap();

    let start = std::time::Instant::now();
    let vk = get_ultra_honk_verification_key(circuit_bytecode, true).unwrap();
    assert_eq!(get_slow_low_memory(), true);

    let proof = prove_ultra_honk(circuit_bytecode, initial_witness, vk.clone(), true).unwrap();
    info!(
        "ultra honk low memory proof generation time: {:?}",
        start.elapsed()
    );
    assert_eq!(get_slow_low_memory(), true);

    let verdict = verify_ultra_honk(proof, vk).unwrap();
    assert!(
        verdict,
        "Ultra Honk low memory proof verification should succeed"
    );
}

#[test]
fn test_srs_setup_from_bytecode() {
    let _ = tracing_subscriber::fmt::try_init();

    let bytecode = load_product_bytecode();
    let start = std::time::Instant::now();
    let srs = setup_srs_from_bytecode(&bytecode, None, false).unwrap();
    info!("srs setup time: {:?}", start.elapsed());
    assert!(srs > 0, "SRS num_points should be > 0");
}

#[test]
fn test_srs_setup_from_circuit_size() {
    let _ = tracing_subscriber::fmt::try_init();

    let start = std::time::Instant::now();
    let circuit_size = 22;
    let srs = setup_srs(circuit_size, None).unwrap();
    info!("srs setup time: {:?}", start.elapsed());
    assert_eq!(srs, 33);
}

#[test]
fn test_compute_subgroup_size() {
    let mut subgroup_size = compute_subgroup_size(22);
    assert_eq!(subgroup_size, 32);

    subgroup_size = compute_subgroup_size(50);
    assert_eq!(subgroup_size, 64);

    subgroup_size = compute_subgroup_size(100);
    assert_eq!(subgroup_size, 128);

    subgroup_size = compute_subgroup_size(1000);
    assert_eq!(subgroup_size, 1024);

    subgroup_size = compute_subgroup_size(10000);
    assert_eq!(subgroup_size, 16384);

    subgroup_size = compute_subgroup_size(100000);
    assert_eq!(subgroup_size, 131072);

    subgroup_size = compute_subgroup_size(200000);
    assert_eq!(subgroup_size, 262144);

    subgroup_size = compute_subgroup_size(500000);
    assert_eq!(subgroup_size, 524288);

    subgroup_size = compute_subgroup_size(1000000);
    assert_eq!(subgroup_size, 1048576);
}

// --- New tests from eng review ---

#[test]
fn test_invalid_witness_input_error() {
    let result = witness::from_vec_str_to_witness_map(vec!["not_a_number"]);
    assert!(result.is_err(), "Invalid witness input should return Err");
}

#[test]
fn test_circuit_format_round_trip() {
    // Decode program from bytecode, re-serialize, decode again, compare
    let bytecode = load_product_bytecode();
    let program1 = circuit::get_program(&bytecode).unwrap();
    let reserialized = acvm::acir::circuit::Program::serialize_program(&program1);
    let program2 = acvm::acir::circuit::Program::<acvm::acir::FieldElement>::deserialize_program(
        &reserialized,
    )
    .unwrap();
    assert_eq!(program1.functions.len(), program2.functions.len());
}
