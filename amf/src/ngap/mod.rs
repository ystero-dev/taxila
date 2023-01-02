pub(crate) mod messages;

use std::net::SocketAddr;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

use tokio::sync::Mutex;

use sctp_rs::{BindxFlags, ConnectedSocket, Listener, SendInfo, Socket, SocketToAssociation};

const NGAP_SCTP_PORT: u16 = 38412;
const NGAP_SCTP_PPID: u32 = 60;

pub struct Gnb {
    sock: ConnectedSocket,
    _address: SocketAddr,
}

impl Gnb {
    async fn handle_new_connection(me: Arc<Mutex<Self>>) -> std::io::Result<()> {
        // This block is required because the `MutexGuard` below otherwise would only be dropped
        // 'after' the loop (that is never).
        {
            let gnb = me.lock().await;
            let send_info = SendInfo {
                sid: 0, // Always use 'Stream ID' of '0' for the Non-UE signaling.
                ppid: NGAP_SCTP_PPID,
                flags: 0,
                assoc_id: 0,
                context: 0, // TODO: Use context later
            };
            (*gnb).sock.sctp_set_default_sendinfo(send_info)?;
        }

        loop {}
    }
}

pub struct NgapManager {
    socket: Listener,
    peers: Vec<Arc<Mutex<Gnb>>>,
    should_stop: AtomicBool,
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
            should_stop: AtomicBool::new(false),
        })
    }

    pub async fn run(me: Arc<Mutex<Self>>) -> std::io::Result<()> {
        loop {
            let mut ngap = me.lock().await;
            if *(*ngap).should_stop.get_mut() {
                break;
            }
            let result =
                tokio::time::timeout(Duration::from_millis(2000), (*ngap).socket.accept()).await;
            match result {
                Ok(result) => {
                    let (accepted, client_addr) = result?;
                    let gnb = Arc::new(Mutex::new(Gnb {
                        sock: accepted,
                        _address: client_addr,
                    }));

                    (*ngap).peers.push(Arc::clone(&gnb));

                    // Accepted a Socket, this is always from one gNB.
                    // TODO: Join on this task?
                    tokio::task::spawn(async move {
                        // TODO: Not sure what to do with the error?
                        let _ = Gnb::handle_new_connection(gnb).await;
                    });
                }
                _ => {
                    log::trace!("Elapsed timeout of 2 sec and no data.");
                }
            }
        }

        Ok(())
    }

    pub(crate) fn stop(&mut self) {
        self.should_stop.store(false, Ordering::Relaxed);
    }
}
