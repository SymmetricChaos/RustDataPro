use anyhow::{Context, Result};
use egui::Key;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::Read,
    path::PathBuf,
};

/// A list of keybinds divided into Duration and Frequency
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct KsfData {
    pub name: String,
    pub duration: Vec<(Key, String)>,
    pub frequency: Vec<(Key, String)>,
}

impl Default for KsfData {
    fn default() -> Self {
        serde_json::from_str(
            r#"{
                "name": "DEFAULT",
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
    pub fn from_file(file_path: PathBuf) -> Result<Self> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(&self).context("unable to convert ksf to json")
    }
}
