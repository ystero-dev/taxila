//! Handling of NGSetup Messages.
//!
//! This module deals with handling of `NGSetup` messages and sending appropriate response
//! messages.

use sctp_rs::AssociationId;

// Common NGAP Types
use ngap::messages::r17::{
    Criticality, ProcedureCode, ProtocolIE_ID, SuccessfulOutcome, SuccessfulOutcomeValue,
    UnsuccessfulOutcome, UnsuccessfulOutcomeValue, ID_NG_SETUP, NGAP_PDU,
};

// Types related to NGSetupRequest
use ngap::messages::r17::{
    GlobalRANNodeID, NGSetupRequest, NGSetupRequestProtocolIEs_EntryValue as RequestIEValue,
    SupportedTAList,
};

// Types related to NGSetupResponse
use ngap::messages::r17::{
    AMFName, NGSetupResponse, NGSetupResponseProtocolIEs, NGSetupResponseProtocolIEs_Entry,
    NGSetupResponseProtocolIEs_EntryValue as ResponseIEValue, PLMNIdentity, PLMNSupportItem,
    PLMNSupportList, RelativeAMFCapacity, ServedGUAMIItem, ServedGUAMIList, SliceSupportItem,
    SliceSupportList, GUAMI, ID_AMF_NAME, ID_PLMN_SUPPORT_LIST, ID_RELATIVE_AMF_CAPACITY,
    ID_SERVED_GUAMI_LIST, SST, S_NSSAI,
};

// Types related to NGSetupFailure
use ngap::messages::r17::{
    Cause, CauseMisc, CauseProtocol, CriticalityDiagnostics, NGSetupFailure,
    NGSetupFailureProtocolIEs, NGSetupFailureProtocolIEs_Entry,
    NGSetupFailureProtocolIEs_EntryValue as FailureIE, ID_CAUSE, ID_CRITICALITY_DIAGNOSTICS,
};

use crate::amf::config::PlmnConfig;

use super::ngap_manager::{NgapManager, RanNode};

impl NgapManager {
    pub(super) async fn process_ng_setup_request(
        &mut self,
        id: AssociationId,
        ngsetup: NGSetupRequest,
    ) -> std::io::Result<()> {
        log::debug!(
            "Processing 'NgSetupRequest' received on AssociationID: {}",
            id,
        );

        log::trace!("Message: {:#?}", ngsetup);

        let mut global_rannode_id_present = false;
        let mut supported_ta_list_present = false;
        let mut paging_drx_present = false;
        let mut ran_ta_supported = false;

        let mut ran_node_id: Option<Box<GlobalRANNodeID>> = None;
        let mut supported_ta_list: Option<Box<SupportedTAList>> = None;
        let mut name: Option<String> = None;
        for ie in ngsetup.protocol_i_es.0 {
            match ie.value {
                RequestIEValue::Id_DefaultPagingDRX(_paging_drx) => {
                    paging_drx_present = true;
                }
                RequestIEValue::Id_Extended_RANNodeName(_ext_ran_node_name) => {}
                RequestIEValue::Id_GlobalRANNodeID(global_ran_node_id) => {
                    global_rannode_id_present = true;
                    ran_node_id = Some(Box::new(global_ran_node_id));
                }
                RequestIEValue::Id_NB_IoT_DefaultPagingDRX(_nb_iot_def_paging_drx) => {
                    log::warn!("Received unhandled NB_IOT Default Paging DRX");
                }
                RequestIEValue::Id_RANNodeName(ran_node_name) => {
                    name = Some(ran_node_name.0.clone());
                }
                RequestIEValue::Id_SupportedTAList(recd_supported_ta_list) => {
                    supported_ta_list_present = true;
                    ran_ta_supported = Self::any_tas_supported(
                        &recd_supported_ta_list,
                        &self.config.plmn,
                        &self.config.tacs,
                    );
                    supported_ta_list = Some(Box::new(recd_supported_ta_list));
                }
                RequestIEValue::Id_UERetentionInformation(_ue_retention_info) => {
                    log::warn!("Received unhandled UE Retention Information");
                }
            }
        }
        if !global_rannode_id_present || !supported_ta_list_present {
            log::error!("Missing Mandatory IEs with Criticality Reject, Sending Failure.");
            return self
                .send_ngsetup_failure(
                    id,
                    Cause::Protocol(CauseProtocol(CauseProtocol::ABSTRACT_SYNTAX_ERROR_REJECT)),
                    None,
                )
                .await;
        }

        if !paging_drx_present {
            log::warn!("PagingDRX IE not present, using the default configured value.");
            // TODO: May be send Criticality Diagnostics with Success Response?
        }

        if !ran_ta_supported {
            log::error!("None of the RAN TAs supported!");
            return self
                .send_ngsetup_failure(
                    id,
                    Cause::Misc(CauseMisc(CauseMisc::UNKNOWN_PLMN_OR_SNPN)),
                    None,
                )
                .await;
        }

        let name = if name.is_none() {
            String::new()
        } else {
            name.unwrap()
        };

        self.send_ngsetup_success(id).await?;

        let ran_node = RanNode {
            sctp_id: id,
            ran_node_id: ran_node_id.unwrap(),
            supported_ta_list: supported_ta_list.unwrap(),
            name,
            next_ue_stream: 1,
            ngsetup_success: true,
        };

        log::info!(
            "NGSetupRequest Processing Successful. NGSetup complete for RAN Node[{}]",
            ran_node
        );

        self.ran_nodes.insert(id, ran_node);

        Ok(())
    }

