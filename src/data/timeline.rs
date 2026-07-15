use anyhow::{Context, Result};
use egui::Key;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Read,
    ops::{Deref, DerefMut},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timeline(Vec<(Key, f32)>);

impl Default for Timeline {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Deref for Timeline {
    type Target = Vec<(Key, f32)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Timeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Timeline {
    pub fn from_file(file_path: &Path) -> Result<Self> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self).context("unable to convert timeline data to json")
    }
}
