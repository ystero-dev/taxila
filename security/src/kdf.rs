//! A Common Key Derivation function API
//!
//! For All the Integrity and Cipher Keys used in 3GPP, A Common Key Derivation Function is defined
//! in the Appendix B.2 of 33.222

/// KDF Parameter: Actual Parameter used by a KDF
#[derive(Debug, Clone)]
pub struct KdfParam<'a> {
    param: &'a [u8],
    len: u16,
}

impl<'a> KdfParam<'a> {
    /// Generate a [`KdfParam`] from a string literal (or reference to a `String`).
    pub fn from_str(string: &'a str) -> Self {
        let bytes = string.as_bytes();
        Self {
            param: bytes,
            len: bytes.len() as u16,
        }
    }

    /// Generate a [`KdfParam`] from a slice of `u8`.
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Self {
            param: bytes,
            len: bytes.len() as u16,
        }
    }

    // Internal function used by `kdf_common` to obtain the structure serialized as bytes. First
    // the payload and then length as Big Endian `u16`.
    fn into_bytes(&self) -> Vec<u8> {
        let mut out = self.param.to_vec();
        out.extend(self.len.to_be_bytes());
        out
    }
}

use hmac::{Hmac, Mac};
use sha2::Sha256;

/// Common Key Derivation Function
///
/// An actual implementation of a Key Derivation Function should use this function for deriving the
/// required key for specific use. A set of Key Derivation functions used in LTE and 5G are
/// specified in Appendix A of Specification 33.401
pub fn kdf_common<'a, T>(key: T, fc: u8, params: &'a [KdfParam]) -> [u8; 32]
where
    T: AsRef<[u8]>,
{
    let mut mac =
        Hmac::<Sha256>::new_from_slice(key.as_ref()).expect("HMAC can take key of any size");

    let mut message = vec![fc];
    for param in params {
        message.extend(param.into_bytes());
    }

    mac.update(&message);

    mac.finalize().into_bytes().into()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_simple_example() {
        let key = b"my secret and secure key";

        let param_bytes = super::KdfParam::from_bytes(b"input message");
        let output_bytes = super::kdf_common(key, 0, &[param_bytes]);

        let param_str = super::KdfParam::from_str("input message");
        let output_str = super::kdf_common(key, 0, &[param_str]);

        assert!(output_bytes == output_str);
    }
}
