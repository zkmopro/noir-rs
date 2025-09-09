pub mod proof_utils;

// Re-export commonly used items for convenience
pub use proof_utils::{
    ProofWithPublicInputs,
    get_num_public_inputs_from_circuit,
    parse_proof_with_public_inputs,
    combine_proof_and_public_inputs,
};