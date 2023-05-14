use std::path::PathBuf;

pub fn create_temp_dir() -> Result<PathBuf, Box<dyn std::error::Error + Sync + Send>> {
    let root = std::env::current_dir().unwrap();
    let temp_dir = root.join("temp");
    std::fs::create_dir_all(&temp_dir)?;
    Ok(temp_dir)
}
