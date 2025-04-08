#pragma once

#include "barretenberg/common/wasm_export.hpp"
#include "barretenberg/ecc/curves/bn254/fr.hpp"

extern "C" {

using namespace bb;

WASM_EXPORT void pedersen_hash(const uint8_t *inputs_buffer, const uint32_t *hash_index, uint8_t *output);
WASM_EXPORT void pedersen_hashes(const uint8_t *inputs_buffer, const uint32_t *hash_index, uint8_t *output);
WASM_EXPORT void pedersen_hash_buffer(const uint8_t *input_buffer, const uint32_t *hash_index, uint8_t *output);
}
