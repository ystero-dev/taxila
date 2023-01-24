/// 5G Registration Type :  24.501 (Release 17) Section: 9.11.3.7
pub struct FivegRegistrationType {
    iei: Option<u8>,
    follow_on_req_pending: bool,
    reg_type: RegistrationType,
}

/// Registration Type of `FivegRegistrationType` See also: [`FivegRegistrationType`]
#[repr(u8)]
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
pub struct NasKeySetId {
    iei: Option<u8>,
    sec_context: SecurityContextType,
    identifier: u8,
}

/// Security Context Type for `NasKeySetId` See also: [`NasKeySetId`]
#[repr(u8)]
pub enum SecurityContextType {
    Native = 0x00,
    Mapped = 0x01,
}

/// 5GS Mobile Identity
pub struct FivegsMobileIdentity {
    iei: Option<u8>,
    length: u16,
    identity_type: u8,
    identity: MobileIdentityType,
}

pub struct FivegGuti {
    mcc: u16,
    mnc: u16,
    amf_region_id: u8,
    amf_set_id: u16,
    amf_pointer: u8,
    tmsi: u32,
}

#[repr(u8)]
pub enum MobileIdentityType {
    Suci = 1,
    FivegGuti = 2,
    Imei = 3,
    FivegSTmsi = 4,
    ImeiSv = 5,
    MacAddress = 6,
    Eui64 = 7,
}

impl FivegRegistrationType {
    pub fn encode(&self, encode_iei: bool) -> Vec<u8> {
        vec![]
    }

    pub fn decode(data: &[u8], decode_iei: bool) -> std::io::Result<(Self, usize)> {
        let value = data[0];
        let iei = if decode_iei {
            Some((value & 0xF0) >> 4)
        } else {
            None
        };

        let follow_on_req_pending = (value & 0x08) == 0;

        let reg_type = match value & 0x07 {
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
            1,
        ))
    }
}

impl NasKeySetId {
    pub fn encode(&self, encode_iei: bool) -> Vec<u8> {
        vec![]
    }

    pub fn decode(data: &[u8], decode_iei: bool) -> std::io::Result<(Self, usize)> {
        let value = data[0];
        let iei = if decode_iei {
            Some((value & 0xF0) >> 4)
        } else {
            None
        };

        let sec_context = match value & 0x80 {
            0 => SecurityContextType::Native,
            0x80 => SecurityContextType::Mapped,
            _ => unreachable!(),
        };

        let identifier = value & 0x07;

        Ok((
            Self {
                iei,
                sec_context,
                identifier,
            },
            1,
        ))
    }
}

impl FivegsMobileIdentity {
    pub fn encode(&self, encode_iei: bool) -> Vec<u8> {
        vec![]
    }

    pub fn decode(data: &[u8], decode_iei: bool) -> std::io::Result<(Self, usize)> {
        let mut decoded = 0;
        let iei = if decode_iei {
            decoded += 1;
            Some(data[0] & 0xF0)
        } else {
            None
        };

        let length = u16::from_le_bytes(data[decoded..decoded + 2].try_into().unwrap());
        decoded += 2;

        let identity_type = data[decoded] & 0x07;
        decoded += 1;

        let identity = match identity_type {
            1 => MobileIdentityType::Suci,
            2 => MobileIdentityType::FivegGuti,
            3 => MobileIdentityType::Imei,
            4 => MobileIdentityType::FivegSTmsi,
            5 => MobileIdentityType::ImeiSv,
            6 => MobileIdentityType::MacAddress,
            7 => MobileIdentityType::Eui64,
            _ => MobileIdentityType::Suci,
        };

        // TODO: Complete implementation for this.
        let _ = match identity_type {
            FivegGuti => FivegGuti::decode(&data[decoded..])?,
            _ => todo!(),
        };

        Ok((
            Self {
                iei,
                length,
                identity_type,
                identity,
            },
            decoded,
        ))
    }
}

impl FivegGuti {
    pub fn decode(data: &[u8]) -> std::io::Result<(Self, usize)> {
        todo!()
    }
}
