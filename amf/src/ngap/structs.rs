use std::collections::HashMap;
use std::net::SocketAddr;

use asn1_codecs::{aper::AperCodec, PerCodecData};

use tokio::sync::mpsc::{self, Receiver, Sender};

use sctp_rs::{
    AssociationId, BindxFlags, ConnectedSocket, Event, Listener, Notification, NotificationOrData,
    SendInfo, Socket, SocketToAssociation, SubscribeEventAssocId,
};

use ngap::messages::r17::NGAP_PDU;

use crate::messages::{
    AmfToNgapMessage, GnbToNgapMessage, NgapToAmfMessage, NgapToGnbMessage, PDUMessage,
    ReceivedDataMessage,
};

const NGAP_SCTP_PORT: u16 = 38412;
const NGAP_SCTP_PPID: u32 = 60;

struct GnbConnection {
    sock: ConnectedSocket,
    id: AssociationId,
    address: SocketAddr,
    gnb_to_ngap_tx: Sender<GnbToNgapMessage>,
    ngap_to_gnb_rx: Receiver<NgapToGnbMessage>,
}

impl GnbConnection {
    async fn handle_new_connection(mut self) -> std::io::Result<()> {
        log::info!("New Connection.");

        Self::init_new_connection(&self).await?;

        loop {
            tokio::select! {

                _ = self.ngap_to_gnb_rx.recv() => {

                }

                received = self.sock.sctp_recv() => {
                    let received = received?;
                    match received {
                        NotificationOrData::Notification(notification) => match notification {
                            Notification::Shutdown(_) => {
                                log::info!("Shutdown Event Received for GNB: {}", self.address);
                                break Ok(());
                            }
                            _ => {
                                log::debug!("Received Notification: {:#?}", notification);
                            }
                        },
                        NotificationOrData::Data(data) => {
                            log::debug!("Received Data: {:#?}", data);
                            if data.payload.len() == 0 {
                                log::info!("Remote end '{}' closed connection.", self.address);
                                break Ok(());
                            } else {
                                let msg = GnbToNgapMessage::ReceivedData(
                                    ReceivedDataMessage {
                                        id: self.id,
                                        rxdata: data
                                });
                                let _ = self.gnb_to_ngap_tx.send(msg).await;
                            }
                        }
                    }
                }
            }
        }
    }

    async fn init_new_connection(&self) -> std::io::Result<()> {
        let send_info = SendInfo {
            sid: 0, // Always use 'Stream ID' of '0' for the Non-UE signaling.
            ppid: NGAP_SCTP_PPID,
            flags: 0,
            assoc_id: 0,
            context: 0, // TODO: Use context later
        };
        log::debug!(
            "Setting Default SendInfo: sid: {}, ppid: {}",
            send_info.sid,
            send_info.ppid
        );

        let events = &[Event::Association, Event::Shutdown];
        let subscribe_assoc_id = SubscribeEventAssocId::All;
        self.sock
            .sctp_subscribe_events(events, subscribe_assoc_id)?;
        self.sock.sctp_request_rcvinfo(true)?;
        self.sock.sctp_set_default_sendinfo(send_info)
    }
}

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
    socket: Listener,
    gnb_connections: HashMap<AssociationId, Sender<NgapToGnbMessage>>,
}

impl NgapManager {
    pub(crate) fn from_config(config: &crate::amf::structs::NgapConfig) -> std::io::Result<Self> {
        let socket = Socket::new_v6(SocketToAssociation::OneToOne)?;

        let port = if config.port.is_some() {
            config.port.unwrap()
        } else {
            NGAP_SCTP_PORT
        };

        let mut bind_addrs = vec![];
        for addr in &config.addrs {
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
            socket,
            gnb_connections: HashMap::new(),
        })
    }

    pub(crate) async fn run(
        mut self,
        mut amf_to_ngap_rx: Receiver<AmfToNgapMessage>,
        mut ngap_to_amf_tx: Sender<NgapToAmfMessage>,
    ) -> std::io::Result<()> {
        let (tx, mut rx) = mpsc::channel::<GnbToNgapMessage>(10);
        let mut tasks = vec![];
        loop {
            tokio::select! {
                accepted = self.socket.accept() => {
                    let (accepted, client_addr) = accepted?;
                    let conn_status = accepted.sctp_get_status(0)?;

                    let (ngap_to_gnb_tx, ngap_to_gnb_rx) = mpsc::channel(10);

                    let gnb_connection = GnbConnection {
                        id: conn_status.assoc_id,
                        sock: accepted,
                        address: client_addr,
                        gnb_to_ngap_tx: tx.clone(),
                        ngap_to_gnb_rx
                    };

                    self.gnb_connections.insert(conn_status.assoc_id, ngap_to_gnb_tx);

                    log::info!(
                        "Spawning New Task for GNB: (Association:{}, ClientAddress: {}).",
                        conn_status.assoc_id,
                        client_addr
                    );
                    tasks.push(tokio::spawn(
                        GnbConnection::handle_new_connection(gnb_connection))
                    );
                    log::debug!("spawned task!");
                }
                Some(GnbToNgapMessage::ReceivedData(
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
