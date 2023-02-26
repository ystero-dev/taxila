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

/// Encrypt a given NAS payload based on Algorithm Identity and given NAS Key and
/// other parameters
pub fn nas_encrypt_payload(
    key: NasKey,
    algo_identity: NasEncryptionAlgoIdentity,
    count: u32,
    bearer: u8,
    downlink: bool,
    payload: &[u8],
    bitlen: u32,
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
        NasEncryptionAlgoIdentity::Nea2 => {
            let mut output = security_3gpp::encrypt_aes128_ctr(key, iv, payload);
            let remaining = 32 - bitlen % 32;
            if remaining > 0 {
                let output_len = output.len();
                // TODO: Case for when bitlen is < 4 bytes
                let last = output.split_off(output_len - 4);
                let mut last_u32 = u32::from_be_bytes(last.try_into().unwrap());
                let last_u32 = last_u32 & (0xFFFFFFFF << remaining);
                let last = last_u32.to_be_bytes();
                output.extend(last)
            }
            output
        }
        _ => todo!(),
    }
}

/// Decrypt encrypted NAS payload.
///
/// See also [`nas_encrypt_payload`]
pub fn nas_decrypt_payload(
    key: NasKey,
    algo_identity: NasEncryptionAlgoIdentity,
    count: u32,
    bearer: u8,
    downlink: bool,
    payload: &[u8],
    bitlen: u32,
) -> Vec<u8> {
    nas_encrypt_payload(key, algo_identity, count, bearer, downlink, payload, bitlen)
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
        struct TestSet<'ts> {
            name: &'ts str,
            key: [u8; 16],
            count: u32,
            bearer: u8,
            downlink: bool,
            payload: &'ts str,
            bitlen: u32,
            ciphertext: &'ts str,
        }

        let testsets = vec![
            // TestSet 1 33.401 C.1.1
            TestSet {
                name: "TestSet 1",
                key: hex::decode("d3c5d592327fb11c4035c6680af8c6d1")
                    .unwrap()
                    .try_into()
                    .unwrap(),
                count: 0x398a59b4_u32,
                bearer: 0x15_u8,
                downlink: true,
                payload: "981ba6824c1bfb1ab485472029b71d808ce33e2cc3c0b5fc1f3de8a6dc66b1f0",
                bitlen: 253,
                ciphertext: "e9fed8a63d155304d71df20bf3e82214b20ed7dad2f233dc3c22d7bdeeed8e78",
            },
            // TestSet 2 33.401 C.1.2
            TestSet {
                name: "TestSet 2",
                key: hex::decode("2bd6459f82c440e0952c49104805ff48")
                    .unwrap()
                    .try_into()
                    .unwrap(),
                count: 0xc675a64b_u32,
                bearer: 0x0c_u8,
                downlink: true,
                payload: "7ec61272743bf1614726446a6c38ced166f6ca76eb5430044286346cef130f92922b03450d3a9975e5bd2ea0eb55ad8e1b199e3ec4316020e9a1b285e762795359b7bdfd39bef4b2484583d5afe082aee638bf5fd5a606193901a08f4ab41aab9b134880",
                bitlen: 798,
                ciphertext: "5961605353c64bdca15b195e288553a910632506d6200aa790c4c806c99904cf2445cc50bb1cf168a49673734e081b57e324ce5259c0e78d4cd97b870976503c0943f2cb5ae8f052c7b7d392239587b8956086bcab18836042e2e6ce42432a17105c53d0",
            },
            // TestSet 3 33.401 C.1.3
            TestSet {
                name: "TestSet 3",
                key: hex::decode("0a8b6bd8d9b08b08d64e32d1817777fb")
                    .unwrap()
                    .try_into()
                    .unwrap(),
                count: 0x544d49cd_u32,
                bearer: 0x04_u8,
                downlink: false,
                payload: "fd40a41d370a1f65745095687d47ba1d36d2349e23f644392c8ea9c49d40c13271aff264d0f24800",
                bitlen: 310,
                ciphertext: "75750d37b4bba2a4dedb34235bd68c6645acdaaca48138a3b0c471e2a7041a576423d2927287f000",
            },
            // TestSet 4 33.401 C.1.4
            TestSet {
                name: "TestSet 5",
                key: hex::decode("aa1f95aea533bcb32eb63bf52d8f831a")
                    .unwrap()
                    .try_into()
                    .unwrap(),
                count: 0x72d8c671_u32,
                bearer: 0x10_u8,
                downlink: true,
                payload: "fb1b96c5c8badfb2e8e8edfde78e57f2ad81e74103fc430a534dcc37afcec70e1517bb06f27219dae49022ddc47a068de4c9496a951a6b09edbdc864c7adbd740ac50c022f3082bafd22d78197c5d508b977bca13f32e652e74ba728576077ce628c535e87dc6077ba07d29068590c8cb5f1088e082cfa0ec961302d69cf3d44",
                bitlen: 1022,
                ciphertext: "dfb440acb3773549efc04628aeb8d8156275230bdc690d94b00d8d95f28c4b56307f60f4ca55eba661ebba72ac808fa8c49e26788ed04a5d606cb418de74878b9a22f8ef29590bc4eb57c9faf7c41524a885b8979c423f2f8f8e0592a9879201be7ff9777a162ab810feb324ba74c4c156e04d39097209653ac33e5a5f2d8864",
            },
            // TestSet 5 33.401 C.1.5
            TestSet {
                name: "TestSet 5",
                key: hex::decode("9618ae46891f86578eebe90ef7a1202e")
                    .unwrap()
                    .try_into()
                    .unwrap(),
                count: 0xc675a64b_u32,
                bearer: 0x0c_u8,
                downlink: true,
                payload: "8daa17b1ae050529c6827f28c0ef6a1242e93f8b314fb18a77f790ae049fedd612267fecaefc450174d76d9f9aa7755a30cd90a9a5874bf48eaf70eea3a62a250a8b6bd8d9b08b08d64e32d1817777fb544d49cd49720e219dbf8bbed33904e1fd40a41d370a1f65745095687d47ba1d36d2349e23f644392c8ea9c49d40c13271aff264d0f24841d6465f0996ff84e65fc517c53efc3363c38492a8",
                bitlen: 1245,
                ciphertext: "919c8c33d66789703d05a0d7ce82a2aeac4ee76c0f4da050335e8a84e7897ba5df2f36bd513e3d0c8578c7a0fcf043e03aa3a39fbaad7d15be074faa5d9029f71fb457b647834714b0e18f117fca10677945096c8c5f326ba8d6095eb29c3e36cf245d1622aafe921f7566c4f5d644f2f1fc0ec684ddb21349747622e209295d27ff3f95623371d49b147c0af486171f22cd04b1cbeb2658223e6938",
            },
            // TestSet 6 33.401 C.1.6
            TestSet {
                name: "TestSet 1",
                key: hex::decode("54f4e2e04c83786eec8fb5abe8e36566")
                    .unwrap()
                    .try_into()
                    .unwrap(),
                count: 0xaca4f50f_u32,
                bearer: 0x0b_u8,
                downlink: false,
                payload: "40981ba6824c1bfb4286b299783daf442c099f7ab0f58d5c8e46b104f08f01b41ab485472029b71d36bd1a3d90dc3a41b46d51672ac4c9663a2be063da4bc8d2808ce33e2cccbfc634e1b259060876a0fbb5a437ebcc8d31c19e4454318745e3fa16bb11adae248879fe52db2543e53cf445d3d828ce0bf5c560593d97278a59762dd0c2c9cd68d4496a792508614014b13b6aa51128c18cd6a90b87978c2ff1cabe7d9f898a411bfdb84f68f6727b1499cdd30df0443ab4a66653330bcba1105e4cec034c73e605b4310eaaadcfd5b0ca27ffd89d144df4792759427c9cc1f8cd8c87202364b8a687954cb05a8d4e2d99e73db160deb180ad0841e96741a5d59fe4189f15420026fe4cd12104932fb38f735340438aaf7eca6fd5cfd3a195ce5abe65272af607ada1be65a6b4c9c0693234092c4d018f1756c6db9dc8a6d80b888138616b681262f954d0e7711748780d92291d86299972db741cfa4f37b8b56cdb18a7ca8218e86e4b4b716a4d04371fbec262fc5ad0b3819b187b97e55b1a4d7c19ee24c8b4d7723cfedf045b8acae4869517d80e50615d9035d5d9c5a40af602280b542597b0cb18619eeb35925759d195e100e8e4aa0c38a3c2abe0f3d8ff04f3c33c295069c23694b5bbeacdd542e28e8a94edb9119f412d054be1fa7200b09000",
                bitlen: 3861,
                ciphertext: "5cb72c6edc878f1566e10253afc364c9fa540d914db94cbee275d0917ca6af0d77acb4ef3bbe1a722b2ef5bd1d4b8e2aa5024ec1388a201e7bce7920aec615895f763a5564dcc4c482a2ee1d8bfecc4498eca83fbb75f9ab530e0dafbede2fa5895b82991b6277c529e0f2529d7f79606be96706296dedfa9d7412b616958cb563c678c02825c30d0aee77c4c146d2765412421a808d13cec819694c75ad572e9b973d948b81a9337c3b2a17192e22c2069f7ed1162af44cdea817603665e807ce40c8e0dd9d6394dc6e31153fe1955c47afb51f2617ee0c5e3b8ef1ad7574ed343edc2743cc94c990e1f1fd264253c178dea739c0befeebcd9f9b76d49c1015c9fecf50e53b8b5204dbcd3eed863855dabcdcc94b31e318021568855c8b9e52a981957a112827f978ba960f1447911b317b5511fbcc7fb13ac153db74251117e4861eb9e83bffffc4eb7755579038e57924b1f78b3e1ad90bab2a07871b72db5eef96c334044966db0c37cafd1a89e5646a3580eb6465f121dce9cb88d85b96cf23ccccd4280767bee8eeb23d8652461db6493103003baf89f5e18261ea43c84a92ebffffe4909dc46c5192f825f770600b9602c557b5f8b431a79d45977dd9c41b863da9e142e90020cfd074d6927b7ab3b6725d1a6f3f98b9c9daa8982aff06782800",
            },
        ];

        for ts in testsets {
            let result = super::nas_encrypt_payload(
                ts.key,
                super::NasEncryptionAlgoIdentity::Nea2,
                ts.count,
                ts.bearer,
                ts.downlink,
                hex::decode(ts.payload).unwrap().as_ref(),
                ts.bitlen,
            );
            assert!(
                ts.ciphertext == hex::encode(&result),
                "Failure:{}, Expected: {}, Computed:{}",
                ts.name,
                ts.ciphertext,
                hex::encode(&result)
            );
            // decrypt it and we should get back the payload
            let result = super::nas_decrypt_payload(
                ts.key.try_into().unwrap(),
                super::NasEncryptionAlgoIdentity::Nea2,
                ts.count,
                ts.bearer,
                ts.downlink,
                &result,
                ts.bitlen,
            );
            assert!(
                result == hex::decode(ts.payload).unwrap(),
                "Failure:{}, Expected: {}, Computed:{}",
                ts.name,
                ts.payload,
                hex::encode(&result)
            );
        }
    }

    #[ignore]
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

        assert!(
            "e9fed8a63d155304d71df20bf3e82214b20ed7dad2f233dc3c22d7bdeeed8e78"
                == hex::encode(result)
        );
    }
}
