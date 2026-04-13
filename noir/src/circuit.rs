use std::io::Read;

use acvm::acir::{circuit::Program, FieldElement};
use base64::engine::{general_purpose, Engine};
use flate2::bufread::GzDecoder;

/// Get the acir buffer (compressed) from the circuit bytecode
pub fn get_acir_buffer(circuit_bytecode: &str) -> Result<Vec<u8>, String> {
    let acir_buffer = general_purpose::STANDARD
        .decode(circuit_bytecode)
        .map_err(|e| e.to_string())?;

    Ok(acir_buffer)
}

/// Uncompress the acir buffer
pub fn uncompress_acir_buffer(acir_buffer: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut decoder = GzDecoder::new(acir_buffer.as_slice());
    let mut acir_buffer_uncompressed = Vec::<u8>::new();
    decoder
        .read_to_end(&mut acir_buffer_uncompressed)
        .map_err(|e| e.to_string())?;

    Ok(acir_buffer_uncompressed)
}

/// Get the acir buffer (uncompressed) from the circuit bytecode.
///
/// Round-trips through `Program` deserialization/serialization to ensure
/// the output format matches the `NOIR_SERIALIZATION_FORMAT` env var
/// (e.g. msgpack-compact for barretenberg compatibility).
pub fn get_acir_buffer_uncompressed(circuit_bytecode: &str) -> Result<Vec<u8>, String> {
    let acir_buffer = get_acir_buffer(circuit_bytecode)?;
    // Round-trip through Program to re-serialize in the current format
    let program: Program<FieldElement> = Program::deserialize_program(&acir_buffer)
        .map_err(|e| format!("Failed to deserialize program: {}", e))?;
    let reserialized = Program::serialize_program(&program);
    uncompress_acir_buffer(reserialized)
}

/// Decode the circuit bytecode into compressed and uncompressed acir buffers
pub fn decode_circuit(circuit_bytecode: &str) -> Result<(Vec<u8>, Vec<u8>), String> {
    let acir_buffer = get_acir_buffer(circuit_bytecode)?;
    let acir_buffer_uncompressed = get_acir_buffer_uncompressed(circuit_bytecode)?;

    Ok((acir_buffer, acir_buffer_uncompressed))
}

/// Get the program from the circuit bytecode
pub fn get_program(circuit_bytecode: &str) -> Result<Program<FieldElement>, String> {
    let acir_buffer: Vec<u8> = get_acir_buffer(circuit_bytecode)?;
    Program::deserialize_program(&acir_buffer).map_err(|e| e.to_string())
}
