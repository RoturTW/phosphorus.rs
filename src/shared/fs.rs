use std::fs;
use std::path::PathBuf;

pub fn read_file(path: &PathBuf) -> Result<String, String> {
    if !path.exists() {
        return Err(format!("file not found: {}", path.display()));
    }
    fs::read_to_string(path)
        .map_err(|e| format!("{}: {}", path.display(), e))
}