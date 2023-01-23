//! NAS Messages

use super::headers::Nas5gMmMessageHeader;
use super::ies::{FivegRegistrationType, NasKeySetId};

struct RegistrationRequest {
    header: Nas5gMmMessageHeader,
    reg_type: FivegRegistrationType,
    ngksi: NasKeySetId,
}

impl RegistrationRequest {
    pub fn encode(&self) -> Vec<u8> {
        vec![]
    }

    pub fn decode(data: &[u8]) -> std::io::Result<Self> {
        todo!();
    }
}
