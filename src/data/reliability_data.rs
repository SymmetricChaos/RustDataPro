use anyhow::{Context, Result};
use egui::Key;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IoaData {
    pub ten_sec_interval: Vec<(Key, f32)>,
    pub sixty_sec_interval: Vec<(Key, f32)>,
    pub total_duration: Vec<(Key, f32)>,
    pub total_count: Vec<(Key, f32)>,
}

impl IoaData {
    pub fn new() -> Self {
        Self {
            ten_sec_interval: Vec::new(),
            sixty_sec_interval: Vec::new(),
            total_duration: Vec::new(),
            total_count: Vec::new(),
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
