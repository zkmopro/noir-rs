use super::{
    bindgen,
    models::{Fq, Fr, Point},
    traits::{DeserializeBuffer, SerializeBuffer},
};

pub unsafe fn schnorr_construct_signature(
    message: &[u8],
    private_key: &Fr,
) -> ([u8; 32], [u8; 32]) {
    let mut s = [0; 32];
    let mut e = [0; 32];
    bindgen::schnorr_construct_signature(
        message.to_buffer().as_slice().as_ptr(),
        private_key.to_buffer().as_slice().as_ptr(),
        s.as_mut_ptr(),
        e.as_mut_ptr(),
    );
    (s, e)
}

pub unsafe fn schnorr_verify_signature(
    message: &[u8],
    public_key: &Point,
    sig_s: &mut [u8; 32],
    sig_e: &mut [u8; 32],
) -> bool {
    let mut result = false;
    bindgen::schnorr_verify_signature(
        message.to_buffer().as_slice().as_ptr(),
        public_key.to_buffer().as_slice().as_ptr(),
        sig_s.as_mut_ptr(),
        sig_e.as_mut_ptr(),
        &mut result,
    );
    result
}

pub unsafe fn schnorr_multisig_create_multisig_public_key(public_key: &Fq) -> [u8; 128] {
    let mut result = [0; 128];
    bindgen::schnorr_multisig_create_multisig_public_key(
        public_key.to_buffer().as_slice().as_ptr(),
        result.as_mut_ptr(),
    );
    result
}
