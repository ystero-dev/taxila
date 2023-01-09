use sctp_rs::AssociationId;

use crate::ngap::messages::r17::UnsuccessfulOutcome;

use super::structs::Amf;

impl Amf {
    pub(super) fn process_unsuccessful_outcome(
        &self,
        _id: AssociationId,
        _failure: UnsuccessfulOutcome,
    ) {
    }
}
