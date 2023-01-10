use std::collections::HashMap;

use asn1_codecs::{aper::AperCodec, PerCodecData};

use tokio::sync::mpsc::{self, Receiver, Sender};

use sctp_rs::{AssociationId, BindxFlags, Listener, Socket, SocketToAssociation};

use ngap::messages::r17::NGAP_PDU;

use crate::config::AmfConfig;
use crate::messages::{
    AmfToNgapMessage, NgapMgrToRanConnMessage, NgapToAmfMessage, RanConnToNgapMgrMessage,
    ReceivedDataMessage,
};

use super::ran_connection::RanConnection;

const NGAP_SCTP_PORT: u16 = 38412;

// NgapManager: Is a struct that connects the `NGAP` messages received from the `GNB` to the `AMF`
// processing. The encoding and decoding of the NGAP PDUs is performed by this structure.
//
// Whenever a new connection arrives on the listening socket, `NgapManager` spawns a task for
// processing the connection. A Map of 'AssociationId' -> 'Sender' (channel Sender) is maintained
// by the NgapManager. Whenever a message is received from the 'Amf', it will have a header
// containing the `AssociationID`, which determines the channel to be used  for sending the message
// to the 'GNB'. A message with `AssociationID` of '0' is a special control message. 'AMF' will use
// this ID for sending Control messages to 'NgapManager'. Such control messages can be used for
// performing graceful shutdown etc.
pub(crate) struct NgapManager {
    config: AmfConfig,
    socket: Listener,
    ran_connections: HashMap<AssociationId, Sender<NgapMgrToRanConnMessage>>,
}

impl NgapManager {
    pub(crate) fn from_config(config: AmfConfig) -> std::io::Result<Self> {
        let socket = Socket::new_v6(SocketToAssociation::OneToOne)?;

        let port = if config.ngap.port.is_some() {
            config.ngap.port.unwrap()
        } else {
            NGAP_SCTP_PORT
        };

        let mut bind_addrs = vec![];
        for addr in &config.ngap.addrs {
            let bind_addr = if addr.is_ipv6() {
                format!("[{}]:{}", addr, port).parse().unwrap()
            } else {
                format!("{}:{}", addr, port).parse().unwrap()
            };
            bind_addrs.push(bind_addr);
        }

        socket.sctp_bindx(&bind_addrs, BindxFlags::Add)?;

        // TODO: Make it configurable
        let socket = socket.listen(100)?;

        Ok(Self {
            config,
            socket,
            ran_connections: HashMap::new(),
        })
    }

    pub(crate) async fn run(
        mut self,
        mut amf_to_ngap_rx: Receiver<AmfToNgapMessage>,
        _ngap_to_amf_tx: Sender<NgapToAmfMessage>,
    ) -> std::io::Result<()> {
        let (tx, mut rx) = mpsc::channel::<RanConnToNgapMgrMessage>(10);
        let mut tasks = vec![];
        loop {
            tokio::select! {
                accepted = self.socket.accept() => {
                    let (accepted, client_addr) = accepted?;
                    let conn_status = accepted.sctp_get_status(0)?;

                    let (ngap_to_ranconn_tx, ngap_to_ranconn_rx) = mpsc::channel(10);

                    let gnb_connection = RanConnection::new(
                        conn_status.assoc_id,
                        accepted,
                        client_addr,
                        tx.clone(),
                        ngap_to_ranconn_rx
                    );

                    self.ran_connections.insert(conn_status.assoc_id, ngap_to_ranconn_tx);

                    log::info!(
                        "Spawning New Task for GNB: (Association:{}, ClientAddress: {}).",
                        conn_status.assoc_id,
                        client_addr
                    );
                    tasks.push(tokio::spawn(
                        RanConnection::handle_new_connection(gnb_connection))
                    );
                    log::debug!("spawned task!");
                }
                Some(RanConnToNgapMgrMessage::ReceivedData(
                        ReceivedDataMessage { id, rxdata}
                    )) = rx.recv() => {
                    let mut codec_data =
                    PerCodecData::from_slice_aper(&rxdata.payload);
                    let pdu = NGAP_PDU::aper_decode(&mut codec_data).unwrap();
                    match pdu {
                        NGAP_PDU::InitiatingMessage(init) => self.process_initiating_message(id, init),
                        NGAP_PDU::SuccessfulOutcome(success) => {
                            self.process_successful_outcome(id, success)
                        }
                        NGAP_PDU::UnsuccessfulOutcome(failure) => {
                            self.process_unsuccessful_outcome(id, failure)
                        }
                    }
                }
                Some(_amf_data) = amf_to_ngap_rx.recv() => {
                    log::debug!("Data Received from AMF.");
                    break ;
                }
            }
            log::debug!("select loop completed..");
        }

        futures::future::join_all(tasks).await;

        Ok(())
    }
}
