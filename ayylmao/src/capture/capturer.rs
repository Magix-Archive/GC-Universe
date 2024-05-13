use std::sync::{Arc, Mutex};

use crossbeam_channel::Sender;
use log::{debug, info, warn};
use pcap::{Capture, Device, Linktype, Packet as PcapPacket};

use super::processor::{PacketProcessor, Packet};

const CONNECT_CMD: u32 = 0x00000145;
const DISCONNECT_CMD: u32 = 0x00000194;

pub struct Capturer {
    shutdown_hook: Arc<Mutex<bool>>,
    processor: PacketProcessor
}

impl Capturer {
    /// Creates a new instance of the `Capturer`.
    /// shutdown_hook: A flag to signal the packet capturing loop to stop.
    /// packet_callback: An event sender for when a packet is captured.
    pub fn new(
        shutdown_hook: Arc<Mutex<bool>>
    ) -> Self {
        Capturer {
            shutdown_hook,
            processor: PacketProcessor::new()
        }
    }

    /// Main loop for capturing packets.
    /// device: The device to capture packets from.
    pub fn run(&mut self, device: Device, tx: Sender<Packet>) {
        // Initialize the packet capturer.
        let mut capturer = Capture::from_device(device)
            .unwrap()
            .promisc(true)
            .timeout(1)
            .open()
            .unwrap();
        _ = capturer.filter("udp portrange 22101-22102", true);

        // Determine if the data link is Ethernet.
        let link = capturer.get_datalink();
        let is_ethernet = link.eq(&Linktype::ETHERNET);

        // Start capturing packets.
        while self.shutdown_hook.lock().unwrap().eq(&false) {
            if let Ok(packet) = capturer.next_packet() {
                self.parse_packet(packet, is_ethernet, &tx);
            }
        }

        // Save seeds.
        self.processor.bruteforce.save();
    }

    /// Parses a received packet.
    /// packet: The packet to parse.
    /// is_ethernet: Was the packet received via Ethernet?
    pub fn parse_packet(
        &mut self,
        packet: PcapPacket,
        is_ethernet: bool,
        tx: &Sender<Packet>
    ) {
        let data = packet.data.to_vec();

        let (data, port) = {
            // Remove Ethernet header.
            let data = if is_ethernet {
                (&data[14..]).to_vec()
            } else {
                data
            };
            // Determine the port from the packet.
            let port = u16::from_be_bytes([data[20], data[21]]);
            // Remove IPv4 header.
            let data = &data[20 + 8..];
            (Vec::from(data), port)
        };

        // Explaination: the server will send packets on one of these ports, never the client.
        let is_client = port != 22101 && port != 22102;

        // Check if the packet is a KCP handshake.
        // Handshake structure: https://github.com/KingRainbow44/Open-Shen/blob/stable/src/objects/handshake.ts
        if data.len() == 20 {
            let magic = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
            match magic {
                CONNECT_CMD => {
                    let conv = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
                    let token = u32::from_be_bytes([data[8], data[9], data[10], data[11]]);

                    self.processor.init(conv, token);

                    debug!("Received CONNECT_CMD! Conversation: {}, Token: {}", conv, token);
                },
                DISCONNECT_CMD => {
                    info!("{} disconnected.", if is_client { "Client" } else { "Server" })
                },
                _ => warn!("Unknown magic value: {:x?}", magic)
            };
        } else {
            // otherwise, Process the packet.
            self.processor.process(&data, is_client);
            let (client_packet, server_packet) = self.processor.receive_both();

            // Forward the packet to the handler.
            if let Some(packet) = client_packet {
                tx.send(packet).unwrap();
            }
            if let Some(packet) = server_packet {
                tx.send(packet).unwrap();
            }
        }
    }
}
