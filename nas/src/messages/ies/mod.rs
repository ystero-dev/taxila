/// 5G Registration Type :  24.501 (Release 17) Section: 9.11.3.7
#[derive(Debug, Eq, PartialEq)]
pub struct FivegRegistrationType {
    iei: Option<u8>,
    follow_on_req_pending: bool,
    reg_type: RegistrationType,
}

/// Registration Type of `FivegRegistrationType` See also: [`FivegRegistrationType`]
#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum RegistrationType {
    Initial = 0x01,

    MobilityUpdating = 0x02,

    PeriodicUpdating = 0x03,

    Emergency = 0x04,

    SnpnOnboarding = 0x05,

    DisasterRoamingUpdating = 0x06,

    DisasterRoamingInitial = 0x07,
}

/// NAS Keyset Encoding. 24.501 (Release 17) Section: 9.11.3.32
#[derive(Debug, Eq, PartialEq)]
pub struct NasKeySetId {
    iei: Option<u8>,
    sec_context: SecurityContextType,
    identifier: u8,
}

/// Security Context Type for `NasKeySetId` See also: [`NasKeySetId`]
#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum SecurityContextType {
    Native = 0x00,
    Mapped = 0x01,
}

/// 5GS Mobile Identity: 24.501 (Release 17) Section 9.11.3.4
#[derive(Debug, Eq, PartialEq)]
pub struct FivegsMobileIdentity {
    iei: Option<u8>,
    length: u16,
    identity: MobileIdentity,
}

/// 5G-GUTI Mobile Identity: 24.501 (Release 17) Figure 9.11.3.4.1
#[derive(Debug, Eq, PartialEq)]
pub struct FivegGuti {
    mcc: u16,
    mnc: u16,
    amf_region_id: u8,
    amf_set_id: u16,
    amf_pointer: u8,
    tmsi: u32,
}

/// SUCI Mobile Identity: 24.501 (Release 17) Figure 9.11.3.4.3-3A/9.11.3.4.4
#[derive(Debug, Eq, PartialEq)]
pub struct Suci {
    supi_format: u8, // TODO: Make Enum
    mcc: u16,
    mnc: u16,
    routing_indicator: u16,
    protection_scheme: u8, // TODO: Make Enum
    home_network_pki: u8,
    scheme_output: Vec<u8>,
}

/// Enum representing Mobile Identity: See also [`FivegsmobileIdentity`]
#[repr(u8)]
#[derive(Debug, Eq, PartialEq)]
pub enum MobileIdentity {
    Suci(Suci),
    FivegGuti(FivegGuti),
    // TODO: Other variant support
}

/// UE Security Capability : 24.501 (Release 17) Section 9.11.3.54
#[derive(Debug, Eq, PartialEq)]
pub struct UeSecurityCapability {
    capabilities: Vec<u8>,
}

impl FivegRegistrationType {
    pub(crate) fn encode(&self, encode_iei: bool) -> Vec<u8> {
        vec![]
    }

    pub(crate) fn decode(
        data: &[u8],
        decode_iei: bool,
        upper: bool,
    ) -> std::io::Result<(Self, usize)> {
        let value = data[0];

        let iei = if decode_iei {
            Some((value & 0xF0) >> 4)
        } else {
            None
        };

        let for_reg_type = if upper { value >> 4 } else { value & 0x0F };

        let follow_on_req_pending = (for_reg_type & 0x08) == 0;

        let reg_type = match for_reg_type & 0x07 {
            1 => RegistrationType::Initial,
            2 => RegistrationType::MobilityUpdating,
            3 => RegistrationType::PeriodicUpdating,
            4 => RegistrationType::Emergency,
            5 => RegistrationType::SnpnOnboarding,
            6 => RegistrationType::DisasterRoamingUpdating,
            7 => RegistrationType::DisasterRoamingInitial,
            _ => RegistrationType::Initial,
        };

        Ok((
            Self {
                iei,
                follow_on_req_pending,
                reg_type,
            },
            0,
        ))
    }
}

impl NasKeySetId {
    pub(crate) fn encode(&self, encode_iei: bool) -> Vec<u8> {
        vec![]
    }

    pub(crate) fn decode(
        data: &[u8],
        decode_iei: bool,
        upper: bool,
    ) -> std::io::Result<(Self, usize)> {
        log::trace!("NasKeySetId decode");

        let value = data[0];
        let iei = if decode_iei {
            Some((value & 0xF0) >> 4)
        } else {
            None
        };

        let sec_context_and_id = if upper { value >> 4 } else { value & 0x0F };
        let sec_context = match sec_context_and_id & 0x08 {
            0 => SecurityContextType::Native,
            0x08 => SecurityContextType::Mapped,
            _ => unreachable!(),
        };

        let identifier = sec_context_and_id & 0x07;

        Ok((
            Self {
                iei,
                sec_context,
                identifier,
            },
            0,
        ))
    }
}

impl FivegsMobileIdentity {
    pub(crate) fn encode(&self, encode_iei: bool) -> Vec<u8> {
        vec![]
    }

