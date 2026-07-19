use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Read,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

/// A list of assessments names paired with a list of their conditions.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AssessmentsData(Vec<(String, Vec<String>)>);

impl Deref for AssessmentsData {
    type Target = Vec<(String, Vec<String>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AssessmentsData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AssessmentsData {
    pub fn from_file(file_path: &PathBuf) -> Result<Self> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self).context("unable to convert assessments to json")
    }

    pub fn fa_conditions() -> Self {
        Self(vec![(
            "FA".into(),
            vec![
                "Ignore".into(),
                "Tangible".into(),
                "Demand".into(),
                "Attention".into(),
                "ToyPlay".into(),
            ],
        )])
    }
}
