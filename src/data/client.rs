use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    pub first_name: String,
    pub last_name: String,
    pub client_id: String,
    pub case_manager: String,
    pub primary_therapist: String,
    pub assessments: Vec<String>,
    pub conditions: Vec<String>,
    pub session_number: u32,
}

impl Default for ClientData {
    fn default() -> Self {
        Self {
            first_name: String::from("NONE"),
            last_name: String::from("NONE"),
            client_id: String::from("NONE"),
            case_manager: String::from("NONE"),
            primary_therapist: String::from("NONE"),
            assessments: vec![String::from("NONE")],
            conditions: vec![String::from("NONE")],
            session_number: 0,
        }
    }
}

impl Display for ClientData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Client: {} {}\nID: {}\nCase Manager: {}\nPrimary Therapist: {}\nSession Number: {}", // NOTICE: assessments and conditions are exluded from this display
            self.first_name,
            self.last_name,
            self.client_id,
            self.case_manager,
            self.primary_therapist,
            self.session_number
        )
    }
}

impl ClientData {
    pub fn from_file(file_path: &PathBuf) -> Result<Self> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }
}
