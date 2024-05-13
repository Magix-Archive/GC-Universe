use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::Serialize;

/// Check if a file exists from a file path.
/// file_path: The path to the file.
pub fn file_exists<S: AsRef<str>>(file_path: S) -> bool {
    Path::new(file_path.as_ref()).exists()
}

/// Write a JSON file from a Rust object.
/// file_path: The path to the file.
/// data: The Rust object to write to the file.
pub fn write_json<T>(file_path: &str, data: T) -> std::io::Result<()>
    where T: Serialize,
{
    let file = File::create(file_path)?;
    serde_json::to_writer_pretty(file, &data)?;
    Ok(())
}

/// Read a file from a file path.
/// file_path: The path to the file.
pub fn read_file<S: AsRef<str>>(file_path: S) -> std::io::Result<String> {
    let mut file = File::open(file_path.as_ref())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Write a file from a string.
/// file_path: The path to the file.
pub fn write_file(file_path: &str, content: String) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Performs an XOR cipher on the data.
/// data: The data to encrypt/decrypt.
/// key: The key to use for the cipher.
pub fn xor(data: &mut [u8], key: &[u8]) {
    for i in 0..data.len() {
        data[i] ^= key[i % key.len()];
    }
}