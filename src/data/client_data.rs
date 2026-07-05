use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    pub name: String,
    pub client_id: String,
    pub case_manager: String,
    pub primary_therapist: String,
    pub assessments: Vec<String>,
    pub conditions: Vec<String>,
    pub session_number: u32,
}

impl Default for ClientData {
    fn default() -> Self {
        serde_json5::from_str(
            r#"{
                "name": "None None",
                "client_id": "0000000000000000",
                "case_manager": "None None",
                "primary_therapist": "None None",
                "assessments": [
                    "None"
                ],
                "conditions": [
                    "None"
                ],
                "session_number": 0
            }"#,
        )
        .unwrap()
    }
}

impl Display for ClientData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Client: {}\nID: {}\nCase Manager: {}\nPrimary Therapist: {}\nSession Number: {}", // NOTICE: assessments and conditions are exluded from this display
            self.name,
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
        Ok(serde_json5::from_str(&s)?)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json5::to_string(&self).context("unable to convert client data to json")
    }
}
