mod mm;
pub use mm::Nas5gMmMessageHeader;

mod sm;
pub use sm::Nas5gSmMessageHeader;

/// An Enum representing Extended Protocol Discriminator
/// See 24.007 (Release 17) Section 11.2.3.1A
#[repr(u8)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExtProtoDiscriminator {
    /// 5GS Session Management Messages
    FivegNasSessionManagementType = 0x2E,

    /// 5GS Mobility Management Messages
    FivegNasMobilityManagementType = 0x7E,
}

impl From<u8> for ExtProtoDiscriminator {
    fn from(val: u8) -> Self {
        match val {
            0x7E => Self::FivegNasMobilityManagementType,
            0x2E => Self::FivegNasSessionManagementType,
            _ => unreachable!(),
        }
    }
}

/// NAS Message Header: A common structure representing NAS Message Header
/// 24.501 (Release 17) Section 9 General message format and information elements coding
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NasMessageHeader {
    SecurityProtected(SecurityProtectedHeader),
    Nas5gMm(Nas5gMmMessageHeader),
    Nas5gSm(Nas5gSmMessageHeader),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SecurityProtectedHeader {
    pub extended_protocol_disc: ExtProtoDiscriminator,
    pub sec_header_type: Nas5gSecurityHeader,
    pub mac: u32,
    pub seq_no: u8,
}

/// An Enum representing NAS Security Header Type
/// See 24.501 (Release 17) Section 9.3
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Nas5gSecurityHeader {
    /// No Security and Integrity protection for NAS Messages.
    PlainText = 0x00,

    /// Integrity Protected.
    IntegrityProtected = 0x01,

    /// Integrity Protected and Ciphered
    IntegrityProtectedAndCiphered = 0x02,

    /// Integrity Protected Security Mode Command
    IntegrityProtectedSecurityModeCommand = 0x03,

    /// Integrity Protected Security Mode Complete
    IntegrityProtectedSecurityModeComplete = 0x04,
}
