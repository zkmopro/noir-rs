pub mod acir;
pub mod blake2s;
pub mod common;
pub mod models;
pub mod pedersen;
pub mod poseidon;
pub mod schnorr;
pub mod srs;
pub mod traits;

#[cfg(test)]
pub mod tests;

mod bindgen {
    // This matches bindgen::Builder output
    include!("bindings.rs");
}

use self::traits::SerializeBuffer;
use std::{ffi::CStr, slice};

#[derive(Debug, thiserror::Error)]
pub enum BufferError {
    #[error("Binding call error")]
    Null,
}

pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    /// Constructs a Buffer from a raw pointer, reading a u32 length followed by that many bytes.
    ///
    /// # Safety
    /// This method is unsafe because it trusts the caller to ensure that `ptr` is a valid pointer
    /// pointing to at least `u32` bytes plus the length indicated by the u32 value.
    pub unsafe fn from_ptr(ptr: *const u8) -> Result<Self, BufferError> {
        if ptr.is_null() {
            return Err(BufferError::Null);
        }
        let len_slice = slice::from_raw_parts(ptr, 4);
        let len = u32::from_be_bytes([len_slice[0], len_slice[1], len_slice[2], len_slice[3]]);
        let data_ptr = ptr.add(4);
        let data = slice::from_raw_parts(data_ptr, len as usize);
        Ok(Self {
            data: data.to_vec(),
        })
    }

    /// Returns a reference to the buffer's data as a slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Consumes the Buffer, returning its underlying data as a Vec<u8>.
    pub fn to_vec(self) -> Vec<u8> {
        self.data
    }
}

/// Parses a C string from a raw pointer and returns a Rust String.
///
/// # Safety
/// This function is unsafe because it trusts the caller to provide a valid null-terminated
/// C string. Dereferencing an invalid pointer can cause undefined behavior.
pub unsafe fn parse_c_str(ptr: *const ::std::os::raw::c_char) -> Option<String> {
    if ptr.is_null() {
        return None;
    }
    CStr::from_ptr(ptr)
        .to_str()
        .map_or(None, |s| Some(s.to_string()))
}

impl<T: SerializeBuffer> SerializeBuffer for &[T] {
    fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&(self.len() as u32).to_be_bytes());
        for elem in self.iter() {
            buffer.extend_from_slice(&elem.to_buffer());
        }
        buffer
    }
}

impl SerializeBuffer for u8 {
    fn to_buffer(&self) -> Vec<u8> {
        vec![*self]
    }
}
