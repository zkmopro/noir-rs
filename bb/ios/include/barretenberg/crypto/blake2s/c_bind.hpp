#include "barretenberg/common/wasm_export.hpp"
#include "barretenberg/ecc/curves/bn254/fr.hpp"
#include <cstddef>
#include <cstdint>

extern "C" {

using namespace bb;

WASM_EXPORT void blake2s(uint8_t const* data, uint8_t * out);

WASM_EXPORT void blake2s_to_field_(uint8_t const* data, uint8_t * r);
}
