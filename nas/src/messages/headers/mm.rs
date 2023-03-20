/// NAS 5GS Memory Management common header. This header will be part of all NAS Messages from the
/// 24.501 (Release 17) Section 8.2 5 GS mobility management messages.
use super::{ExtProtoDiscriminator, Nas5gSecurityHeader};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Nas5gMmMessageHeader {
    pub extended_protocol_disc: ExtProtoDiscriminator,
    pub sec_header_type: Nas5gSecurityHeader,
    pub message_type: u8,
}

impl Nas5gMmMessageHeader {
    pub fn encode(&self) -> Vec<u8> {
        vec![]
    }

    pub fn decode(data: &[u8]) -> std::io::Result<(Self, usize)> {
        let mut decoded = 0;
        let extended_protocol_disc = match data[decoded] {
            0x2E => ExtProtoDiscriminator::FivegNasSessionManagementType,
            0x7E => ExtProtoDiscriminator::FivegNasMobilityManagementType,
            _ => unreachable!(),
        };
        decoded += 1;

        let sec_header_type = match data[decoded] & 0x0F {
            0 => Nas5gSecurityHeader::PlainText,
            1 => Nas5gSecurityHeader::IntegrityProtected,
            2 => Nas5gSecurityHeader::IntegrityProtectedAndCiphered,
            3 => Nas5gSecurityHeader::IntegrityProtectedSecurityModeCommand,
            4 => Nas5gSecurityHeader::IntegrityProtectedSecurityModeComplete,
            _ => todo!(),
        };
        decoded += 1;

        let message_type = data[decoded];
        decoded += 1;

        Ok((
            Self {
                extended_protocol_disc,
                sec_header_type,
                message_type,
            },
            decoded,
        ))
    }
}
