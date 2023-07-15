use std::{fs::File, io::Read};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Keys {
    pub deepl: String,
    pub assembly_ai: String,
}

impl Keys {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => {
                let error =
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to open keys file");
                return Err(error);
            }
        };
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        let keys: Keys = match serde_json::from_str(&file_content) {
            Ok(keys) => keys,
            Err(_) => {
                let error =
                    std::io::Error::new(std::io::ErrorKind::Other, "Failed to parse keys file");
                return Err(error);
            }
        };

        return Ok(keys);
    }
}
