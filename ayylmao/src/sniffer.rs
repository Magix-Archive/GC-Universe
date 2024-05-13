use std::sync::Arc;

use crossbeam_channel::Receiver;
use futures_util::SinkExt;
use log::{error, warn};
use serde::Serialize;
use tokio::sync::Mutex;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;


use crate::{capture::processor::Packet, options::Options};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct EncodedPacket {
    time: u64,
    source: String, // can be: client, server
    packet_id: u16,
    packet_name: String,
    length: usize,
    data: String, // JSON-encoded protobuf data
}

impl From<Packet> for EncodedPacket {
    fn from(packet: Packet) -> Self {
        // sent_time is from the packet's header
        let sent_time = 0;
        // packet_name needs to be looked up from the id
        let packet_name = "unknown".to_string();
        // parsed_data is the result of protobuf decoding and json encoding
        let parsed_data = "{}".to_string();

        EncodedPacket {
            time: sent_time,
            source: if packet.is_client { "client" } else { "server" }.to_string(),
            packet_id: packet.id,
            packet_name: packet_name,
            length: packet.data.len(),
            data: parsed_data
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PacketMessage {
    packet_id: u32,
    data: EncodedPacket
}

/// Packet handler for sniffed packets.
/// Forwards all packets to a websocket server.
pub async fn run(app: Options, hook: Arc<std::sync::Mutex<bool>>, rx: Receiver<Packet>) {
    // Prepare the websocket server.
    let addr = format!("{}:{}", app.sniffer.bind_address, app.sniffer.bind_port);
    let socket = TcpListener::bind(&addr).await;
    if let Err(error) = socket {
        error!("Failed to bind to {}: {}", addr, error);
        return;
    }
    let socket = socket.unwrap();

    // Initialize the clients list.
    let clients = Arc::new(Mutex::new(Vec::new()));

    // Start the websocket server loop.
    let add_clients = clients.clone();
    tokio::spawn(async move {
        while let Ok((stream, _)) = socket.accept().await {
            // Accept the websocket connection.
            let ws_stream = match tokio_tungstenite::accept_async(stream).await {
                Ok(ws_stream) => ws_stream,
                Err(error) => {
                    warn!("Failed to accept websocket connection: {}", error);
                    continue;
                }
            };

            let mut clients = add_clients.lock().await;
            clients.push(ws_stream);
        }
    });

    // Start the packet loop.
    while hook.lock().unwrap().eq(&false) {
        if let Ok(packet) = rx.recv() {
            // Encode the packet.
            let packet = serde_json::to_string(&PacketMessage {
                packet_id: packet.id as _, data: packet.into()
            }).unwrap();

            // Forward the packet to the websocket server.
            let mut clients = clients.lock().await;
            for client in clients.iter_mut() {
                _ = client.send(Message::Text(packet.clone())).await;
            }
        }
    }
}
