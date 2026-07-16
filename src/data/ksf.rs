use anyhow::{Context, Result};
use egui::Key;
use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

/// Key Specification File. A list of keybinds divided into Duration and Frequency.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct KsfData {
    pub duration: Vec<(Key, String)>,
    pub frequency: Vec<(Key, String)>,
}

impl Default for KsfData {
    fn default() -> Self {
        serde_json::from_str(
            r#"{
                "frequency": [
                    ["V", "NegVoc"],
                    ["A", "Aggression"],
                    ["M", "Mand"],
                    ["S", "SIB"],
                    ["I", "Instruction"],
                    ["C", "Compliance"]
                ],
                "duration": [
                    ["Num4", "ToyEngage"],
                    ["Num1", "Sr+"],
                    ["Num2", "Sdelta"]
                ]
            }"#,
        )
        .unwrap()
    }
}

impl KsfData {
    pub fn blank() -> Self {
        Self {
            duration: Vec::new(),
            frequency: Vec::new(),
        }
    }

    /// All key/description pairs. Frequency first.
    pub fn pairs(&self) -> impl Iterator<Item = &(Key, String)> {
        self.frequency.iter().chain(self.duration.iter())
    }

    /// All keys. Frequency first.
    pub fn keys(&self) -> impl Iterator<Item = &Key> {
        self.pairs().map(|(k, _)| k)
    }

    /// All description. Frequency first.
    pub fn descriptions(&self) -> impl Iterator<Item = &String> {
        self.pairs().map(|(_, d)| d)
    }

    /// Create a HashMap from the contents.
    pub fn create_map(&self) -> IndexMap<Key, String> {
        let mut map = IndexMap::with_capacity(self.frequency.len() + self.duration.len());
        for (k, v) in self.pairs() {
            map.insert(*k, v.clone());
        }
        map
    }

    /// Are all keys unique AND all descriptions unique?
    pub fn is_valid(&self) -> bool {
        self.keys().all_unique() && self.descriptions().all_unique()
    }

    pub fn from_file(file_path: &Path) -> Result<Self> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        let out: KsfData = serde_json::from_str(&s)?;
        if !out.is_valid() {
            Err(anyhow::anyhow!(
                "ksf contains duplicate keys or duplicate descriptions"
            ))
        } else {
            Ok(out)
        }
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self).context("unable to convert ksf to json")
    }
}
