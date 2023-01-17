use std::net::SocketAddr;

use tokio::sync::mpsc::{Receiver, Sender};

use sctp_rs::{
    AssociationId, ConnectedSocket, Event, Notification, NotificationOrData, SendInfo,
    SubscribeEventAssocId,
};

use crate::messages::{NgapMgrToRanConnMessage, RanConnToNgapMgrMessage, ReceivedDataMessage};

const NGAP_SCTP_PPID: u32 = 60;

pub(crate) struct RanConnection {
    sock: ConnectedSocket,
    id: AssociationId,
    address: SocketAddr,
    ranconn_to_ngap_tx: Sender<RanConnToNgapMgrMessage>,
    ngap_to_ranconn_rx: Receiver<NgapMgrToRanConnMessage>,
}

impl RanConnection {
    pub(crate) fn new(
        id: AssociationId,
        sock: ConnectedSocket,
        address: SocketAddr,
        ranconn_to_ngap_tx: Sender<RanConnToNgapMgrMessage>,
        ngap_to_ranconn_rx: Receiver<NgapMgrToRanConnMessage>,
    ) -> Self {
        Self {
            sock,
            id,
            address,
            ranconn_to_ngap_tx,
            ngap_to_ranconn_rx,
        }
    }

    pub(crate) async fn handle_new_connection(mut self) -> std::io::Result<()> {
        log::info!("New Connection.");

        Self::init_new_connection(&self).await?;

        loop {
            tokio::select! {

                Some(msg) = self.ngap_to_ranconn_rx.recv() => {
                    log::debug!("RanConnection: Message: {:#?}", msg);
                    match msg {
                        NgapMgrToRanConnMessage::SendData(m) =>  {
                            self.sock.sctp_send(m.txdata).await?;
                        }
                        NgapMgrToRanConnMessage::Signal(_) => {
                            log::warn!("Signal Received. Closing RanConnection Task for {:#?}", self.address);
                            break Ok(());
                        }
                    }

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
                            if data.payload.is_empty() {
                                log::info!("Remote end '{}' closed connection.", self.address);
                                break Ok(());
                            } else {
                                let msg = RanConnToNgapMgrMessage::ReceivedData(
                                    ReceivedDataMessage {
                                        id: self.id,
                                        rxdata: data
                                });
                                let _ = self.ranconn_to_ngap_tx.send(msg).await;
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
