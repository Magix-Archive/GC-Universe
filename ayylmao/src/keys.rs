use bytes::BufMut;
use common::utils;
use std::collections::HashMap;

use crate::capture::bruteforce::MT19937_64;

const KEYS: &str = include_str!("../resources/mhy_keys.txt");

pub struct Key {
    bytes: Vec<u8>
}

impl Key {
    /// Creates a new instance of the `Key`.
    pub fn new(seed: u64) -> Self {
        let mut generator = MT19937_64::default();
        generator.seed(seed);
        let seed = generator.next_ulong();
        generator.seed(seed);

        _ = generator.next_ulong(); // Skip the first number.

        // Generate the key.
        let mut bytes = vec![];
        for _ in (0..4096).step_by(8) {
            bytes.put_u64(generator.next_ulong());
        }

        Key { bytes }
    }

    /// Creates a new instance of the `Key`.
    /// Uses an existing key.
    pub fn from(key: &[u8]) -> Self {
        Key { bytes: key.to_vec() }
    }

    /// Performs an XOR cipher on the data.
    /// data: The data to encrypt/decrypt.
    pub fn xor(&self, data: &mut [u8]) {
        utils::xor(data, &self.bytes);
    }

    /// Compares this key to the pre-computed values.
    /// known: The known prefix and suffix of the key.
    /// data: The test data to compare against.
    pub fn compare(&self, known: ([u8; 2], [u8; 2]), data: &[u8]) -> bool {
        let (prefix, suffix) = known;

        let data_len = data.len();
        let key_len = self.bytes.len();

        let prefix_valid = self.bytes[0] == prefix[0] && self.bytes[1] == prefix[1];
        let suffix_valid =
            self.bytes[(data_len - 2) % key_len] == suffix[0] &&
            self.bytes[(data_len - 1) % key_len] == suffix[1];

        prefix_valid && suffix_valid
    }
}

/// Loads miHoYo's dispatch keys.
/// Returns: A map of <first byte> -> <4096-bit XOR key>
pub fn dispatch_keys() -> HashMap<u16, [u8; 4096]> {
    let mut keys = HashMap::new();
    for key in KEYS.lines() {
        let parts = key.split(": ").collect::<Vec<&str>>();
        let (first_byte, key) = match parts.as_slice() {
            [f, s] => (f, s),
            _ => panic!("Invalid key format.")
        };

        let first_byte = first_byte.parse::<u16>().unwrap();
        let mut key_bytes = [0u8; 4096];

        let mut i = 0;
        for byte in (0..key.len() - 1).step_by(2) {
            key_bytes[i] = u8::from_str_radix(&key[byte..byte + 2], 16).unwrap();
            i += 1;
        }

        keys.insert(first_byte, key_bytes);
    }

    keys
}
