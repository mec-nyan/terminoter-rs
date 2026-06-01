use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub notes: Vec<Note>,
}

pub fn load_data(path: &str) -> Result<Data, String> {
    // Read file:
    let result = fs::read_to_string(path);
    match result {
        Err(msg) => Err(format!("Error reading file: {}", msg)),
        Ok(content) => {
            let data = serde_json::from_str(&content);
            match data {
                Err(msg) => Err(format!("error reading JSON: {}", msg)),
                Ok(data) => Ok(data),
            }
        }
    }
}
