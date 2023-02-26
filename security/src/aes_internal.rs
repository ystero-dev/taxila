//! AES 128 Encryption and Identity functions used.
//!

use cmac::{Cmac, Mac};
use ctr::cipher::{KeyIvInit, StreamCipher};

pub type AesKey = [u8; 16];
pub type AesIv = [u8; 16];
pub type AesMac = [u8; 16];

type Aes128Ctr64BE = ctr::Ctr64BE<aes::Aes128>;

/// Encrypt using AES 128 in CTR Mode (NEA2 33.501 / EEA2 33.401)
pub fn encrypt_aes128_ctr(key: AesKey, iv: AesIv, payload: &[u8]) -> Vec<u8> {
    let mut out = vec![0_u8; payload.len()];
    let mut cipher = Aes128Ctr64BE::new(&key.into(), &iv.into());
    let _ = cipher.apply_keystream_b2b(payload, &mut out);

    out
}

/// Calculate MAC using AES 128 - CMAC
pub fn mac_aes128_cmac(key: AesKey, message: &[u8]) -> AesMac {
    eprintln!("key: {}", hex::encode(key));
    eprintln!("kv: {}", hex::encode(message));
    let mut mac = Cmac::<aes::Aes128>::new_from_slice(&key).unwrap();

    mac.update(message);

    let result = mac.finalize();

    let result = result.into_bytes();
    eprintln!("result : {}", hex::encode(result));
    result.into()
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_empty_payload() {
        struct KeyPayloadTag<'a> {
            key: &'a str,
            payload: &'a str,
            tag: &'a str,
        }

        let testcases = [
            KeyPayloadTag {
                key: "2B7E151628AED2A6ABF7158809CF4F3C",
                payload: "",
                tag: "BB1D6929E95937287FA37D129B756746",
            },
            KeyPayloadTag {
                key: "2B7E151628AED2A6ABF7158809CF4F3C",
                payload: "6BC1BEE22E409F96E93D7E117393172A",
                tag: "070A16B46B4D4144F79BDD9DD04A287C",
            },
            KeyPayloadTag {
                key: "2B7E151628AED2A6ABF7158809CF4F3C",
                payload: "6BC1BEE22E409F96E93D7E117393172AAE2D8A57",
                tag: "7D85449EA6EA19C823A7BF78837DFADE",
            },
            KeyPayloadTag {
                key: "2bd6459f82c5b300952c49104881ff48",
                payload: "38a6f056c0000000333234626339384",
                tag: "118c6eb8b775144b0b83111054c96eb6",
            },
        ];

        for tc in testcases {
            let key = hex::decode(tc.key).unwrap().try_into().unwrap();
            let message = hex::decode(tc.payload).unwrap();
            eprintln!("message: {:?}", message);

            let mac = super::mac_aes128_cmac(key, &message);
            eprintln!("mac: {}", hex::encode(mac));

            assert!(
                tc.tag.to_lowercase() == hex::encode(mac),
                "{}",
                hex::encode(mac)
            );
        }
    }
}
