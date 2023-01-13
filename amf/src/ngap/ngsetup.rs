use sctp_rs::{AssociationId, SendData};

use asn1_codecs::{aper::AperCodec, PerCodecData};

use ngap::messages::r17::{
    AMFName, Cause, CauseMisc, CauseProtocol, Criticality, CriticalityDiagnostics, NGSetupFailure,
    NGSetupFailureProtocolIEs, NGSetupFailureProtocolIEs_Entry,
    NGSetupFailureProtocolIEs_EntryValue as FailureIE, NGSetupRequest,
    NGSetupRequestProtocolIEs_EntryValue as RequestIEValue, NGSetupResponse,
    NGSetupResponseProtocolIEs, NGSetupResponseProtocolIEs_Entry,
    NGSetupResponseProtocolIEs_EntryValue as ResponseIEValue, PLMNIdentity, PLMNSupportItem,
    PLMNSupportList, ProcedureCode, ProtocolIE_ID, RelativeAMFCapacity, ServedGUAMIItem,
    ServedGUAMIList, SliceSupportItem, SliceSupportList, SuccessfulOutcome, SuccessfulOutcomeValue,
    SupportedTAList, UnsuccessfulOutcome, UnsuccessfulOutcomeValue, GUAMI, NGAP_PDU, SST, S_NSSAI,
};

use crate::config::PlmnConfig;
use crate::messages::{NgapMgrToRanConnMessage, SendDataMessage};

use super::ngap_manager::NgapManager;

impl NgapManager {
    pub(super) async fn process_ng_setup_request(
        &self,
        id: AssociationId,
        ngsetup: NGSetupRequest,
    ) {
        log::debug!(
            "Received from AssociationID: {}, NGSetupRequest: {:#?}",
            id,
            ngsetup,
        );

        let mut global_rannode_id_present = false;
        let mut supported_ta_list_present = false;
        let mut paging_drx_present = false;
        let mut ran_ta_supported = false;

        for ie in ngsetup.protocol_i_es.0 {
            match ie.value {
                RequestIEValue::Id_DefaultPagingDRX(_paging_drx) => {
                    paging_drx_present = true;
                }
                RequestIEValue::Id_Extended_RANNodeName(_ext_ran_node_name) => {}
                RequestIEValue::Id_GlobalRANNodeID(_ran_node_id) => {
                    global_rannode_id_present = true;
                }
                RequestIEValue::Id_NB_IoT_DefaultPagingDRX(_nb_iot_def_paging_drx) => {
                    log::warn!("Received unhandled NB_IOT Default Paging DRX");
                }
                RequestIEValue::Id_RANNodeName(_ran_node_name) => {}
                RequestIEValue::Id_SupportedTAList(supported_ta_list) => {
                    supported_ta_list_present = true;
                    ran_ta_supported = Self::any_tas_supported(
                        &supported_ta_list,
                        &self.config.plmn,
                        &self.config.tac,
                    );
                }
                RequestIEValue::Id_UERetentionInformation(_ue_retention_info) => {
                    log::warn!("Received unhandled UE Retention Information");
                }
            }
        }
        if !global_rannode_id_present || !supported_ta_list_present {
            log::error!("Missing Mandatory IEs with Criticality Reject, Sending Failure.");
            self.send_ngsetup_failure(
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
            self.send_ngsetup_failure(
                id,
                Cause::Misc(CauseMisc(CauseMisc::UNKNOWN_PLMN_OR_SNPN)),
                None,
            )
            .await;
        }

        self.send_ngsetup_success(id).await;
    }

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

        false
    }

    async fn send_ngsetup_success(&self, id: AssociationId) {
        // Prepare the NGSetup Success
        //
        // IEs first
        let mut ies = vec![];

        // AMF Name
        let amf_name_ie = NGSetupResponseProtocolIEs_Entry {
            id: ProtocolIE_ID(1), // TODO: ID_AMF_NAME when the const is made public
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
            id: ProtocolIE_ID(96), // TODO: ID_GUAMI when the const is available.
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
            id: ProtocolIE_ID(80), // TODO: ID_PLMN_SUPPORT_LIST when the const is available.
            criticality: Criticality(Criticality::REJECT),
            value: ResponseIEValue::Id_PLMNSupportList(PLMNSupportList(plmns)),
        };
        ies.push(plmn_support_ie);

        // Relative AMF Capacity
        let relative_amf_capacity_ie = NGSetupResponseProtocolIEs_Entry {
            id: ProtocolIE_ID(86), // TODO: ID_RELATIVE_AMF_CAPACITY when the const is available.
            criticality: Criticality(Criticality::REJECT),
            value: ResponseIEValue::Id_RelativeAMFCapacity(RelativeAMFCapacity(255)),
        };
        ies.push(relative_amf_capacity_ie);

        let response = SuccessfulOutcome {
            procedure_code: ProcedureCode(21),
            criticality: Criticality(Criticality::REJECT),
            value: SuccessfulOutcomeValue::Id_NGSetup(NGSetupResponse {
                protocol_i_es: NGSetupResponseProtocolIEs(ies),
            }),
        };

        let pdu = NGAP_PDU::SuccessfulOutcome(response);
        log::debug!("Response: {:#?}", pdu);

        let mut codec_data = PerCodecData::new_aper();
        let result = pdu.aper_encode(&mut codec_data); // TODO: Handle error
        log::debug!("Result: encode: {:#?}", result);
        let data = codec_data.get_inner().unwrap();

        let senddata = NgapMgrToRanConnMessage::SendData(SendDataMessage {
            txdata: SendData {
                payload: data,
                snd_info: None,
            },
            id: id,
        });

        let tx = self.ran_connections.get(&id).unwrap();
        // TODO : Handle Error.
        let _ = tx.send(senddata).await;
    }

    async fn send_ngsetup_failure(
        &self,
        id: AssociationId,
        cause: Cause,
        diag: Option<CriticalityDiagnostics>,
    ) {
        let mut ies = vec![];

        let cause_ie = NGSetupFailureProtocolIEs_Entry {
            id: ProtocolIE_ID(15),
            criticality: Criticality(Criticality::REJECT),
            value: FailureIE::Id_Cause(cause),
        };
        ies.push(cause_ie);

        if diag.is_some() {
            let diag_ie = NGSetupFailureProtocolIEs_Entry {
                id: ProtocolIE_ID(19),
                criticality: Criticality(Criticality::IGNORE),
                value: FailureIE::Id_CriticalityDiagnostics(diag.unwrap()),
            };
            ies.push(diag_ie);
        }

        let failure = UnsuccessfulOutcome {
            procedure_code: ProcedureCode(21),
            criticality: Criticality(Criticality::REJECT),
            value: UnsuccessfulOutcomeValue::Id_NGSetup(NGSetupFailure {
                protocol_i_es: NGSetupFailureProtocolIEs(ies),
            }),
        };

        let pdu = NGAP_PDU::UnsuccessfulOutcome(failure);
        log::debug!("Response: {:#?}", pdu);

        let mut codec_data = PerCodecData::new_aper();
        let result = pdu.aper_encode(&mut codec_data); // TODO: Handle error
        log::debug!("Result: encode: {:#?}", result);
        let data = codec_data.get_inner().unwrap();

        let senddata = NgapMgrToRanConnMessage::SendData(SendDataMessage {
            txdata: SendData {
                payload: data,
                snd_info: None,
            },
            id: id,
        });

        let tx = self.ran_connections.get(&id).unwrap();
        // TODO : Handle Error.
        let _ = tx.send(senddata).await;
    }
}
