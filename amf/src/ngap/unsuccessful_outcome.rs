use sctp_rs::AssociationId;

use ngap::messages::r17::UnsuccessfulOutcome;

use super::structs::NgapManager;

impl NgapManager {
    pub(super) fn process_unsuccessful_outcome(
        &self,
        _id: AssociationId,
        _failure: UnsuccessfulOutcome,
    ) {
    }
}
