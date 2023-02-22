//! NAS Security related common functions etc.

/// Key Derivation Function ID (FC param) (Section A.8 33.501)
const ALGO_KEY_DERIVE_FC: u8 = 0x69_u8;

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NasAlgorithmType {
    /// 5G NAS Encryption Algorithm - Algorithm Type N-NAS-enc-alg (Table A.8-1 33.501)
    Encryption = 0x01,

    /// 5G NAS Integrity Algorithm Algorithm Type N-NAS-int-alg (Table A.8-1 33.501)
    Integrity = 0x02_u8,
}

/// NAS Encryption Algorithm Identity
#[repr(u8)]
pub enum NasEncryptionAlgoIdentity {
    /// - `NAS_ALGO_IDENTITY_NEA0` - Null Encryption Algorithm (Section D.1 33.501)
    Nea0 = 0x00,
    /// - `NAS_ALGO_IDENTITY_NEA1` - Snow3G based Encryption Algorithm (Section D.2.1.2 33.501)
    Nea1 = 0x01,
    /// - `NAS_ALGO_IDENTITY_NEA2` - 128 AES - CTR based Encryption Algorithm (Section D.2.1.3 33.501)
    Nea2 = 0x02,
    /// - `NAS_ALGO_IDENTITY_NEA3` - ZUC based Encryption Algorithm (Section D.2.1.4 33.501)
    Nea3 = 0x03,
}

/// NAS Identity Algorithm Identity
pub enum NasIntegrityAlgoIdentity {
    /// - `NAS_ALGO_IDENTITY_NIA0` - Null Integrity Algorithm (Section D.1 33.501)
    Nia0 = 0x00,
    /// - `NAS_ALGO_IDENTITY_NIA1` - 128 Snow3g based Integrity Algorithm (Section D.2.1.3 33.501)
    Nia1 = 0x01,
    /// - `NAS_ALGO_IDENTITY_NIA2` - 128 EAS - CMAC based Integrity Algorithm (Section D.2.1.2 33.501)
    Nia2 = 0x02,
    /// - `NAS_ALGO_IDENTITY_NIA3` - ZUC based Integrity Algorithm (Section D.2.1.4 33.501)
    Nia3 = 0x03,
}

/// NAS Key Type: The 128 bit key used by Encryption or Identity Algorithms
pub type NasKey = [u8; 16];

/// NAS MAC Type: The 32 bit array resulting from MAC calculation.
pub type NasMac = [u8; 4];

/// Obtain the Key for Encryption Algorithm for NAS
pub fn nas_encryption_algorithm_key(
    input_key: security_3gpp::SecurityKey,
    identity: NasEncryptionAlgoIdentity,
) -> NasKey {
    let algo = &[NasAlgorithmType::Encryption as u8];
    let algo_param = security_3gpp::KdfParam::from_bytes(algo);

    let algo_identity = &[identity as u8];
    let algo_id_param = security_3gpp::KdfParam::from_bytes(algo_identity);

    let kdf =
        security_3gpp::kdf_common(input_key, ALGO_KEY_DERIVE_FC, &[algo_param, algo_id_param]);

    kdf[16..].try_into().unwrap()
}

/// Obtain the Key for Integrity Algorithm for NAS
pub fn nas_integrity_algorithm_key(
    input_key: security_3gpp::SecurityKey,
    identity: NasIntegrityAlgoIdentity,
) -> NasKey {
    let algo = &[NasAlgorithmType::Integrity as u8];
    let algo_param = security_3gpp::KdfParam::from_bytes(algo);

    let algo_identity = &[identity as u8];
    let algo_id_param = security_3gpp::KdfParam::from_bytes(algo_identity);

    let kdf =
        security_3gpp::kdf_common(input_key, ALGO_KEY_DERIVE_FC, &[algo_param, algo_id_param]);

    kdf[16..].try_into().unwrap()
}

/// Encrypt and/or Decrypt a given NAS payload based on Algorithm Identity and given NAS Key and
/// other parameters
pub fn nas_encrypt_payload(
    key: NasKey,
    algo_identity: NasEncryptionAlgoIdentity,
    count: u32,
    bearer: u8,
    downlink: bool,
    payload: &[u8],
) -> Vec<u8> {
    let mut iv = vec![0_u8; 16];
    let count_bytes = count.to_be_bytes();
    iv.splice(0..4, count_bytes);
    iv[4] = bearer << 3;
    if downlink {
        iv[4] |= 0x04;
    }
    let iv = iv.try_into().unwrap();

    match algo_identity {
        NasEncryptionAlgoIdentity::Nea0 => payload.to_vec(),
        NasEncryptionAlgoIdentity::Nea2 => security_3gpp::encrypt_aes128_ctr(key, iv, payload),
        _ => todo!(),
    }
}

/// Calculate Message Authenticity Code for NAS using a given NAS Key for Integrity Protection
pub fn nas_calculate_mac(
    key: NasKey,
    algo_identity: NasIntegrityAlgoIdentity,
    count: u32,
    bearer: u8,
    downlink: bool,
    payload: &[u8],
) -> NasMac {
    let mut kv = vec![0_u8; 8];
    let count_bytes = count.to_be_bytes();
    kv.splice(0..4, count_bytes);
    kv[4] = bearer << 3;
    if downlink {
        kv[4] |= 0x04;
    }
    kv.extend(payload);

    match algo_identity {
        NasIntegrityAlgoIdentity::Nia0 => [0_u8; 4],
        NasIntegrityAlgoIdentity::Nia2 => security_3gpp::mac_aes128_cmac(key, &kv)[..4]
            .try_into()
            .unwrap(),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_set_1_33_401_c1() {
        let key = hex::decode("d3c5d592327fb11c4035c6680af8c6d1").unwrap();
        let count = 0x398a59b4_u32;
        let bearer = 0x15_u8;
        let downlink = true;
        let payload =
            hex::decode("981ba6824c1bfb1ab485472029b71d808ce33e2cc3c0b5fc1f3de8a6dc66b1f0")
                .unwrap();

        let result = super::nas_encrypt_payload(
            key.try_into().unwrap(),
            super::NasEncryptionAlgoIdentity::Nea2,
            count,
            bearer,
            downlink,
            &payload,
        );

        assert!(
            "e9fed8a63d155304d71df20bf3e82214b20ed7dad2f233dc3c22d7bdeeed8e78"
                == hex::encode(result)
        );
    }

    #[test]
    fn test_set_1_33_401_c2() {
        let key = hex::decode("2bd6459f82c5b300952c49104881ff48").unwrap();
        let count = 0x38a6f056_u32;
        let bearer = 0x18_u8;
        let downlink = false;
        let payload = hex::decode("3332346263393840").unwrap();

        let result = super::nas_calculate_mac(
            key.try_into().unwrap(),
            super::NasIntegrityAlgoIdentity::Nia2,
            count,
            bearer,
            downlink,
            &payload,
        );

        eprintln!("result: {}", hex::encode(result));
        assert!(
            "e9fed8a63d155304d71df20bf3e82214b20ed7dad2f233dc3c22d7bdeeed8e78"
                == hex::encode(result)
        );
    }
}
