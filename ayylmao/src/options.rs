use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Sniffer {
    #[serde(default = "Sniffer::default_address")]
    pub bind_address: String,
    #[serde(default = "Sniffer::default_port")]
    pub bind_port: u16
}

impl Sniffer {
    fn default_address() -> String { "127.0.0.1".to_string() }
    fn default_port() -> u16 { 8080 }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Deobfuscation {
    pub obfuscated_defs: String,
    pub output_dir: String,
    pub packet_dumps: String
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Options {
    #[serde(default)]
    pub device: u16,
    #[serde(default)]
    pub sniffer: Sniffer,
    #[serde(default)]
    pub deobfuscation: Deobfuscation,
}
