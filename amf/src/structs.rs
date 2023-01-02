use std::net::IpAddr;
use std::sync::Arc;

use tokio::sync::Mutex;
use tokio::task::JoinHandle;

use serde::{Deserialize, Serialize};

use crate::ngap::NgapManager;

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
    ngap: Arc<Mutex<NgapManager>>,
}

impl Amf {
    pub fn from_config(config: &AmfConfig) -> std::io::Result<Self> {
        Ok(Self {
            ngap: Arc::new(Mutex::new(NgapManager::from_config(&config.ngap)?)),
        })
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        log::info!("Started");

        let ngap = Arc::clone(&self.ngap);
        let ngap_task: JoinHandle<_> = tokio::spawn(async move {
            let _ = NgapManager::run(ngap).await;
        });

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
