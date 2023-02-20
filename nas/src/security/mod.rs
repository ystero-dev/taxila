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

/// Encrypt and/or Decrypt a given NAS packet based on Algorithm Identity and given NAS Key and
/// other parameters
pub fn nas_encrypt_payload(
    key: NasKey,
    algo_identity: NasEncryptionAlgoIdentity,
    count: u32,
    bearer: u8,
    downlink: bool,
    payload: &[u8],
) -> Vec<u8> {
    todo!();
}
