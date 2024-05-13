use std::{io::Write, time::{SystemTime, UNIX_EPOCH}};

use base64::engine::{Engine, general_purpose::STANDARD};
use kcp::Kcp;
use log::{error, warn};
use rsa::{pkcs1::DecodeRsaPrivateKey, Pkcs1v15Encrypt, RsaPrivateKey};

use crate::keys::{self, Key};

use super::bruteforce::Bruteforce;

/// Utility function to create a new KCP instance.
/// conv: The KCP conversation ID.
/// token: The KCP token.
fn new_kcp(conv: u32, token: u32) -> Kcp<Writer> {
    let mut kcp = Kcp::new(conv, token, Writer);
    kcp.set_nodelay(true, 10, 2, false);
    kcp.set_wndsize(256, 256);

    kcp
}

/// Validates the packet data.
/// +---------+---------+-------------------+-------------+------------------+------------------+---------+
/// |  Magic  |  CmdId  | PacketHead Length | Data Length | PacketHead bytes |    Data bytes    |  Magic  |
/// +---------+---------+-------------------+-------------+------------------+------------------+---------+
/// | 2 bytes | 2 bytes | 2 bytes           | 4 bytes     | size = 3rd field | size = 4th field | 2 bytes |
/// +---------+---------+-------------------+-------------+------------------+------------------+---------+
/// data: The packet data.
fn is_valid(data: &[u8]) -> bool {
    if data.len() <= 2 {
        data[0] == 0x45 && data[1] == 0x67
    } else {
        data[0] == 0x45
            && data[1] == 0x67
            && data[data.len() - 2] == 0x89
            && data[data.len() - 1] == 0xAB
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: u16,
    pub header: Vec<u8>,
    pub data: Vec<u8>,

    pub is_client: bool
}

pub enum PacketKey {
    Dispatch(Key),
    Session(Key),
}

pub struct PacketProcessor {
    client: Option<Kcp<Writer>>,
    server: Option<Kcp<Writer>>,
    decryption_key: Option<PacketKey>,

    // Used in brute-forcing the decryption key.
    rsa_key: RsaPrivateKey,
    token_rsp: Option<(u64, u64)>,
    pub bruteforce: Bruteforce
}

impl PacketProcessor {
    /// Creates a new instance of the `PacketProcessor`.
    pub fn new() -> Self {
        // Parse the RSA key.
        let rsa_key = include_str!("../../resources/rsa_key.pem");
        let rsa_key = RsaPrivateKey::from_pkcs1_pem(rsa_key).unwrap();

        PacketProcessor {
            client: None,
            server: None,
            decryption_key: None,

            rsa_key,
            token_rsp: None,
            bruteforce: Bruteforce::new()
        }
    }

    /// Initializes the client and server KCP readers.
    /// conversation: The client's KCP conversation (conv) ID.
    /// token: The client's KCP token. (GI proprietary)
    pub fn init(&mut self, conversation: u32, token: u32) {
        self.client = Some(new_kcp(conversation, token));
        self.server = Some(new_kcp(conversation, token));
    }

    /// Processes new KCP packets.
    /// data: The raw UDP packet data.
    /// is_client: Is the packet from the client?
    pub fn process(&mut self, data: &[u8], is_client: bool) {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32;

        let kcp = if is_client {
            self.client.as_mut()
        } else {
            self.server.as_mut()
        };

        if let Some(kcp) = kcp {
            _ = kcp.update(time);
            _ = kcp.input(data);
        }
    }

    /// Reads a packet from the KCP instance.
    pub fn receive(&mut self, from_client: bool) -> Option<Packet> {
        // Determine which KCP instance to read from.
        let kcp = if from_client {
            self.client.as_mut()
        } else {
            self.server.as_mut()
        };
        // Check if the KCP instance exists.
        let kcp = match kcp {
            Some(kcp) => kcp,
            None => return None
        };

        // Read the packet.
        let size = match kcp.peeksize() {
            Ok(size) => size,
            Err(_) => return None
        };

        let mut buffer = vec![0; size];
        match kcp.recv(&mut buffer) {
            Ok(_) => self.decrypt_packet(buffer, from_client),
            Err(_) => return None
        }
    }

    /// Receives a packet from the client and server.
    pub fn receive_both(&mut self) -> (Option<Packet>, Option<Packet>) {
        let client = self.receive(true);
        let server = self.receive(false);

        (client, server)
    }

    /// Decrypts the packet data.
    /// packet: The raw packet data.
    /// from_client: Is the packet from the client?
    pub fn decrypt_packet(
        &mut self,
        mut packet: Vec<u8>,
        from_client: bool
    ) -> Option<Packet> {
        // Attempt to decrypt the data.
        let data: Option<Vec<u8>> = match &self.decryption_key {
            None => {
                // Determine which miHoYo dispatch key to use.
                let keys = keys::dispatch_keys();
                let index = u16::from_be_bytes([packet[0] ^ 0x45, packet[1] ^ 0x67]);

                if keys.contains_key(&index) {
                    let key = Key::from(&keys[&index]);
                    key.xor(&mut packet);

                    self.decryption_key = Some(PacketKey::Dispatch(key));

                    Some(packet)
                } else {
                    error!("No dispatch key found for bytes {:x?}", index);
                    return None;
                }
            },
            Some(PacketKey::Dispatch(key)) => {
                // Test dispatch key. (miHoYo might be nice!)
                let mut test = [packet[0].clone(), packet[1].clone()];
                key.xor(&mut test);

                if is_valid(&test) {
                    // Continue using the working key.
                    key.xor(&mut packet);
                    Some(packet)
                } else {
                    // Prepare for brute-forcing the session key.
                    let (sent_time, seed) = match &self.token_rsp {
                        Some((sent_time, seed)) => (*sent_time, *seed),
                        None => {
                            error!("No token response data found.");
                            return None;
                        }
                    };

                    // Attempt to brute-force the session key.
                    if let Some(seed) = self.bruteforce.run(sent_time, seed, &packet) {
                        let key = Key::new(seed);
                        key.xor(&mut packet);

                        if is_valid(&packet) {
                            self.decryption_key = Some(PacketKey::Session(key));
                            Some(packet)
                        } else {
                            warn!("Invalid session key created.");
                            None
                        }
                    } else {
                        None
                    }
                }
            },
            Some(PacketKey::Session(key)) => {
                key.xor(&mut packet);
                if !is_valid(&packet) {
                    warn!("Invalidated session key.");
                    self.decryption_key = None;
                    None
                } else {
                    Some(packet)
                }
            }
        };

        // Re-check for a valid packet.
        let data = match data {
            Some(data) => data,
            None => return None
        };
        if !is_valid(&data) {
            warn!("Invalid packet data received (after decryption).");
            warn!("Maybe encryption key changed?");
            return None;
        }

        // Parse the packet.
        let id = u16::from_be_bytes([data[2], data[3]]);
        let header_size = u16::from_be_bytes([data[4], data[5]]) as usize;
        // UNUSED: let data_size = u32::from_be_bytes([data[6], data[7], data[8], data[9]]);

        let header = data[10..10 + header_size].to_vec();
        let data = data[10 + header_size..data.len() - 2].to_vec();

        Some(Packet {
            id,
            header,
            data,
            is_client: from_client
        })
    }

    /// Processes the given packet.
    /// This only does something in the instance of 'GetPlayerTokenRsp'.
    pub fn update(&mut self, packet: Packet) {
        // TODO: Update if the packet is a 'GetPlayerTokenRsp'.
    }

    /// Updates the internal token response data.
    /// sent_time: The time the packet was sent. (this comes from the packet header)
    /// rsa_key: The RSA-string key. (serverRandKey)
    pub fn set_token_rsp(&mut self, sent_time: u64, rsa_key: String) {
        let content = STANDARD.decode(rsa_key).unwrap();
        self.rsa_key.decrypt(Pkcs1v15Encrypt, &content).unwrap();

        self.token_rsp = Some((
            sent_time,
            u64::from_be_bytes(content[0..8].try_into().unwrap())
        ));
    }
}

pub struct Writer;

impl Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
