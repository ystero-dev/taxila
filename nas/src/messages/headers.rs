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

impl NasMessageHeader {
    pub fn decode(data: &[u8]) -> std::io::Result<(Self, usize)> {
        let mut decoded = 0;
        let extended_protocol_disc = match data[decoded] {
            0x2E => ExtProtoDiscriminator::FivegNasSessionManagementType,
            0x7E => ExtProtoDiscriminator::FivegNasMobilityManagementType,
            _ => unreachable!(),
        };
        decoded += 1;

        match extended_protocol_disc {
            // Protocol Discriminator is 5G MM. The message could either be a 5G MM Message or a
            // security protected (with security header) 5G MM or 5G SM message. Section 9.2 24.501
            ExtProtoDiscriminator::FivegNasMobilityManagementType => {
                let sec_header_type = match data[decoded] & 0x0F {
                    0 => Nas5gSecurityHeader::PlainText,
                    1 => Nas5gSecurityHeader::IntegrityProtected,
                    2 => Nas5gSecurityHeader::IntegrityProtectedAndCiphered,
                    3 => Nas5gSecurityHeader::IntegrityProtectedSecurityModeCommand,
                    4 => Nas5gSecurityHeader::IntegrityProtectedSecurityModeComplete,
                    _ => todo!(),
                };
                decoded += 1;

                match sec_header_type {
                    // Not Security Protected: a 5GMM Message
                    Nas5gSecurityHeader::PlainText => {
                        let message_type = data[decoded];
                        decoded += 1;
                        Ok((
                            Self::Nas5gMm(Nas5gMmMessageHeader {
                                extended_protocol_disc,
                                sec_header_type,
                                message_type,
                            }),
                            decoded,
                        ))
                    }
                    // Security Protected Header with NAS Container containing the actual message.
                    _ => {
                        let mac =
                            u32::from_be_bytes(data[decoded..decoded + 4].try_into().unwrap());
                        decoded += 4;

                        let seq_no = data[decoded];
                        decoded += 1;
                        Ok((
                            Self::SecurityProtected(SecurityProtectedHeader {
                                extended_protocol_disc,
                                sec_header_type,
                                mac,
                                seq_no,
                            }),
                            decoded,
                        ))
                    }
                }
            }
            // 5G SM Message: Not ciphered 5G SM Message. This could be a result of an initial not
            // security protected 5G SM Message (eg. Service Request) or a 5G SM message contained
            // in the NAS container of a security protected 5G NAS message.
            ExtProtoDiscriminator::FivegNasSessionManagementType => {
                let pdu_session_identity = data[decoded];
                decoded += 1;

                let proc_transaction_identity = data[decoded];
                decoded += 1;

                Ok((
                    Self::Nas5gSm(Nas5gSmMessageHeader {
                        extended_protocol_disc,
                        pdu_session_identity,
                        proc_transaction_identity,
                    }),
                    decoded,
                ))
            }
        }
    }
}