    pub(crate) fn decode(data: &[u8], decode_iei: bool) -> std::io::Result<(Self, usize)> {
        log::trace!("FivegsMobileIdentity decode");

        let mut decoded = 0;
        let iei = if decode_iei {
            decoded += 1;
            Some(data[0] & 0xF0)
        } else {
            None
        };

        let length = u16::from_be_bytes(data[decoded..decoded + 2].try_into().unwrap());
        decoded += 2;

        let (identity, identity_decoded) = MobileIdentity::decode(&data[decoded..], length)?;
        decoded += identity_decoded;

        Ok((
            Self {
                iei,
                length,
                identity,
            },
            decoded,
        ))
    }
}

impl FivegGuti {
    pub(crate) fn decode(data: &[u8]) -> std::io::Result<(Self, usize)> {
        log::trace!("FivegGuti decode");

        let mut decoded = 0;

        let (mcc, mnc, mcc_mnc_decoded) = decode_mcc_mnc(&data[decoded..])?;
        decoded += mcc_mnc_decoded;

        let amf_region_id = data[decoded];
        decoded += 1;

        let amf_set_id = data[decoded] as u16;
        decoded += 1;

        let amf_set_id_remaining = (data[decoded] & 0xC0) >> 6;
        let amf_set_id = (amf_set_id << 2) | amf_set_id_remaining as u16;

        let amf_pointer = data[decoded] & 0x3F;
        decoded += 1;

        let tmsi = u32::from_be_bytes(data[decoded..decoded + 4].try_into().unwrap());
        decoded += 4;

        Ok((
            Self {
                mcc,
                mnc: mnc.into(),
                amf_region_id,
                amf_set_id,
                amf_pointer,
                tmsi,
            },
            decoded,
        ))
    }
}

impl MobileIdentity {
    pub(crate) fn encode(&self) -> Vec<u8> {
        todo!();
    }

    pub(crate) fn decode(data: &[u8], length: u16) -> std::io::Result<(Self, usize)> {
        log::trace!("MobileIdentity decode");

        let identity_type_byte = data[0];
        match identity_type_byte & 0x07 {
            1 => {
                let (suci, decoded) = Suci::decode(data, length)?;
                Ok((Self::Suci(suci), decoded))
            }
            _ => {
                log::error!(
                    "identity type decode not supported yet: {:?}",
                    identity_type_byte
                );
                todo!();
            }
        }
    }
}

impl Suci {
    pub(crate) fn encode(&self) -> Vec<u8> {
        todo!();
    }

    pub(crate) fn decode(data: &[u8], length: u16) -> std::io::Result<(Self, usize)> {
        log::trace!("Suci decode");

        let mut decoded = 0;

        let supi_format = (data[decoded] & 0xF0) >> 4;
        decoded += 1;

        let (mcc, mnc, mcc_mnc_decoded) = decode_mcc_mnc(&data[decoded..])?;
        decoded += mcc_mnc_decoded;

        let (ri2, ri1) = (((data[decoded] & 0xF0) >> 4), data[decoded] & 0x0F);
        decoded += 1;
        let (ri4, ri3) = (((data[decoded] & 0xF0) >> 4), data[decoded] & 0x0F);
        decoded += 1;

        let mut routing_indicator: u16 = ri1 as u16;
        if ri2 != 0x0F {
            routing_indicator = ri2 as u16 * 10
        };
        if ri3 != 0x0F {
            routing_indicator = ri3 as u16 * 100
        };
        if ri4 != 0x0F {
            routing_indicator = ri4 as u16 * 1000
        };

        let protection_scheme = data[decoded] & 0x0F;
        decoded += 1;

        let home_network_pki = data[decoded];
        decoded += 1;

        let scheme_output = data[decoded..length as usize].try_into().unwrap();
        decoded += length as usize - decoded;
        Ok((
            Self {
                supi_format,
                mcc,
                mnc,
                routing_indicator,
                protection_scheme,
                home_network_pki,
                scheme_output,
            },
            decoded,
        ))
    }
}

impl UeSecurityCapability {
    pub(crate) fn encode(&self) -> Vec<u8> {
        todo!();
    }

    pub(crate) fn decode(data: &[u8]) -> std::io::Result<(Self, usize)> {
        log::trace!("UeSecurityCapability decode");

        let mut decoded = 0;
        let iei = data[decoded];
        decoded += 1;

        let length = data[decoded];
        decoded += 1;

        let capabilities = data[decoded..decoded + length as usize].try_into().unwrap();
        decoded += length as usize;

        Ok((Self { capabilities }, decoded))
    }
}

fn decode_mcc_mnc(data: &[u8]) -> std::io::Result<(u16, u16, usize)> {
    log::trace!("decode mcc-mnc");

    let mut decoded = 0;

    let mcc1 = data[decoded] & 0x0f;
    let mcc2 = (data[decoded] & 0xf0) >> 4;
    decoded += 1;

    let mcc3 = data[decoded] & 0x0f;
    let mnc3 = (data[decoded] & 0xf0) >> 4;
    decoded += 1;

    let mnc1 = data[decoded] & 0x0f;
    let mnc2 = (data[decoded] & 0xf0) >> 4;

    let mcc = mcc3 as u16 + 10 * mcc2 as u16 + 100 * mcc1 as u16;
    let mnc = if mnc3 == 0x0f {
        mnc2 as u16 + mnc1 as u16 * 10
    } else {
        mnc2 as u16 + mnc1 as u16 * 10 + mnc3 as u16 * 100
    };
    decoded += 1;

    Ok((mcc, mnc, decoded))
}
