use super::bindgen;

pub unsafe fn example_simple_create_and_verify_proof() -> bool {
    let mut result = false;
    bindgen::examples_simple_create_and_verify_proof(&mut result);
    result
}
