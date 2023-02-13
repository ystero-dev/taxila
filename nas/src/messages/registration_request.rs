//! NAS Messages

use super::headers::Nas5gMmMessageHeader;
use super::ies::{FivegRegistrationType, FivegsMobileIdentity, NasKeySetId, UeSecurityCapability};

#[derive(Debug, Eq, PartialEq)]
pub struct RegistrationRequest {
    header: Nas5gMmMessageHeader,
    req_type: FivegRegistrationType,
    ngksi: NasKeySetId,
    identity: FivegsMobileIdentity,
    ue_sec_capability: Option<UeSecurityCapability>,
}

impl RegistrationRequest {
    pub const UE_SEC_CAPABILITY_IEI: u8 = 0x2E;

    pub fn encode(&self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];
        output.extend(self.header.encode());
        output.extend(self.req_type.encode(false));
        output.extend(self.ngksi.encode(false));
        output.extend(self.identity.encode(false));

        output
    }

    pub fn decode(data: &[u8]) -> std::io::Result<(Self, usize)> {
        log::trace!("RegistrationRequest decode.");
        let mut decoded = 0;

        let (header, header_decoded) = Nas5gMmMessageHeader::decode(&data[decoded..])?;
        decoded += header_decoded;

        assert_eq!(header.message_type, super::MM_MSG_TYPE_REGISTRATION_REQUEST);

        let (req_type, req_type_decoded) =
            FivegRegistrationType::decode(&data[decoded..], false, false)?;
        decoded += req_type_decoded;

        let (ngksi, ngksi_decoded) = NasKeySetId::decode(&data[decoded..], false, true)?;
        decoded += ngksi_decoded;

        // Since both the above return 0, we now increment
        decoded += 1;

        let (identity, identity_decoded) = FivegsMobileIdentity::decode(&data[decoded..], false)?;
        decoded += identity_decoded;

        let mut ue_sec_capability = None;
        while decoded < data.len() {
            let value = data[decoded];
            let iei = if value > 0x80 {
                (value & 0xF0) >> 4
            } else {
                value
            };

            match iei {
                Self::UE_SEC_CAPABILITY_IEI => {
                    let (ue_sec_cap, ue_sec_cap_decoded) =
                        UeSecurityCapability::decode(&data[decoded..])?;
                    decoded += ue_sec_cap_decoded;
                    ue_sec_capability = Some(ue_sec_cap);
                }
                _ => {
                    log::error!("Unsupported IEI Type: {:x}", value);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!(
                            "Unsupported IEI :{:x}, {:?}, decoded: {}",
                            value, identity, decoded
                        ),
                    ));
                }
            }
        }

        Ok((
            Self {
                header,
                req_type,
                ngksi,
                identity,
                ue_sec_capability,
            },
            decoded,
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn decode_registration_request() {
        let data = [
            126, 0, 65, 121, 0, 13, 1, 9, 241, 7, 0, 0, 0, 0, 0, 0, 0, 0, 16, 46, 4, 240, 240, 240,
            240,
        ];

        let result = RegistrationRequest::decode(&data);
        assert!(result.is_ok(), "{:#?}", result.err().unwrap());

        let (reg_request, _decoded) = result.unwrap();
        assert!(matches!(
            reg_request,
            RegistrationRequest {
                header: Nas5gMmMessageHeader { .. },
                ..
            }
        ));
    }
}
