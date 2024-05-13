use common::utils;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sniffer {
    #[serde(default = "Sniffer::default_address")]
    pub bind_address: String,
    #[serde(default = "Sniffer::default_port")]
    pub bind_port: u16
}

impl Default for Sniffer {
    /// Create a new `Sniffer` with default values.
    fn default() -> Self {
        Sniffer {
            bind_address: Sniffer::default_address(),
            bind_port: Sniffer::default_port()
        }
    }
}

impl Sniffer {
    fn default_address() -> String { "127.0.0.1".to_string() }
    fn default_port() -> u16 { 8080 }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deobfuscation {
    #[serde(default = "Deobfuscation::default_obfuscated_defs")]
    pub obfuscated_defs: String,
    #[serde(default = "Deobfuscation::default_output_dir")]
    pub output_dir: String,
    #[serde(default = "Deobfuscation::default_dumps")]
    pub packet_dumps: String
}

impl Default for Deobfuscation {
    /// Create a new `Deobfuscation` with default values.
    fn default() -> Self {
        Deobfuscation {
            obfuscated_defs: Deobfuscation::default_obfuscated_defs(),
            output_dir: Deobfuscation::default_output_dir(),
            packet_dumps: Deobfuscation::default_dumps()
        }
    }
}

impl Deobfuscation {
    fn default_obfuscated_defs() -> String { "defs".to_string() }
    fn default_output_dir() -> String { "output".to_string() }
    fn default_dumps() -> String { "dumps".to_string() }

    /// Create the directories if they do not exist.
    pub fn make_dirs(&self) {
        utils::make_dir(&self.output_dir);
        utils::make_dir(&self.packet_dumps);
        utils::make_dir(&self.obfuscated_defs);
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    #[serde(default)]
    pub device: u16,
    #[serde(default)]
    pub sniffer: Sniffer,
    #[serde(default)]
    pub deobfuscation: Deobfuscation,
}
