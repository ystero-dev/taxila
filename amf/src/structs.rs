use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use sctp_rs::AssociationId;

use crate::ngap::messages::r17::{
    InitiatingMessage, InitiatingMessageValue, NGSetupRequest, SuccessfulOutcome,
    UnsuccessfulOutcome, NGAP_PDU,
};
use crate::ngap::NgapManager;

use crate::messages::{NgapToAmfMessage, PDUMessage};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct NgapConfig {
    pub addrs: Vec<IpAddr>,
    pub port: Option<u16>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AmfConfig {
    pub ngap: NgapConfig,
}

pub struct Amf {
    config: AmfConfig,
}

impl Amf {
    pub fn from_config(config: AmfConfig) -> std::io::Result<Self> {
        Ok(Self { config })
    }

    pub async fn run(self) -> std::io::Result<()> {
        log::info!("Started AMF");

        let (amf_to_ngap_tx, amf_to_ngap_rx) = mpsc::channel(10);
        let (ngap_to_amf_tx, mut ngap_to_amf_rx) = mpsc::channel::<NgapToAmfMessage>(10);

        let ngap = NgapManager::from_config(&self.config.ngap)?;
        let ngap_task = tokio::spawn(NgapManager::run(ngap, amf_to_ngap_rx, ngap_to_amf_tx));

        loop {
            tokio::select! {
                Some(msg) = ngap_to_amf_rx.recv() => {
                    match msg {
                        NgapToAmfMessage::PDU(pdu) => self.process_ngap_pdu_message(pdu),
                    }
                }
            }
        }

        let _ = ngap_task.await;

        Ok(())
    }
}

impl Amf {
    fn process_ngap_pdu_message(&self, msg: PDUMessage) {
        match msg.pdu {
            NGAP_PDU::InitiatingMessage(init) => self.process_initiating_message(msg.id, init),
            NGAP_PDU::SuccessfulOutcome(success) => {
                self.process_successful_outcome(msg.id, success)
            }
            NGAP_PDU::UnsuccessfulOutcome(failure) => {
                self.process_unsuccessful_outcome(msg.id, failure)
            }
        }
    }

    fn process_initiating_message(&self, id: AssociationId, init: InitiatingMessage) {
        match init.value {
            InitiatingMessageValue::Id_NGSetup(ng_setup_req) => {
                self.process_ng_setup_request(id, ng_setup_req)
            }
            _ => (),
        }
    }

    fn process_ng_setup_request(&self, id: AssociationId, ngsetup: NGSetupRequest) {
        log::info!(
            "Received from AssociationID: {}, NGSetupRequest: {:#?}",
            id,
            ngsetup,
        );
    }

    fn process_successful_outcome(&self, id: AssociationId, success: SuccessfulOutcome) {}
    fn process_unsuccessful_outcome(&self, id: AssociationId, failure: UnsuccessfulOutcome) {}
}

#[cfg(test)]
mod tests {

    #[test]
    fn works() {
        let config_str = "ngap:\n addrs:\n - 127.0.0.1 \n - ::1 \nport: 38413";
        let amf_config: Result<crate::structs::AmfConfig, _> = serde_yaml::from_str(config_str);
        assert!(amf_config.is_ok(), "{:#?}", amf_config.err().unwrap());
    }
}
