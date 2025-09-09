use tracing::info;
use bb::barretenberg_api::{acir::{get_circuit_sizes, acir_get_slow_low_memory}, srs::init_srs};
use crate::backends::barretenberg::{srs::{setup_srs_from_bytecode, setup_srs, netsrs::NetSrs}, verify::{verify_ultra_honk, verify_ultra_honk_keccak, get_ultra_honk_verification_key, get_ultra_honk_keccak_verification_key}, prove::{prove_ultra_honk, prove_ultra_honk_keccak}, utils::compute_subgroup_size};
use acir::{FieldElement, native_types::{Witness, WitnessMap}};
use crate::{witness, circuit};
mod proof_utils;
use serde_json;
use std::path::PathBuf;
use std::env;

const BYTECODE: &str = "H4sIAAAAAAAA/62QQQqAMAwErfigpEna5OZXLLb/f4KKLZbiTQdCQg7Dsm66mc9x00O717rhG9ico5cgMOfoMxJu4C2pAEsKioqisnslysoaLVkEQ6aMRYxKFc//ZYQr29L10XfhXv4jB52E+OpMAQAA";

/// Helper function to construct robust paths to circuit files
/// This function tries multiple strategies to find the correct path:
/// 1. First tries to use CARGO_MANIFEST_DIR if available
/// 2. Falls back to searching from current directory
/// 3. Finally tries relative paths as last resort
fn get_circuit_path(filename: &str) -> PathBuf {
    // Try to use CARGO_MANIFEST_DIR first (most reliable)
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
    
    // Try to find from current directory by going up to workspace root
    if let Ok(current_dir) = env::current_dir() {
        let mut path = current_dir.clone();
        
        // Try to go up to workspace root and find circuits/target
        while path.parent().is_some() {
            let test_path = path.join("circuits").join("target").join(filename);
            if test_path.exists() {
                return test_path;
            }
            path = path.parent().unwrap().to_path_buf();
        }
    }
    
    // Fallback: try relative paths (less reliable but might work in some cases)
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
    
    // If all else fails, return the expected path and let the caller handle the error
    PathBuf::from(format!("circuits/target/{}", filename))
}


#[test]
fn test_acir_get_circuit_size() {
    let (_, constraint_system_buf) = circuit::decode_circuit(BYTECODE).unwrap();
    let circuit_sizes = unsafe { 
        get_circuit_sizes(&constraint_system_buf, false) 
    }; 
    assert_eq!(circuit_sizes.total, 3560);
    assert_eq!(circuit_sizes.subgroup, 4096);
}

#[test]
fn test_prove_and_verify_ultra_honk() {
    let _ = tracing_subscriber::fmt::try_init();

    // Setup SRS
    println!("setting up srs");
    setup_srs_from_bytecode(BYTECODE, None, false).unwrap();
    println!("srs setup");

    // Ultra Honk

    // Get the witness map from the vector of field elements
    // The vector items can be either a FieldElement, an unsigned integer
    // For hex or decimal strings, use from_vec_str_to_witness_map
    println!("getting initial witness");
    let initial_witness = witness::from_vec_to_witness_map(vec![5 as u128, 6 as u128, 30 as u128]).unwrap();
    println!("initial witness: {:?}", initial_witness);

    let start = std::time::Instant::now();
    println!("getting vk");
    let vk = get_ultra_honk_verification_key(BYTECODE, false).unwrap();
    println!("vk: {:?}", vk);
    assert_eq!(acir_get_slow_low_memory(), false);

    let proof = prove_ultra_honk(BYTECODE, initial_witness, vk.clone(), false).unwrap();
    info!("ultra honk proof generation time: {:?}", start.elapsed());
    assert_eq!(acir_get_slow_low_memory(), false);

    let verdict = verify_ultra_honk(proof, vk).unwrap();
    info!("honk proof verification verdict: {}", verdict);
}

