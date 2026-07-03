use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    pub first_name: String,
    pub last_name: String,
    pub client_id: String,
    pub session_number: u32,
}

impl Default for ClientData {
    fn default() -> Self {
        Self {
            first_name: "MISSING".into(),
            last_name: "MISSING".into(),
            client_id: "MISSING".into(),
            // assessment: "MISSING".into(),
            // condition: "MISSING".into(),
            // data_type: "MISSING".into(),
            session_number: 99999,
        }
    }
}

impl Display for ClientData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Client: {} {}\nID: {}\nSession Number: {}",
            self.first_name,
            self.last_name,
            self.client_id,
            // self.assessment,
            // self.condition,
            // self.data_type,
            self.session_number
        )
    }
}

impl ClientData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from_file(file_path: PathBuf) -> Result<ClientData> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }
}
