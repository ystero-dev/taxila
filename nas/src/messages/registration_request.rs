//! NAS Messages

use super::headers::Nas5gMmMessageHeader;
use super::ies::{FivegRegistrationType, FivegsMobileIdentity, NasKeySetId};

struct RegistrationRequest {
    header: Nas5gMmMessageHeader,
    req_type: FivegRegistrationType,
    ngksi: NasKeySetId,
    identity: FivegsMobileIdentity,
}

impl RegistrationRequest {
    pub fn encode(&self) -> Vec<u8> {
        let mut output: Vec<u8> = vec![];
        output.extend(self.header.encode());
        output.extend(self.req_type.encode(false));
        output.extend(self.ngksi.encode(false));
        output.extend(self.identity.encode(false));

        output
    }

    pub fn decode(data: &[u8]) -> std::io::Result<(Self, usize)> {
        todo!();
    }
}
