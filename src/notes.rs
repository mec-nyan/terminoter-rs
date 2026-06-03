use std::{fs, io};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Note {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Data {
    pub notes: Vec<Note>,
}

pub fn load_data(path: &str) -> io::Result<Data> {
    let result = fs::read_to_string(path)?;
    let data: Data = serde_json::from_str(&result)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::notes::{Data, Note, load_data};

    #[test]
    fn load_data_ok() {
        let file_name = "test_data_ok.json";
        let content = r#"{
  "notes": [
    {
      "content": "This is a note.",
      "metadata": {}
    }
  ]
}"#;
        fs::write(file_name, content).expect("writing content to file.");

        let data = load_data(file_name).expect("should load data correctly.");

        let _ = fs::remove_file(file_name);

        let expected = Data {
            notes: vec![Note {
                content: "This is a note.".to_string(),
            }],
        };

        assert_eq!(data, expected);
    }

    #[test]
    fn load_data_invalid_json() {
        let file_name = "test_data_invalid.json";
        let content = "not a json file haha";
        fs::write(file_name, content).expect("writing content to file.");

        assert!(load_data(file_name).is_err());

        let _ = fs::remove_file(file_name);
    }

    #[test]
    fn load_data_no_file() {
        let file_name = "test_data_no_file.json";

        assert!(load_data(file_name).is_err());
    }
}
