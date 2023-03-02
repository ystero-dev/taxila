//! NAS Manager
//!
//! The main NAS Manager thread. This is responsible for managing Network side NAS state for the
//! UEs.

use tokio::sync::mpsc::{Receiver, Sender};

use ngap::messages::r17::NAS_PDU;

use nas::messages::{
    headers::{ExtProtoDiscriminator, Nas5gMmMessageHeader, Nas5gSecurityHeader},
    RegistrationRequest, MM_MSG_TYPE_REGISTRATION_REQUEST,
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
                            // First Octet is Extended Protocol Identity, Use it to call
                            // appropriate function to decode (and handle) the rest of the message.
                            let ext_proto_disc = msg.pdu.0[0].into();
                            log::debug!("received NAS PDU Message from AMF: {:?}", ext_proto_disc);
                            match ext_proto_disc {
                                ExtProtoDiscriminator::FivegNasMobilityManagementType => {
                                    self.decode_nas_mm_message(msg.pdu)?;
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

    fn decode_nas_mm_message(&self, nas_pdu: NAS_PDU) -> std::io::Result<()> {
        let (header, decoded) = Nas5gMmMessageHeader::decode(&nas_pdu.0)?;

        match header.sec_header_type {
            Nas5gSecurityHeader::PlainText => {
                Self::decode_nas_message(&header, nas_pdu)?;
            }
            _ => todo!(),
        }

        Ok(())
    }

    // This is a decrypted message, which will be decoded and right now only printed. Eventually,
    // this is the message type that will be returned by this function.
    fn decode_nas_message(header: &Nas5gMmMessageHeader, nas_pdu: NAS_PDU) -> std::io::Result<()> {
        if header.message_type == MM_MSG_TYPE_REGISTRATION_REQUEST {
            let (reg_request, decoded) = RegistrationRequest::decode(&nas_pdu.0)?;
            log::debug!("Reg Request: {:#?}", reg_request);
        }
        Ok(())
    }
}
