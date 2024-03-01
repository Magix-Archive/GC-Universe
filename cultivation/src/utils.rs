use std::fs::File;
use std::path::Path;
use serde::Serialize;

/// Check if a file exists from a file path.
/// file_path: The path to the file.
pub fn file_exists(file_path: &str) -> bool {
    Path::new(&file_path).exists()
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
