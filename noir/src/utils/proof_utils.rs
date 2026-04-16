use crate::circuit::get_program;

/// Struct representing a proof with separated public inputs
#[derive(Debug, Clone)]
pub struct ProofWithPublicInputs {
    /// The proof without public inputs
    pub proof: Vec<u8>,
    /// The public inputs as an array of 32-byte values
    pub public_inputs: Vec<Vec<u8>>,
    /// The number of public inputs
    pub num_public_inputs: usize,
}

/// Get the number of public inputs from a circuit's bytecode.
/// This extracts the circuit information to count public parameters.
pub fn get_num_public_inputs_from_circuit(circuit_bytecode: &str) -> Result<usize, String> {
    let program =
        get_program(circuit_bytecode).map_err(|e| format!("Failed to get program: {}", e))?;

    let main_function = program
        .functions
        .first()
        .ok_or("No functions found in program")?;

    let num_public_inputs = main_function.public_parameters.0.len();

    Ok(num_public_inputs)
}

/// Parse a proof (produced by `prove_ultra_honk` / `prove_ultra_honk_keccak`) into
/// separated components.
///
/// The expected proof format is:
///   `[num_public_inputs (4 bytes BE)] [public_inputs (32 bytes each)] [proof fields]`
///
/// The `num_public_inputs` parameter is retained for API compatibility and cross-checked
/// against the 4-byte prefix embedded in the proof.
pub fn parse_proof_with_public_inputs(
    combined_proof: &[u8],
    num_public_inputs: usize,
) -> Result<ProofWithPublicInputs, String> {
    if combined_proof.len() < 4 {
        return Err("Combined proof too short to contain public inputs count".to_string());
    }
    let embedded_num_pub = u32::from_be_bytes(
        combined_proof[0..4]
            .try_into()
            .map_err(|_| "Failed to read num_public_inputs prefix")?,
    ) as usize;

    if embedded_num_pub != num_public_inputs {
        return Err(format!(
            "Public input count mismatch: caller passed {}, proof embeds {}",
            num_public_inputs, embedded_num_pub
        ));
    }

    let public_inputs_size = num_public_inputs * 32;
    let required_len = 4 + public_inputs_size;

    if combined_proof.len() < required_len {
        return Err(format!(
            "Combined proof too small: {} bytes, expected at least {} bytes for {} public inputs",
            combined_proof.len(),
            required_len,
            num_public_inputs
        ));
    }

    let public_inputs_bytes = combined_proof[4..required_len].to_vec();
    let proof_data = combined_proof[required_len..].to_vec();

    if proof_data.len() % 32 != 0 {
        return Err("Proof data must be a multiple of 32 bytes".to_string());
    }

    let public_inputs: Vec<Vec<u8>> = public_inputs_bytes
        .chunks(32)
        .map(|chunk| chunk.to_vec())
        .collect();

    Ok(ProofWithPublicInputs {
        proof: proof_data,
        public_inputs,
        num_public_inputs,
    })
}

/// Combine proof bytes and public inputs back into a single proof.
///
/// Produces the format expected by `verify_ultra_honk` / `verify_ultra_honk_keccak`:
///   `[num_public_inputs (4 bytes BE)] [public_inputs (32 bytes each)] [proof fields]`
pub fn combine_proof_and_public_inputs(proof: Vec<u8>, public_inputs: Vec<Vec<u8>>) -> Vec<u8> {
    let num_pub = public_inputs.len() as u32;
    let mut combined = num_pub.to_be_bytes().to_vec();

    for public_input in &public_inputs {
        combined.extend_from_slice(public_input);
    }

    combined.extend_from_slice(&proof);

    combined
}
