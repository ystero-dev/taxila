use sctp_rs::AssociationId;

use ngap::messages::r17::{NGSetupRequest, PLMNIdentity, SupportedTAList};

use crate::config::PlmnConfig;

use super::ngap_manager::NgapManager;

impl NgapManager {
    pub(super) fn process_ng_setup_request(&self, id: AssociationId, ngsetup: NGSetupRequest) {
        use ngap::messages::r17::NGSetupRequestProtocolIEs_EntryValue as IEValue;

        log::info!(
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
                IEValue::Id_DefaultPagingDRX(_paging_drx) => {
                    paging_drx_present = true;
                }
                IEValue::Id_Extended_RANNodeName(_ext_ran_node_name) => {}
                IEValue::Id_GlobalRANNodeID(_ran_node_id) => {
                    global_rannode_id_present = true;
                }
                IEValue::Id_NB_IoT_DefaultPagingDRX(_nb_iot_def_paging_drx) => {
                    log::warn!("Received unhandled NB_IOT Default Paging DRX");
                }
                IEValue::Id_RANNodeName(_ran_node_name) => {}
                IEValue::Id_SupportedTAList(supported_ta_list) => {
                    supported_ta_list_present = true;
                    ran_ta_supported = Self::any_ta_supported(
                        &supported_ta_list,
                        &self.config.plmn,
                        &self.config.tac,
                    );
                }
                IEValue::Id_UERetentionInformation(_ue_retention_info) => {
                    log::warn!("Received unhandled UE Retention Information");
                }
            }
        }
        if !global_rannode_id_present || !supported_ta_list_present || !paging_drx_present {
            log::error!("Missing Required Values, Sending Failure.");
        }

        if !ran_ta_supported {
            log::error!("None of the RAN TAs supported!");
        }
    }

    pub(crate) fn any_ta_supported(
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
                    log::trace!("Checking for PLMN Identity: {:?}", item);
                    if item.plmn_identity.0 == PLMNIdentity::from_mcc_mnc(plmn.mcc, plmn.mnc).0 {
                        log::trace!("Found: matching MCC({}): MNC({})", plmn.mcc, plmn.mnc);
                        return true;
                    }
                }
            }
        }
        false
    }
}
