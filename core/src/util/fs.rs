use std::path::PathBuf;

pub fn create_temp_dir() -> Result<PathBuf, std::io::Error> {
    let root = std::env::current_dir().unwrap();
    let temp_dir = root.join("temp");
    std::fs::create_dir_all(&temp_dir)?;
    Ok(temp_dir)
}
pub fn check_file_size(path: &PathBuf) -> Result<u64, std::io::Error> {
    let metadata = std::fs::metadata(path)?;
    let size = metadata.len();
    let mb = size / 1024 / 1024;
    Ok(mb)
}
