//! NAS Security related common functions etc.

/// Key Derivation Function ID (FC param) (Section A.8 33.501)
pub const ALGO_KEY_DERIVE_FC: u8 = 0x69_u8;

/// 5G NAS Encryption Algorithm Algorithm Type N-NAS-enc-alg (Table A.8-1 33.501)
pub const N_NAS_ENC_ALGO: u8 = 0x01_u8;
/// 5G NAS Integrity Algorithm Algorithm Type N-NAS-int-alg (Table A.8-1 33.501)
pub const N_NAS_INT_ALGO: u8 = 0x02_u8;

/// - `NAS_ALGO_IDENTITY_NEA0` - Null Encryption Algorithm (Section D.1 33.501)
pub const NAS_ALGO_IDENTITY_NEA0: u8 = 0x00_u8;
/// - `NAS_ALGO_IDENTITY_NEA1` - Snow3G based Encryption Algorithm (Section D.2.1.2 33.501)
pub const NAS_ALGO_IDENTITY_NEA1: u8 = 0x01_u8;
/// - `NAS_ALGO_IDENTITY_NEA2` - 128 AES - CTR based Encryption Algorithm (Section D.2.1.3 33.501)
pub const NAS_ALGO_IDENTITY_NEA2: u8 = 0x02_u8;
/// - `NAS_ALGO_IDENTITY_NEA3` - ZUC based Encryption Algorithm (Section D.2.1.4 33.501)
pub const NAS_ALGO_IDENTITY_NEA3: u8 = 0x03_u8;

/// - `NAS_ALGO_IDENTITY_NIA0` - Null Identity Algorithm (Section D.1 33.501)
pub const NAS_ALGO_IDENTITY_NIA0: u8 = 0x00_u8;
/// - `NAS_ALGO_IDENTITY_NIA1` - 128 Snow3g based Identity Algorithm (Section D.2.1.3 33.501)
pub const NAS_ALGO_IDENTITY_NIA1: u8 = 0x01_u8;
/// - `NAS_ALGO_IDENTITY_NIA2` - 128 EAS - CMAC based Identity Algorithm (Section D.2.1.2 33.501)
pub const NAS_ALGO_IDENTITY_NIA2: u8 = 0x02_u8;
/// - `NAS_ALGO_IDENTITY_NIA3` - ZUC based Identity Algorithm (Section D.2.1.4 33.501)
pub const NAS_ALGO_IDENTITY_NIA3: u8 = 0x03_u8;

/// Obtain the Key for Encryption or Identity Algorithm for NAS
///
/// Supported Encryption and Ciphering algorithms are one of the following -
///  - `N_NAS_ENC_ALGO`
///  - `N_NAS_INT_ALGO`
///
/// Supported Algorithm Identity depends upon the Algorithm used.
/// For Encryption Algorithm
pub fn nas_get_algorithm_key(input_key: &[u8; 32], algo: u8, algo_identity: u8) -> [u8; 16] {
    let algo = &[algo];
    let algo_param = security_3gpp::KdfParam::from_bytes(algo);

    let algo_identity = &[algo_identity];
    let algo_id_param = security_3gpp::KdfParam::from_bytes(algo_identity);

    let kdf =
        security_3gpp::kdf_common(input_key, ALGO_KEY_DERIVE_FC, &[algo_param, algo_id_param]);

    kdf[16..].try_into().unwrap()
}