#[test]
fn test_ultra_honk_keccak() {
    let _ = tracing_subscriber::fmt::try_init();

    // Read the JSON manifest of the circuit
    let keccak_circuit_path = get_circuit_path("keccak.json");
    let keccak_circuit_txt = std::fs::read_to_string(&keccak_circuit_path)
        .unwrap_or_else(|_| panic!("Failed to read circuit file at: {:?}", keccak_circuit_path));
    // Parse the JSON manifest into a dictionary
    let keccak_circuit: serde_json::Value = serde_json::from_str(&keccak_circuit_txt).unwrap();
    // Get the bytecode from the dictionary
    let keccak_circuit_bytecode = keccak_circuit["bytecode"].as_str().unwrap();
    
    // Setup SRS
    setup_srs_from_bytecode(keccak_circuit_bytecode, None, false).unwrap();

    // Ultra Honk

    // Get the witness map from the vector of field elements
    // The vector items can be either a FieldElement, an unsigned integer
    // For hex or decimal strings, use from_vec_str_to_witness_map
    let initial_witness = witness::from_vec_to_witness_map(vec![2 as u128, 5 as u128, 10 as u128, 15 as u128, 20 as u128]).unwrap();

    let start = std::time::Instant::now();
    let vk = get_ultra_honk_keccak_verification_key(keccak_circuit_bytecode, false, false).unwrap();
    assert_eq!(acir_get_slow_low_memory(), false);
    
    let proof = prove_ultra_honk_keccak(keccak_circuit_bytecode, initial_witness, vk.clone(), false, false).unwrap();
    info!("ultra honk proof generation time: {:?}", start.elapsed());
    assert_eq!(acir_get_slow_low_memory(), false);

    let verdict = verify_ultra_honk_keccak(proof, vk, false).unwrap();
    info!("honk proof verification verdict: {}", verdict);
}

#[test]
fn test_ultra_honk_low_memory() {
    let _ = tracing_subscriber::fmt::try_init();

    // Read the JSON manifest of the circuit
    let circuit_path = get_circuit_path("keccak_large.json");
    let circuit_txt = std::fs::read_to_string(&circuit_path)
        .unwrap_or_else(|_| panic!("Failed to read circuit file at: {:?}", circuit_path));
    // Parse the JSON manifest into a dictionary
    let circuit: serde_json::Value = serde_json::from_str(&circuit_txt).unwrap();
    // Get the bytecode from the dictionary
    let circuit_bytecode = circuit["bytecode"].as_str().unwrap();
    
    // Setup SRS
    setup_srs_from_bytecode(circuit_bytecode, None, false).unwrap();

    // Ultra Honk

    // Get the witness map from the vector of field elements
    // The vector items can be either a FieldElement, an unsigned integer
    // For hex or decimal strings, use from_vec_str_to_witness_map
    let initial_witness = witness::from_vec_to_witness_map(vec![2 as u128, 5 as u128, 10 as u128, 15 as u128, 20 as u128]).unwrap();

    let start = std::time::Instant::now();
    let vk = get_ultra_honk_verification_key(circuit_bytecode, true).unwrap();
    assert_eq!(acir_get_slow_low_memory(), true);
    
    let proof = prove_ultra_honk(circuit_bytecode, initial_witness, vk.clone(), true).unwrap();
    info!("ultra honk proof generation time: {:?}", start.elapsed());
    assert_eq!(acir_get_slow_low_memory(), true);

    let verdict = verify_ultra_honk(proof, vk).unwrap();
    info!("honk proof verification verdict: {}", verdict);
}

#[test]
fn test_srs_setup_from_bytecode() {
    let _ = tracing_subscriber::fmt::try_init();

    let start = std::time::Instant::now();
    let srs = setup_srs_from_bytecode(BYTECODE, None, false).unwrap();
    info!("srs setup time: {:?}", start.elapsed());
    // 2^5 + 1 = 33
    assert_eq!(srs, 4097);
}

