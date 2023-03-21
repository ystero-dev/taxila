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

use crate::amf::messages::{NasPduMessage, NgapToAmfMessage};

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

        log::debug!("Message: {:#?}", initial_ue);

        let mut nas_pdu = None;
        let mut user_location = None;
        let mut rrc_establishment_cause = None;
        let mut ue_context_requested = None;

        let mut ran_ue_ngap_id = None;
        for ie in initial_ue.protocol_i_es.0 {
            match ie.value {
                InitialIEValue::Id_RAN_UE_NGAP_ID(r) => {
                    log::debug!("RAN_UE_NGAP_ID_FOUND");
                    ran_ue_ngap_id.replace(r.0);
                }
                InitialIEValue::Id_NAS_PDU(inner_nas_pdu) => {
                    nas_pdu.replace(inner_nas_pdu);
                }
                InitialIEValue::Id_UserLocationInformation(inner_user_location) => {
                    user_location.replace(inner_user_location);
                }
                InitialIEValue::Id_RRCEstablishmentCause(inner_rrc_establishment_cause) => {
                    rrc_establishment_cause.replace(inner_rrc_establishment_cause);
                }
                InitialIEValue::Id_UEContextRequest(inner_ue_context_requested) => {
                    ue_context_requested = ue_context_requested.replace(inner_ue_context_requested);
                }
                _ => {
                    log::warn!("Unsupported IE: {:?}", ie);
                }
            }
        }

        if ran_ue_ngap_id.is_none() || nas_pdu.is_none() || user_location.is_none() {
            return self
                .send_error_indication(
                    id,
                    ran_ue_ngap_id.unwrap(),
                    Cause::Protocol(CauseProtocol(CauseProtocol::ABSTRACT_SYNTAX_ERROR_REJECT)),
                )
                .await;
        }

        if rrc_establishment_cause.is_none() {
            log::warn!("Missing mandatory `RRCEstablishmentCause IE`.");
        }

        // Store the received information in the `NgapRanUe` and then pass the PDU for NAS
        // processing.
        //
        // TODO: Is it possible that we receive an `InitialUE` message for a UE, but we have a Nas
        // context available for that UE? Usually in the case of a handoff, so right now we are not
        // considering that possibility. `InitialUE` means, this is the first message from the UE
        // that we are receiving.
        //
        // The returned `id` is the `amf_ngap_ue_id`
        let id = self.add_ran_ue(
            id,
            sid,
            ran_ue_ngap_id.unwrap(),
            user_location.unwrap(),
            ue_context_requested,
            rrc_establishment_cause,
        );

        let pdu = nas_pdu.unwrap();
        let message = NgapToAmfMessage::NasPduMessage(NasPduMessage {
            id,
            pdu,
            initial_ue: true,
        });
        let _ = self.ngap_to_amf_tx.as_ref().unwrap().send(message).await;

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
