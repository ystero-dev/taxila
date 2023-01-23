/// NAS 5GS Memory Management common header.

pub struct Nas5gMmMessageHeader {
    extended_protocol_disc: ExtProtoDiscriminator,
    security_header_type: Nas5gSecurityHeaderType,
    message_type: u8,
}

/// An Enum representing Extended Protocol Discriminator
/// See 24.007 (Release 17) Section 11.2.3.1A
#[repr(u8)]
pub enum ExtProtoDiscriminator {
    /// 5GS Session Management Messages
    FiveGNasSessionManagementType = 0x2E,

    /// 5GS Mobility Management Messages
    FiveGNasMobilityManagementType = 0x7E,
}

/// An Enum representing NAS Security Header Type
/// See 24.501 (Release 17) Section 9.3
pub enum Nas5gSecurityHeaderType {
    /// No Security and Integrity protection for NAS Messages.
    PlainText = 0x00,

    /// Integrity Protected.
    IntegrityProtected = 0x01,

    /// Integrity Protected and Ciphered
    IntegritypProtectedAndCiphered = 0x02,

    /// Integrity Protected Security Mode Command
    IntegrityProtectedSecurityModeCommand = 0x03,

    /// Integrity Protected Security Mode Complete
    IntegrityProtectedSecurityModeComplete = 0x04,
}

impl Nas5gMmMessageHeader {
    pub fn encode(&self) -> Vec<u8> {
        vec![]
    }

    pub fn decode(data: &[u8]) -> std::io::Result<Self> {
        todo!();
    }
}
