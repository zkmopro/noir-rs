#include "barretenberg/common/wasm_export.hpp"
#include "barretenberg/ecc/curves/bn254/fr.hpp"
#include "barretenberg/ecc/curves/grumpkin/grumpkin.hpp"
#include "multisig.hpp"

extern "C" {

using namespace bb;

WASM_EXPORT void schnorr_construct_signature(uint8_t const* message_buf,
                                             uint8_t const* private_key,
                                             uint8_t* s,
                                             uint8_t* e);

WASM_EXPORT void schnorr_verify_signature(
    uint8_t const* message_buf, uint8_t const* pub_key, uint8_t const* sig_s, uint8_t const* sig_e, bool* result);

WASM_EXPORT void schnorr_multisig_create_multisig_public_key(uint8_t const* private_key, uint8_t* multisig_pubkey_buf);

WASM_EXPORT void schnorr_multisig_validate_and_combine_signer_pubkeys(uint8_t const* signer_pubkey_buf,
                                                                      uint8_t * combined_key_buf,
                                                                      bool* success);

WASM_EXPORT void schnorr_multisig_construct_signature_round_1(uint8_t* round_one_public_output_buf,
                                                              uint8_t* round_one_private_output_buf);

WASM_EXPORT void schnorr_multisig_construct_signature_round_2(uint8_t const* message_buf,
                                                              uint8_t const* private_key,
                                                              uint8_t const* signer_round_one_private_buf,
                                                              uint8_t const* signer_pubkeys_buf,
                                                              uint8_t const* round_one_public_buf,
                                                              uint8_t* round_two_buf,
                                                              bool* success);

WASM_EXPORT void schnorr_multisig_combine_signatures(uint8_t const* message_buf,
                                                     uint8_t const* signer_pubkeys_buf,
                                                     uint8_t const* round_one_buf,
                                                     uint8_t const* round_two_buf,
                                                     uint8_t* s,
                                                     uint8_t* e,
                                                     bool* success);
}
