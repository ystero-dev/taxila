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

pub enum MobileIdentityType {
    Suci,
    FivegGuti,
    Imei,
    FivegSTmsi,
    ImeiSv,
    MacAddress,
    Eui64,
}

impl FivegRegistrationType {
    pub fn encode(&self, encode_iei: bool) -> Vec<u8> {
        vec![]
    }

    pub fn decode(data: &[u8], decode_iei: bool) -> std::io::Result<Self> {
        todo!();
    }
}

impl NasKeySetId {
    pub fn encode(&self, encode_iei: bool) -> Vec<u8> {
        vec![]
    }

    pub fn decode(data: &[u8], decode_iei: bool) -> std::io::Result<Self> {
        todo!();
    }
}

impl FivegsMobileIdentity {
    pub fn encode(&self, encode_iei: bool) -> Vec<u8> {
        vec![]
    }

    pub fn decode(data: &[u8], decode_iei: bool) -> std::io::Result<Self> {
        todo!();
    }
}
