use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes::Aes256;
use cbc::{Decryptor, Encryptor};

pub fn encrypt(pt: &[u8], dk: &[u8; 32], iv: &[u8; 16]) -> Vec<u8> {
    Encryptor::<Aes256>::new(dk.into(), iv.into()).encrypt_padded_vec_mut::<Pkcs7>(pt)
}

pub fn decrypt(ct: &[u8], dk: &[u8; 32], iv: &[u8; 16]) -> Result<Vec<u8>, ()> {
    Decryptor::<Aes256>::new(dk.into(), iv.into())
        .decrypt_padded_vec_mut::<Pkcs7>(ct)
        .map_err(|_| ())
}
