//! Structures, Enumerations and constants for NAS 5G MM Messages.

use crate::messages::headers::Nas5gMmMessageHeader;

mod registration_request;
pub use registration_request::RegistrationRequest;

pub const MM_MSG_TYPE_REGISTRATION_REQUEST: u8 = 0x41;

/// NAS 5G MM Message. The Message will be one of the 5G MM Messages.
pub enum Nas5gMmMessage {
    RegistrationRequest(RegistrationRequest),
}

impl Nas5gMmMessage {
    pub fn decode(data: &[u8]) -> std::io::Result<Self> {
        let mut decoded = 0;
        // TODO: Not sure yet whether we need to 'keep' the header part in the underlying message
        // structure. For now keeping it, but may be we will have to revisit that part.
        //
        // We know for a fact that this is `Nas5gMmMessageHeader`. If it was a security protected
        // message, this function would be called 'after' we decrypt and integrity verify the
        // message.
        //
        // TODO: Do we error if this is not plain text?
        let (header, _) = Nas5gMmMessageHeader::decode(data)?;

        match header.message_type {
            MM_MSG_TYPE_REGISTRATION_REQUEST => {
                let (reg_request, decoded) = RegistrationRequest::decode(data)?;
                if decoded != data.len() {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!(
                            "Undecoded octets. Decoded: {}, length: {}",
                            decoded,
                            data.len()
                        ),
                    ))
                } else {
                    Ok(Self::RegistrationRequest(reg_request))
                }
            }
            _ => todo!(),
        }
    }
}