    // If any of the RAN TAs received matches the configured TAs.
    pub(crate) fn any_tas_supported(
        ran_tas: &SupportedTAList,
        plmn: &PlmnConfig,
        tacs: &Vec<u32>,
    ) -> bool {
        log::debug!("Checking if Matching TAs found for current Config.");

        for tac in tacs {
            log::trace!("Checking for TAC: {}", tac);
            for supported_ta in &ran_tas.0 {
                if supported_ta.tac != *tac {
                    log::trace!("TAC: {} Not supported!", tac);
                    continue;
                }
                log::trace!(
                    "TAC Matching, now checking PLMN. PLMN From Config: {:?}",
                    PLMNIdentity::from_mcc_mnc(plmn.mcc, plmn.mnc)
                );
                for item in &supported_ta.broadcast_plmn_list.0 {
                    log::trace!("Checking for PLMN Identity: {:?}", item.plmn_identity);
                    if item.plmn_identity.0 == PLMNIdentity::from_mcc_mnc(plmn.mcc, plmn.mnc).0 {
                        log::trace!("Found: matching MCC({}): MNC({})", plmn.mcc, plmn.mnc);
                        return true;
                    }
                }
            }
        }

        log::debug!("No Matching TAs found!");
        false
    }

    async fn send_ngsetup_success(&self, id: AssociationId) -> std::io::Result<()> {
        log::debug!("Sending `NGSetupResponse` (Success).");

        // Prepare the NGSetup Success
        //
        // IEs first
        let mut ies = vec![];

        // AMF Name
        let amf_name_ie = NGSetupResponseProtocolIEs_Entry {
            id: ProtocolIE_ID(ID_AMF_NAME),
            criticality: Criticality(Criticality::REJECT),
            value: ResponseIEValue::Id_AMFName(AMFName(self.config.amf_name.clone())),
        };
        ies.push(amf_name_ie);

        // Served GUAMI List
        //
        let mut guamis = vec![];
        let served_guami_item = ServedGUAMIItem {
            guami: GUAMI::from_plmn_id_and_amf_id(
                self.config.plmn.mcc,
                self.config.plmn.mnc,
                self.config.amf_id.region,
                self.config.amf_id.set,
                self.config.amf_id.pointer,
            ),
            backup_amf_name: None,
            ie_extensions: None,
        };
        guamis.push(served_guami_item);

        let served_guami_list_ie = NGSetupResponseProtocolIEs_Entry {
            id: ProtocolIE_ID(ID_SERVED_GUAMI_LIST),
            criticality: Criticality(Criticality::REJECT),
            value: ResponseIEValue::Id_ServedGUAMIList(ServedGUAMIList(guamis)),
        };
        ies.push(served_guami_list_ie);

        // PLMN Support List
        let mut plmns = vec![];
        let mut slices = vec![];

        let slice_item = SliceSupportItem {
            s_nssai: S_NSSAI {
                sst: SST(vec![1]),
                sd: None,
                ie_extensions: None,
            },
            ie_extensions: None,
        };
        slices.push(slice_item);

        let plmn_support_item = PLMNSupportItem {
            plmn_identity: PLMNIdentity::from_mcc_mnc(self.config.plmn.mcc, self.config.plmn.mnc),
            slice_support_list: SliceSupportList(slices),
            ie_extensions: None,
        };
        plmns.push(plmn_support_item);
        let plmn_support_ie = NGSetupResponseProtocolIEs_Entry {
            id: ProtocolIE_ID(ID_PLMN_SUPPORT_LIST),
            criticality: Criticality(Criticality::REJECT),
            value: ResponseIEValue::Id_PLMNSupportList(PLMNSupportList(plmns)),
        };
        ies.push(plmn_support_ie);

        // Relative AMF Capacity
        let relative_amf_capacity_ie = NGSetupResponseProtocolIEs_Entry {
            id: ProtocolIE_ID(ID_RELATIVE_AMF_CAPACITY),
            criticality: Criticality(Criticality::REJECT),
            value: ResponseIEValue::Id_RelativeAMFCapacity(RelativeAMFCapacity(255)),
        };
        ies.push(relative_amf_capacity_ie);

        let response = SuccessfulOutcome {
            procedure_code: ProcedureCode(ID_NG_SETUP),
            criticality: Criticality(Criticality::REJECT),
            value: SuccessfulOutcomeValue::Id_NGSetup(NGSetupResponse {
                protocol_i_es: NGSetupResponseProtocolIEs(ies),
            }),
        };

        let pdu = NGAP_PDU::SuccessfulOutcome(response);
        if let Err(e) = self.ngap_send_pdu(id, pdu).await {
            log::error!("Error in Sending NGSetupResponse. ({})", e);
            Err(e)
        } else {
            Ok(())
        }
    }

