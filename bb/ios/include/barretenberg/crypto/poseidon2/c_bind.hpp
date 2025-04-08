#pragma once

#include "barretenberg/common/wasm_export.hpp"
#include "barretenberg/ecc/curves/bn254/fr.hpp"

extern "C" {

using namespace bb;

WASM_EXPORT void poseidon_hash(const uint8_t * inputs_buffer, uint8_t * output);
WASM_EXPORT void poseidon_hashes(const uint8_t * inputs_buffer, uint8_t * output);
}