#[test]
fn test_srs_setup_from_circuit_size() {
    let _ = tracing_subscriber::fmt::try_init();

    let start = std::time::Instant::now();
    let circuit_size = 22;
    let srs = setup_srs(circuit_size, None).unwrap();
    info!("srs setup time: {:?}", start.elapsed());
    // 2^5 + 1 = 33
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

/*#[test]
fn test_ultra_honk_recursive_proving() {
    // Read the JSON manifest of the circuit 
    let recursed_circuit_path = get_circuit_path("recursed.json");
    let recursed_circuit_txt = std::fs::read_to_string(&recursed_circuit_path)
        .unwrap_or_else(|_| panic!("Failed to read circuit file at: {:?}", recursed_circuit_path));
    // Parse the JSON manifest into a dictionary
    let recursed_circuit: serde_json::Value = serde_json::from_str(&recursed_circuit_txt).unwrap();
    // Get the bytecode from the dictionary
    let recursed_circuit_bytecode = recursed_circuit["bytecode"].as_str().unwrap();

    setup_srs(String::from(recursed_circuit_bytecode), None, true).unwrap();

    let mut initial_witness = WitnessMap::new();
    // x
    initial_witness.insert(Witness(0), FieldElement::from(5u128));
    // y
    initial_witness.insert(Witness(1), FieldElement::from(25u128));

    let (recursed_proof, recursed_vk) = prove_ultra_honk(recursed_circuit_bytecode, initial_witness, true).unwrap();

    let (proof_as_fields, vk_as_fields, key_hash) = recursion::generate_recursive_honk_proof_artifacts(recursed_proof, recursed_vk).unwrap();

    //println!("proof: {:?}", proof_as_fields);
    //println!("vk: {:?}", vk_as_fields);
    //println!("key_hash: {:?}", key_hash);
    
    assert_eq!(proof_as_fields.len(), 463);
    assert_eq!(vk_as_fields.len(), 128);
    //assert_eq!(key_hash, "0x25240793a378438025d0dbe8a4e197c93ec663864a5c9b01699199423dab1008");

    // Read the JSON manifest of the circuit 
    let recursive_circuit_path = get_circuit_path("recursive.json");
    let recursive_circuit_txt = std::fs::read_to_string(&recursive_circuit_path)
        .unwrap_or_else(|_| panic!("Failed to read circuit file at: {:?}", recursive_circuit_path));
    // Parse the JSON manifest into a dictionary
    let recursive_circuit: serde_json::Value = serde_json::from_str(&recursive_circuit_txt).unwrap();
    // Get the bytecode from the dictionary
    let recursive_circuit_bytecode = recursive_circuit["bytecode"].as_str().unwrap();
    println!("recursive_circuit_bytecode: {:?}", recursive_circuit_bytecode);

    // IMPORTANT: Likely to run into a timeout for the net srs, replace None with a path to a local srs file
    // before running this test
    setup_srs(String::from(recursive_circuit_bytecode), None, true).unwrap();

    let mut initial_witness_recursive = WitnessMap::new();
    let mut index = 0;
    // Verification key
    vk_as_fields.iter().for_each(|v| {
        initial_witness_recursive.insert(Witness(index), FieldElement::try_from_str(v).unwrap());
        index += 1;
    });
    // Proof
    proof_as_fields.iter().for_each(|v| {
        initial_witness_recursive.insert(Witness(index), FieldElement::try_from_str(v).unwrap());
        index += 1;
    });
    // Public inputs
    initial_witness_recursive.insert(Witness(index), FieldElement::from(25u128));
    index += 1;
    // Key hash
    initial_witness_recursive.insert(Witness(index), FieldElement::try_from_str(&key_hash).unwrap());

    let (proof, vk) = prove_ultra_honk(recursive_circuit_bytecode, initial_witness_recursive, true).unwrap();

    let verdict = verify_ultra_honk(proof, vk).unwrap();
    assert!(verdict);
}*/
