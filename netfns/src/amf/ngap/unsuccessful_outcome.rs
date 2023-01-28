//! Handling of `UnsuccessfulOutcome` messages received from RAN Node

use sctp_rs::AssociationId;

use ngap::messages::r17::UnsuccessfulOutcome;

use super::ngap_manager::NgapManager;

impl NgapManager {
    pub(super) fn process_unsuccessful_outcome(
        &self,
        _id: AssociationId,
        _sid: u16,
        failure: UnsuccessfulOutcome,
    ) -> std::io::Result<()> {
        log::error!("Unsupported Message received: {:?}", failure.procedure_code);
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Unsupported Initiating Message".to_string(),
        ))
    }
}
