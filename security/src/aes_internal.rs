//! AES 128 Encryption and Identity functions used.
//!

use ctr::cipher::{KeyIvInit, StreamCipher};

pub type AesKey = [u8; 16];
pub type AesIv = [u8; 16];

type Aes128Ctr64BE = ctr::Ctr64BE<aes::Aes128>;

/// Encrypt using AES 128 in CTR Mode (NEA2 33.501 / EEA2 33.401)
pub fn encrypt_aes128_ctr(key: AesKey, iv: AesIv, payload: &[u8]) -> Vec<u8> {
    let mut out = vec![0_u8; payload.len()];
    let mut cipher = Aes128Ctr64BE::new(&key.into(), &iv.into());
    let _ = cipher.apply_keystream_b2b(payload, &mut out);

    out
}
