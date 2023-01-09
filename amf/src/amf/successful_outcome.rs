use sctp_rs::AssociationId;

use ngap::messages::r17::SuccessfulOutcome;

use super::structs::Amf;

impl Amf {
    pub(super) fn process_successful_outcome(
        &self,
        _id: AssociationId,
        _success: SuccessfulOutcome,
    ) {
    }
}
