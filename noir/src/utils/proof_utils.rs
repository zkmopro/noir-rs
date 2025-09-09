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

/// Get the number of public inputs from a circuit's bytecode
/// This extracts the circuit information to count public parameters
pub fn get_num_public_inputs_from_circuit(circuit_bytecode: &str) -> Result<usize, String> {
    let program = get_program(circuit_bytecode)
        .map_err(|e| format!("Failed to get program: {}", e))?;

    // Get the first function (main function)
    let main_function = program.functions.first()
        .ok_or("No functions found in program")?;
    
    // Count public parameters directly from the circuit
    let num_public_inputs = main_function.public_parameters.0.len();

    Ok(num_public_inputs)
}

/// Parse a combined proof (proof + public inputs) into separated components
pub fn parse_proof_with_public_inputs(
    combined_proof: &[u8], 
    num_public_inputs: usize
) -> Result<ProofWithPublicInputs, String> {
    let public_inputs_size = num_public_inputs * 32; // Each field element is 32 bytes
    
    if combined_proof.len() < public_inputs_size {
        return Err(format!(
            "Combined proof too small: {} bytes, expected at least {} bytes for {} public inputs", 
            combined_proof.len(), 
            public_inputs_size,
            num_public_inputs
        ));
    }
    
    // Public inputs are at the beginning of the combined proof
    let public_inputs_bytes = combined_proof[..public_inputs_size].to_vec();
    let proof_data = combined_proof[public_inputs_size..].to_vec();
    
    if proof_data.len() % 32 != 0 {
        return Err("Proof data must be a multiple of 32 bytes".to_string());
    }

    let public_inputs: Vec<Vec<u8>> = public_inputs_bytes
        .chunks(32)
        .map(|chunk| chunk.to_vec())
        .collect();
    
    Ok(ProofWithPublicInputs {
        proof: proof_data,
        public_inputs: public_inputs,
        num_public_inputs,
    })
}

/// Combine proof and public inputs back into a single proof with public inputs
pub fn combine_proof_and_public_inputs(
    proof: Vec<u8>,
    public_inputs: Vec<Vec<u8>>,
) -> Vec<u8> {
    let mut combined = Vec::new();
    
    // Add public inputs first (32 bytes each)
    for public_input in &public_inputs {
        combined.extend_from_slice(public_input);
    }

    combined.extend_from_slice(&proof);
    
    combined
}
