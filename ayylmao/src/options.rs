use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Sniffer {
    #[serde(default = "Sniffer::default_port")]
    pub bind_port: u16
}

impl Sniffer {
    fn default_port() -> u16 { 8080 }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Deobfuscation {
    pub obfuscated_defs: String,
    pub output_dir: String,
    pub packet_dumps: String
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Options {
    #[serde(default)]
    pub device: u16,
    #[serde(default)]
    pub sniffer: Sniffer,
    #[serde(default)]
    pub deobfuscation: Deobfuscation,
}