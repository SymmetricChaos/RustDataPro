use crate::data::{ClientData, DataType, KsfData, SessionData, timeline::Timeline};
use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputData {
    pub datetime: String,
    pub client: ClientData,
    pub session: SessionData,
    pub session_duration: f32,
    pub frequency: IndexMap<String, u32>,
    pub duration: IndexMap<String, (u32, f32)>,
    pub timeline: Timeline,
    pub ksf: KsfData,
}

impl OutputData {
    pub fn session_number(&self) -> u32 {
        self.client.current_session
    }

    pub fn data_type(&self) -> DataType {
        self.session.data_type
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
