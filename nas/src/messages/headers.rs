mod mm;
pub use mm::{Nas5gMmMessageHeader, Nas5gSecurityHeader};

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
