use anyhow::{Context, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientData {
    pub name: String,
    pub id: String,
    pub case_manager: String,
    pub primary_therapist: String,
    pub assessments: Vec<String>,
    pub conditions: Vec<String>,
    pub current_session: u32,
}

impl Default for ClientData {
    fn default() -> Self {
        serde_json::from_str(
            r#"{
                "name": "None None",
                "client_id": "00000000",
                "case_manager": "None None",
                "primary_therapist": "None None",
                "assessments": [
                    "None"
                ],
                "conditions": [
                    "None"
                ],
                "current_session": 0
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
            self.name, self.id, self.case_manager, self.primary_therapist, self.current_session
        )
    }
}

impl ClientData {
    /// String containing only capital letters from client name.
    pub fn initials(&self) -> String {
        self.name
            .chars()
            .filter(|c| c.is_ascii_uppercase())
            .join("")
    }

    pub fn from_file(file_path: &PathBuf) -> Result<Self> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self).context("unable to convert client data to json")
    }
}
