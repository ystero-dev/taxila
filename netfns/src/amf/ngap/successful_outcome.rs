//! Handling of `SuccessfulOutcome` messages received from RAN Node

use sctp_rs::AssociationId;

use ngap::messages::r17::SuccessfulOutcome;

use super::ngap_manager::NgapManager;

impl NgapManager {
    pub(super) fn process_successful_outcome(
        &self,
        _id: AssociationId,
        _sid: u16,
        success: SuccessfulOutcome,
    ) -> std::io::Result<()> {
        log::error!("Unsupported Message received: {:?}", success.procedure_code);
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported Initiating Message".to_string(),
        ))
    }
}
