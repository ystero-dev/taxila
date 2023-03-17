//! NAS Manager
//!
//! The main NAS Manager thread. This is responsible for managing Network side NAS state for the
//! UEs.

use std::collections::HashMap;

use tokio::sync::mpsc::{Receiver, Sender};

use nas::messages::headers::ExtProtoDiscriminator;

use crate::amf::config::AmfConfig;
use crate::amf::messages::{AmfToNasMessage, NasPduMessage, NasToAmfMessage};

use super::amf_ue::AmfUe;

#[derive(Debug, Clone)]
pub(in crate::amf) struct NasManager {
    pub(crate) config: AmfConfig,
    pub(crate) amf_ues: HashMap<u64, AmfUe>,
}

impl NasManager {
    pub(in crate::amf) fn from_config(config: AmfConfig) -> std::io::Result<Self> {
        Ok(Self {
            config,
            amf_ues: HashMap::new(),
        })
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
                            // First Octet is Extended Protocol Identity, Use it to call
                            // appropriate function to decode (and handle) the rest of the message.
                            let ext_proto_disc = msg.pdu.0[0].into();
                            log::debug!("received NAS PDU Message from AMF: {:?}", ext_proto_disc);
                            match ext_proto_disc {
                                ExtProtoDiscriminator::FivegNasMobilityManagementType => {
                                    self.handle_nas_mm_message(msg)?;
                                }
                                _ => todo!(),
                            }
                        }
                    }
                }
            }
        }

        log::warn!("Closing NAS Manager Task!");
        Ok(())
    }

    // Decode the received NAS Message. The received NAS message may be a plain-text message or an
    // integrity protected and/or ciphered message.
    fn handle_nas_mm_message(&mut self, msg: NasPduMessage) -> std::io::Result<()> {
        if msg.initial_ue {
            // First get the `AmfUe` for the given `id`.
            let amf_ue = self.amf_ues.get(&msg.id);
            if amf_ue.is_some() {
                // initial UE Message and we still have an `AmfUe` Entry somewhere? Right now just
                // log a warning and remove this entry!
                log::warn!("Initial UE Message and exisitng `AmfUe`. Deleting it...");
                let _ = self.amf_ues.remove_entry(&msg.id);
            }
            self.amf_ues.insert(msg.id, AmfUe::new_amf_ue(msg.id));
        };
        // Get the AMF UE corresponding to the `amf_ngap_ue_id`.
        let amf_ue = self.amf_ues.get_mut(&msg.id);

        if amf_ue.is_none() {
            log::error!(
                "Unable to find AMF UE corresponding to AMF_NGAP_UE_ID: {}",
                msg.id
            );
        }

        let mut amf_ue = amf_ue.unwrap();

        if msg.initial_ue {
            amf_ue.handle_initial_mm_message(msg.pdu)
        } else {
            amf_ue.handle_mm_message(msg.pdu)
        }
    }
}
