//! NAS Manager
//!
//! The main NAS Manager thread. This is responsible for managing Network side NAS state for the
//! UEs.

use tokio::sync::mpsc::{Receiver, Sender};

use nas::messages::{
    headers::NasMessageHeader, RegistrationRequest, MM_MSG_TYPE_REGISTRATION_REQUEST,
};

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
                Some(msg) = amf_to_nas_rx.recv() => {
                    match msg {
                        AmfToNasMessage::Signal(_) => {
                            log::debug!("Received Signal Message from AMF");
                            break;
                        }
                        AmfToNasMessage::NasPduMessage(msg) => {
                            log::debug!("received NAS PDU Message from AMF: {:#?}", msg);
                            let (header, decoded) = NasMessageHeader::decode(&msg.pdu.0)?;
                            if let NasMessageHeader::Mm(header) = header {
                                if header.message_type == MM_MSG_TYPE_REGISTRATION_REQUEST {
                                    let (reg_request, decoded) = RegistrationRequest::decode(&msg.pdu.0)?;
                                    log::debug!("Reg Request: {:#?}", reg_request);
                                }
                            }
                        }
                    }
                }
            }
        }

        log::warn!("Closing NAS Manager Task!");
        Ok(())
    }
}
