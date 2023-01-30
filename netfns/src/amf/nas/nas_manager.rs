//! NAS Manager
//!
//! The main NAS Manager thread. This is responsible for managing Network side NAS state for the
//! UEs.

use tokio::sync::mpsc::{Receiver, Sender};

use crate::amf::config::AmfConfig;
use crate::amf::messages::{AmfToNasMessage, NasToAmfMessage};

#[derive(Debug, Clone)]
pub(in crate::amf) struct NasManager {
    pub(crate) config: AmfConfig,
}

impl NasManager {
    pub(in crate::amf) fn from_config(config: AmfConfig) -> std::io::Result<Self> {
        Ok(Self { config })
    }

    pub(in crate::amf) async fn run(
        mut self,
        mut amf_to_nas_rx: Receiver<AmfToNasMessage>,
        _nas_to_amf_tx: Sender<NasToAmfMessage>,
    ) -> std::io::Result<()> {
        loop {
            tokio::select! {
                Some(_) = amf_to_nas_rx.recv() => {
                    log::debug!("Received Message from AMF");
                    break;
                }
            }
        }

        log::warn!("Closing NAS Manager Task!");
        Ok(())
    }
}
