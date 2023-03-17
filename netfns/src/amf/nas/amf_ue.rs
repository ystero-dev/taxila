//! The Main `AmfUe` structure

use ngap::messages::r17::NAS_PDU;

use nas::messages::{
    headers::{Nas5gMmMessageHeader, Nas5gSecurityHeader},
    RegistrationRequest, MM_MSG_TYPE_REGISTRATION_REQUEST,
};

#[derive(Debug, Clone)]
pub(in crate::amf) struct AmfUe {
    pub(in crate::amf) amf_ue_ngap_id: u64,
}

impl AmfUe {
    pub(in crate::amf) fn new_amf_ue(amf_ue_ngap_id: u64) -> Self {
        Self { amf_ue_ngap_id }
    }

    // Handle Initial NAS MM Message
    pub(in crate::amf) fn handle_initial_mm_message(
        &mut self,
        nas_pdu: NAS_PDU,
    ) -> std::io::Result<()> {
        let (header, decoded) = Nas5gMmMessageHeader::decode(&nas_pdu.0)?;
        match header.sec_header_type {
            Nas5gSecurityHeader::PlainText => Self::decode_nas_message(&header, nas_pdu),
            _ => todo!(),
        }
    }

    pub(in crate::amf) fn handle_mm_message(&mut self, nas_pdu: NAS_PDU) -> std::io::Result<()> {
        todo!();
    }

    // This is a decrypted message, which will be decoded and right now only printed. Eventually,
    // this is the message type that will be returned by this function.
    fn decode_nas_message(header: &Nas5gMmMessageHeader, nas_pdu: NAS_PDU) -> std::io::Result<()> {
        if header.message_type == MM_MSG_TYPE_REGISTRATION_REQUEST {
            let (reg_request, decoded) = RegistrationRequest::decode(&nas_pdu.0)?;
            log::debug!("Reg Request: {:#?}", reg_request);
        }
        Ok(())
    }
}
