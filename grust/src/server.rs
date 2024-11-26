use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;
use tokio::sync::RwLock;
use anyhow::Result;
use log::{debug, info};
use crate::session::NetworkSession;

/// This is the MTU of KCP.
const MAX_PACKET_SIZE: usize = 1400;

/// Inspiration taken from:
/// https://git.xeondev.com/reversedrooms/NaviaImpact/src/branch/master/gameserver/src/net/gateway.rs
pub struct Server {
    socket: Arc<UdpSocket>,
    sessions: Mutex<HashMap<u32, Arc<RwLock<NetworkSession>>>>
}

impl Server {
    /// Creates a new instance of the server.
    /// This also binds the server to the specified host and port.
    /// host: The host to bind to.
    /// port: The port to bind to.
    pub async fn new<S: Into<String>>(host: S, port: u16) -> Result<Server> {
        let bind = format!("{}:{}", host.into(), port);
        let socket = Arc::new(UdpSocket::bind(bind).await?);

        Ok(Server { socket, sessions: Mutex::new(HashMap::new()) })
    }

    /// Listens for incoming packets.
    /// This will block the current thread.
    pub async fn listen(&mut self) -> Result<()> {
        debug!("Listening for incoming packets...");

        let mut buffer = [0; MAX_PACKET_SIZE];
        loop {
            let Ok((size, address)) = self.socket.recv_from(&mut buffer).await else {
                continue;
            };

            info!("Received {} bytes from {}", size, address);
        }
    }
}
