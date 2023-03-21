//! The Main `AmfUe` structure

use ngap::messages::r17::NAS_PDU;

use nas::messages::{
    headers::{Nas5gMmMessageHeader, Nas5gSecurityHeader, NasMessageHeader},
    Nas5gMmMessage, RegistrationRequest,
};

mod registration_procedure;

#[derive(Debug, Clone)]
pub(in crate::amf) struct AmfUe {
    pub(in crate::amf) amf_ue_ngap_id: u64,
}

impl AmfUe {
    pub(in crate::amf) fn new_amf_ue(amf_ue_ngap_id: u64) -> Self {
        Self { amf_ue_ngap_id }
    }

    // Handle Initial NAS MM Message
    pub(in crate::amf) fn handle_initial_nas_message(
        &mut self,
        nas_pdu: NAS_PDU,
    ) -> std::io::Result<()> {
        let (header, decoded) = NasMessageHeader::decode(&nas_pdu.0)?;
        match header {
            NasMessageHeader::SecurityProtected(_) => {
                self.handle_security_protected_initial_nas_message(nas_pdu)
            }
            NasMessageHeader::Nas5gMm(_) => self.handle_initial_nas_5gmm_message(nas_pdu),
            NasMessageHeader::Nas5gSm(_) => self.handle_initial_nas_5gsm_message(nas_pdu),
        }
    }

    pub(in crate::amf) fn handle_nas_message(&mut self, nas_pdu: NAS_PDU) -> std::io::Result<()> {
        todo!();
    }

    pub(in crate::amf) fn handle_security_protected_initial_nas_message(
        &mut self,
        nas_pdu: NAS_PDU,
    ) -> std::io::Result<()> {
        todo!();
    }

    pub(in crate::amf) fn handle_initial_nas_5gmm_message(
        &mut self,
        nas_pdu: NAS_PDU,
    ) -> std::io::Result<()> {
        let message = Nas5gMmMessage::decode(&nas_pdu.0)?;

        match message {
            Nas5gMmMessage::RegistrationRequest(reg_request) => {
                self.registration_procedure(reg_request, true)
            }
        }
    }

    pub(in crate::amf) fn handle_initial_nas_5gsm_message(
        &mut self,
        nas_pdu: NAS_PDU,
    ) -> std::io::Result<()> {
        todo!();
    }
}
