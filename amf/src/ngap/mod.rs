pub(crate) mod messages;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use sctp_rs::{BindxFlags, ConnectedSocket, Listener, Socket, SocketToAssociation};

const NGAP_SCTP_PORT: u16 = 38412;
const NGAP_SCTP_PPID: u32 = 60;

pub struct Gnb {}

impl Gnb {
    async fn handle_new_connection(me: Arc<Mutex<Self>>, sock: ConnectedSocket, peer: SocketAddr) {}
}

pub struct NgapManager {
    socket: Listener,
    peers: Vec<Arc<Mutex<Gnb>>>,
}

impl NgapManager {
    pub fn from_config(config: &crate::structs::NgapConfig) -> std::io::Result<Self> {
        let socket = Socket::new_v6(SocketToAssociation::OneToOne)?;

        let port = if config.port.is_some() {
            config.port.unwrap()
        } else {
            NGAP_SCTP_PORT
        };

        let mut bind_addrs = vec![];
        for addr in &config.addrs {
            let bind_addr = format!("{}:{}", addr, port).parse().unwrap();
            bind_addrs.push(bind_addr);
        }

        socket.sctp_bindx(&bind_addrs, BindxFlags::Add)?;

        // TODO: Make it configurable
        let socket = socket.listen(100)?;

        Ok(Self {
            socket,
            peers: vec![],
        })
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        loop {
            let (accepted, client_addr) = self.socket.accept().await?;

            let gnb = Arc::new(Mutex::new(Gnb {}));

            self.peers.push(Arc::clone(&gnb));

            // Accepted a Socket, this is always from one gNB.
            tokio::task::spawn(async move {
                Gnb::handle_new_connection(gnb, accepted, client_addr).await;
            });
        }
    }
}
