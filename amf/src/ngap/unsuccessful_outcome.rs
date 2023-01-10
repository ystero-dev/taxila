use sctp_rs::AssociationId;

use ngap::messages::r17::UnsuccessfulOutcome;

use super::ngap_manager::NgapManager;

impl NgapManager {
    pub(super) fn process_unsuccessful_outcome(
        &self,
        _id: AssociationId,
        _failure: UnsuccessfulOutcome,
    ) {
    }
}
