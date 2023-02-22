//! Functionality related to implementation of 3GPP Security Key Derivation Functions, Ciphering
//! and Integrity Protection algorithms etc.

mod kdf;

#[doc(inline)]
pub use kdf::{kdf_common, KdfParam, SecurityKey};

mod aes_internal;
pub use aes_internal::{encrypt_aes128_ctr, mac_aes128_cmac, AesIv, AesKey};
