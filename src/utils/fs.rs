use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Check if a file is binary
pub fn is_binary_file(path: &str) -> Result<bool> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Ok(false);
    }

    let mut file = File::open(path)?;
    let mut buffer = vec![0; 8000]; // Read first 8KB

    let bytes_read = file.read(&mut buffer)?;
    buffer.truncate(bytes_read);

    // Check for null bytes (common in binary files)
    Ok(buffer.iter().any(|&b| b == 0))
}

/// Read file content with encoding detection
pub fn read_file_with_encoding(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

/// Get file extension
pub fn get_file_extension(path: &str) -> Option<String> {
    Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
}
