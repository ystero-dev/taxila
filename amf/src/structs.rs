use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::ngap::NgapManager;

pub(crate) struct AmfToNgapMsg;

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

        let (_amf_to_ngap_tx, amf_to_ngap_rx) = mpsc::channel(10);
        let ngap = NgapManager::from_config(&self.config.ngap)?;
        let ngap_task = tokio::spawn(NgapManager::run(ngap, amf_to_ngap_rx));
        let _ = ngap_task.await;

        Ok(())
    }
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
