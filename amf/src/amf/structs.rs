use std::net::IpAddr;

use serde::{
    de::{Deserializer, Error},
    Deserialize, Serialize,
};
use tokio::sync::mpsc;

use ngap::messages::r17::NGAP_PDU;

use crate::ngap::structs::NgapManager;

use crate::messages::{NgapToAmfMessage, PDUMessage};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct AmfIdConfig {
    pub pointer: u8,
    pub set: u16,
    pub region: u8,
}

impl<'de> Deserialize<'de> for AmfIdConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        // Pointer can at the most be 6 bits.
        if this.pointer > 63 {
            return Err(D::Error::custom("Max supported value for `pointer` is 63."));
        }

        // Set can be at the most 10 bits.
        if this.set > 1024 {
            return Err(D::Error::custom("Max supported value for `pointer` is 63."));
        }

        Ok(this)
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(remote = "Self")]
pub struct PlmnConfig {
    pub mcc: u16,
    pub mnc: u16,
}

impl<'de> Deserialize<'de> for PlmnConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        if this.mcc > 999 || this.mnc > 999 {
            return Err(D::Error::custom(
                "Max supported value for `mcc` and `mnc` is 999.",
            ));
        }

        Ok(this)
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct NgapConfig {
    pub addrs: Vec<IpAddr>,
    pub port: Option<u16>,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct AmfConfig {
    pub ngap: NgapConfig,
    pub plmn: PlmnConfig,
    pub tac: Vec<u32>, // TODO: Validate Max value is 24 bit.
    pub amf_id: AmfIdConfig,
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
    fn process_ngap_pdu_message(&self, _msg: PDUMessage) {}
}

#[cfg(test)]
mod tests {

    #[test]
    fn works() {
        let config_str =
            "ngap:\n addrs:\n - 127.0.0.1 \n - ::1 \nport: 38413\nplmn:\n mcc: 999\n mnc: 99\ntac: [ 1, 2, 3]\namf_id:\n pointer: 63\n set: 10\n region: 1\n";
        let amf_config: Result<crate::amf::structs::AmfConfig, _> =
            serde_yaml::from_str(config_str);
        assert!(amf_config.is_ok(), "{:#?}", amf_config.err().unwrap());
    }
}
