#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::fmt;
use alloc::vec::Vec;

use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes::Aes256;
use cbc::{Decryptor, Encryptor};

pub fn encrypt(pt: &[u8], dk: &[u8; 32], iv: &[u8; 16]) -> Vec<u8> {
    Encryptor::<Aes256>::new(dk.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(pt)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct DecError;

impl fmt::Display for DecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("decrypt failed")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecError {}

pub fn decrypt(ct: &[u8], dk: &[u8; 32], iv: &[u8; 16]) -> Result<Vec<u8>, DecError> {
    Decryptor::<Aes256>::new(dk.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(ct)
        .map_err(|_| DecError)
}
