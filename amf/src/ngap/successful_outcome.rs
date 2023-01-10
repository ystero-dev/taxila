use sctp_rs::AssociationId;

use ngap::messages::r17::SuccessfulOutcome;

use super::structs::NgapManager;

impl NgapManager {
    pub(super) fn process_successful_outcome(
        &self,
        _id: AssociationId,
        _success: SuccessfulOutcome,
    ) {
    }
}
