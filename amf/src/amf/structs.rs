use tokio::sync::mpsc;

use crate::config::AmfConfig;
use crate::messages::NgapToAmfMessage;
use crate::ngap::ngap_manager::NgapManager;

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
        let (ngap_to_amf_tx, mut ngap_to_amf_rx) = mpsc::channel::<NgapToAmfMessage>(10);

        let ngap = NgapManager::from_config(self.config.clone())?;
        let _ = tokio::spawn(NgapManager::run(ngap, amf_to_ngap_rx, ngap_to_amf_tx));

        loop {
            tokio::select! {
                Some(_) = ngap_to_amf_rx.recv() => {
                }
            }
        }
    }
}
