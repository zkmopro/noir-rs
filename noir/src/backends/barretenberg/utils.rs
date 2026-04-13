use crate::backends::barretenberg::api::{self, settings_ultra_honk_poseidon2};
use crate::circuit::decode_circuit;

pub fn compute_subgroup_size(circuit_size: u32) -> u32 {
    let log_value = (circuit_size as f64).log2().ceil() as u32;
    2u32.pow(log_value)
}

/// Get the circuit size suitable for SRS allocation.
///
/// Returns `num_gates_dyadic` (the next power of 2 above num_gates). UltraHonk needs
/// SRS points for witness, permutation, and lookup polynomials, so this dyadic size
/// gives a safe lower bound for SRS setup.
pub fn get_circuit_size(circuit_bytecode: &str, _recursion: bool) -> u32 {
    let (_, acir_buffer_uncompressed) = if let Ok(decoded) = decode_circuit(circuit_bytecode) {
        decoded
    } else {
        return 0;
    };

    let settings = settings_ultra_honk_poseidon2();

    match api::circuit_stats(&acir_buffer_uncompressed, &settings) {
        Ok(info) => info.num_gates_dyadic,
        Err(_) => 0,
    }
}

/// Get the dyadic (next power-of-two) circuit size for the given bytecode.
pub fn get_circuit_size_dyadic(circuit_bytecode: &str) -> u32 {
    let (_, acir_buffer_uncompressed) = if let Ok(decoded) = decode_circuit(circuit_bytecode) {
        decoded
    } else {
        return 0;
    };

    let settings = settings_ultra_honk_poseidon2();

    match api::circuit_stats(&acir_buffer_uncompressed, &settings) {
        Ok(info) => info.num_gates_dyadic,
        Err(_) => 0,
    }
}

pub fn get_subgroup_size(circuit_bytecode: &str, recursion: bool) -> u32 {
    let circuit_size = get_circuit_size(circuit_bytecode, recursion);
    compute_subgroup_size(circuit_size)
}
