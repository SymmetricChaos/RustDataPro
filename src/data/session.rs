use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SessionData {
    pub first_name: String,
    pub last_name: String,
    pub client_id: String,
    pub assessment: String,
    pub condition: String,
    pub data_type: String,
    pub session_number: u32,
}

impl Default for SessionData {
    fn default() -> Self {
        Self {
            first_name: "MISSING".into(),
            last_name: "MISSING".into(),
            client_id: "MISSING".into(),
            assessment: "MISSING".into(),
            condition: "MISSING".into(),
            data_type: "MISSING".into(),
            session_number: u32::MAX,
        }
    }
}

impl Display for SessionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Client: {} {}\nID: {}\nAssessment: {}\nCondition: {}\nData Type: {}\nSession Number: {}",
            self.first_name,
            self.last_name,
            self.client_id,
            self.assessment,
            self.condition,
            self.data_type,
            self.session_number
        )
    }
}

impl SessionData {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn from_file(file_path: PathBuf) -> Result<SessionData> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }
}
