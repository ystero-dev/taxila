use sctp_rs::AssociationId;

use crate::ngap::messages::r17::NGSetupRequest;

use super::structs::Amf;

impl Amf {
    pub(super) fn process_ng_setup_request(&self, id: AssociationId, ngsetup: NGSetupRequest) {
        use crate::ngap::messages::r17::NGSetupRequestProtocolIEs_EntryValue as IEValue;

        log::info!(
            "Received from AssociationID: {}, NGSetupRequest: {:#?}",
            id,
            ngsetup,
        );

        let mut global_rannode_id_present = false;
        let mut supported_ta_list_present = false;
        let mut paging_drx_present = false;

        for ie in ngsetup.protocol_i_es.0 {
            match ie.value {
                IEValue::Id_DefaultPagingDRX(paging_drx) => {
                    paging_drx_present = true;
                }
                IEValue::Id_Extended_RANNodeName(_ext_ran_node_name) => {}
                IEValue::Id_GlobalRANNodeID(ran_node_id) => {
                    global_rannode_id_present = true;
                }
                IEValue::Id_NB_IoT_DefaultPagingDRX(_nb_iot_def_paging_drx) => {
                    log::warn!("Received unhandled NB_IOT Default Paging DRX");
                }
                IEValue::Id_RANNodeName(_ran_node_name) => {}
                IEValue::Id_SupportedTAList(supported_ta_list) => {
                    supported_ta_list_present = true;
                }
                IEValue::Id_UERetentionInformation(_ue_retention_info) => {
                    log::warn!("Received unhandled UE Retention Information");
                }
            }
        }
    }
}
