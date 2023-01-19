//! Handling of Initial UE Message

use sctp_rs::AssociationId;

// Common NGAP Types
use ngap::messages::r17::{Criticality, ProcedureCode, ProtocolIE_ID, NGAP_PDU};

// Initial UE Message Types
use ngap::messages::r17::InitialUEMessage;
use ngap::messages::r17::InitialUEMessageProtocolIEs_EntryValue as InitialIEValue;
use ngap::messages::r17::{Cause, CauseProtocol};
use ngap::messages::r17::{
    ErrorIndication, ErrorIndicationProtocolIEs, ErrorIndicationProtocolIEs_Entry,
    ErrorIndicationProtocolIEs_EntryValue, InitiatingMessage, InitiatingMessageValue, ID_CAUSE,
    ID_ERROR_INDICATION, ID_RAN_UE_NGAP_ID, RAN_UE_NGAP_ID,
};

use super::ngap_manager::NgapManager;

impl NgapManager {
    pub(super) async fn process_initial_ue_message(
        &mut self,
        id: AssociationId,
        sid: u16,
        initial_ue: InitialUEMessage,
    ) -> std::io::Result<()> {
        log::debug!(
            "Processing 'InitialUEMessage' received on AssociationID: {}, Stream ID: {}",
            id,
            sid
        );

        log::trace!("Message: {:#?}", initial_ue);

        let mut ran_ue_ngap_id_present = false;
        let mut nas_pdu_present = false;
        let mut user_location_info_present = false;
        let mut rrc_establishment_cause_present = false;

        let mut ran_ue_ngap_id = 0;
        for ie in initial_ue.protocol_i_es.0 {
            match ie.value {
                InitialIEValue::Id_RAN_UE_NGAP_ID(r) => {
                    ran_ue_ngap_id = r.0;
                    ran_ue_ngap_id_present = true;
                }
                InitialIEValue::Id_NAS_PDU(_nas_pdu) => {
                    nas_pdu_present = true;
                }
                InitialIEValue::Id_UserLocationInformation(_user_location_info) => {
                    user_location_info_present = true;
                }
                InitialIEValue::Id_RRCEstablishmentCause(_rrc_establishment_cause) => {
                    rrc_establishment_cause_present = true;
                }
                InitialIEValue::Id_UEContextRequest(_ue_context_requested) => {
                    // UE context requested.
                }
                _ => {
                    log::warn!("Unsupported IE: {:?}", ie);
                }
            }
        }

        if !ran_ue_ngap_id_present || !nas_pdu_present || !user_location_info_present {
            return self
                .send_error_indication(
                    id,
                    ran_ue_ngap_id,
                    Cause::Protocol(CauseProtocol(CauseProtocol::ABSTRACT_SYNTAX_ERROR_REJECT)),
                )
                .await;
        }

        if !rrc_establishment_cause_present {
            log::warn!("Missing mandatory `RRCEstablishmentCause IE`.");
        }

        self.add_ran_ue(ran_ue_ngap_id, sid);

        Ok(())
    }

    pub(super) async fn send_error_indication(
        &self,
        id: AssociationId,
        ran_ue_ngap_id: u32,
        cause: Cause,
    ) -> std::io::Result<()> {
        let ran_ue_id_ie = ErrorIndicationProtocolIEs_Entry {
            id: ProtocolIE_ID(ID_RAN_UE_NGAP_ID),
            criticality: Criticality(Criticality::IGNORE),
            value: ErrorIndicationProtocolIEs_EntryValue::Id_RAN_UE_NGAP_ID(RAN_UE_NGAP_ID(
                ran_ue_ngap_id,
            )),
        };

        let cause_ie = ErrorIndicationProtocolIEs_Entry {
            id: ProtocolIE_ID(ID_CAUSE),
            criticality: Criticality(Criticality::IGNORE),
            value: ErrorIndicationProtocolIEs_EntryValue::Id_Cause(cause),
        };

        let ies = vec![ran_ue_id_ie, cause_ie];

        let error_indication = ErrorIndication {
            protocol_i_es: ErrorIndicationProtocolIEs(ies),
        };
        let pdu = NGAP_PDU::InitiatingMessage(InitiatingMessage {
            procedure_code: ProcedureCode(ID_ERROR_INDICATION),
            criticality: Criticality(Criticality::IGNORE),
            value: InitiatingMessageValue::Id_ErrorIndication(error_indication),
        });

        self.ngap_send_pdu(id, pdu, Some(ran_ue_ngap_id)).await
    }
}
