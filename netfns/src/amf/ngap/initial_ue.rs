//! Handling of Initial UE Message

use sctp_rs::AssociationId;

// Common NGAP Types
use ngap::messages::r17::{
    Criticality, ProcedureCode, ProtocolIE_ID, SuccessfulOutcome, SuccessfulOutcomeValue,
    UnsuccessfulOutcome, UnsuccessfulOutcomeValue, ID_INITIAL_UE_MESSAGE, NGAP_PDU,
};

// Initial UE Message Types
use ngap::messages::r17::InitialUEMessage;

use super::ngap_manager::NgapManager;

impl NgapManager {
    pub(super) async fn process_initial_ue_message(
        &mut self,
        id: AssociationId,
        sid: u16,
        initial_ue: InitialUEMessage,
    ) -> std::io::Result<()> {
        log::debug!(
            "Processing 'NgSetupRequest' received on AssociationID: {}, Stream ID: {}",
            id,
            sid
        );

        log::trace!("Message: {:#?}", initial_ue);

        let mut ran_ue_ngap_id_present = false;
        let mut nas_pdu_present = false;
        let mut user_location_information_present = false;
        let mut rrc_establishment_present = false;

        for ie in initial_ue.protocol_i_es.0 {
            log::info!("ie: {:#?}", ie);
        }

        Ok(())
    }
}
