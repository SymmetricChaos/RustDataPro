use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReliData {
    pub ten_sec_interval: Vec<(String, f32)>,
    pub sixty_sec_interval: Vec<(String, f32)>,
    pub duration_ratio: Vec<(String, f32)>,
}

impl ReliData {
    pub fn new() -> Self {
        Self {
            ten_sec_interval: Vec::new(),
            sixty_sec_interval: Vec::new(),
            duration_ratio: Vec::new(),
        }
    }

    pub fn from_file(file_path: &Path) -> Result<Self> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self).context("unable to convert session data to json")
    }
}