    async fn send_ngsetup_failure(
        &self,
        id: AssociationId,
        cause: Cause,
        diag: Option<CriticalityDiagnostics>,
    ) -> std::io::Result<()> {
        log::debug!("Sending `NGSetupFailure` (Failure).");

        let mut ies = vec![];

        let cause_ie = NGSetupFailureProtocolIEs_Entry {
            id: ProtocolIE_ID(ID_CAUSE),
            criticality: Criticality(Criticality::REJECT),
            value: FailureIE::Id_Cause(cause),
        };
        ies.push(cause_ie);

        if diag.is_some() {
            let diag_ie = NGSetupFailureProtocolIEs_Entry {
                id: ProtocolIE_ID(ID_CRITICALITY_DIAGNOSTICS),
                criticality: Criticality(Criticality::IGNORE),
                value: FailureIE::Id_CriticalityDiagnostics(diag.unwrap()),
            };
            ies.push(diag_ie);
        }

        let failure = UnsuccessfulOutcome {
            procedure_code: ProcedureCode(ID_NG_SETUP),
            criticality: Criticality(Criticality::REJECT),
            value: UnsuccessfulOutcomeValue::Id_NGSetup(NGSetupFailure {
                protocol_i_es: NGSetupFailureProtocolIEs(ies),
            }),
        };

        let pdu = NGAP_PDU::UnsuccessfulOutcome(failure);
        if let Err(e) = self.ngap_send_pdu(id, pdu).await {
            log::error!("Error in Sending NGSetupFailure. ({})", e);
            Err(e)
        } else {
            log::info!("NGSetupRequest Processing Failed for GNB");
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error NGSetup Processing Failure".to_string(),
            ))
        }
    }
}
