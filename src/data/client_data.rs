use anyhow::{Context, Result};
use chrono::{Datelike, Local, NaiveDate};
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
    pub date_of_admission: String,
    pub location: String,
}

impl Default for ClientData {
    fn default() -> Self {
        Self {
            name: String::from("None"),
            id: String::from("None"),
            case_manager: String::from("None"),
            primary_therapist: String::from("None"),
            assessments: Vec::new(),
            conditions: Vec::new(),
            current_session: 0,
            date_of_admission: Local::now().date_naive().format("%Y-%m-%d").to_string(),
            location: String::from("None"),
        }
    }
}

impl Display for ClientData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Client: {}\nID: {}\nLocation: {}\nCase Manager: {}\nPrimary Therapist: {}\nSession Number: {}\nDOA {}",
            self.name,
            self.id,
            self.location,
            self.case_manager,
            self.primary_therapist,
            self.current_session,
            self.doa()
        )
    }
}

impl ClientData {
    /// Number of days since admission
    pub fn doa(&self) -> i32 {
        let x = NaiveDate::parse_from_str(&self.date_of_admission, "%Y-%m-%d")
            .unwrap()
            .num_days_from_ce();
        Local::now().date_naive().num_days_from_ce() - x
    }

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
