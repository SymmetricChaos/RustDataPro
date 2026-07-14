use anyhow::{Context, Result};
use egui::Key;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

use crate::data::KsfData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IoaData {
    pub ten_sec_interval: IndexMap<Key, f32>,
    pub sixty_sec_interval: IndexMap<Key, f32>,
    pub total_duration: IndexMap<Key, f32>,
    pub total_count: IndexMap<Key, f32>,
}

impl IoaData {
    pub fn new() -> Self {
        Self {
            ten_sec_interval: IndexMap::new(),
            sixty_sec_interval: IndexMap::new(),
            total_duration: IndexMap::new(),
            total_count: IndexMap::new(),
        }
    }

    pub fn from_ksf(ksf: &KsfData) -> Self {
        let mut ioa = IoaData::new();
        // Total duration is meaningless for frequency keys but we need this for alignment
        for (k, _) in ksf.frequency.iter() {
            ioa.total_duration.insert(*k, f32::NAN);
        }
        for (k, _) in ksf.duration.iter() {
            ioa.total_duration.insert(*k, 0.0);
        }
        for k in ksf.keys() {
            ioa.ten_sec_interval.insert(*k, 0.0);
            ioa.sixty_sec_interval.insert(*k, 0.0);
            ioa.total_count.insert(*k, 0.0);
        }
        ioa
    }

    pub fn normalize(&mut self, n: f32) {
        for v in self.ten_sec_interval.values_mut() {
            *v /= n
        }
        for v in self.sixty_sec_interval.values_mut() {
            *v /= n
        }
        for v in self.total_duration.values_mut() {
            *v /= n
        }
        for v in self.total_count.values_mut() {
            *v /= n
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
