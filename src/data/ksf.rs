use anyhow::{Context, Result};
use egui::Key;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

/// A keybind consists of a Key and a description string.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "(String,String)")]
pub struct Keybind(pub Key, pub String);

impl Display for Keybind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.0.name(), self.1)
    }
}

// Becayse egui::Key::from_name() is not used to deserialize egui::Key we need this to help out serde
impl TryFrom<(String, String)> for Keybind {
    type Error = anyhow::Error;

    fn try_from(value: (String, String)) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(Keybind(
            egui::Key::from_name(&value.0).context("invalid key specification for keybind")?,
            value.1,
        ))
    }
}

impl Keybind {
    /// Only for development use. Users should always build from a KSF file.
    pub fn from_string(s: &str) -> Self {
        let mut pair = s.split(",");
        let key = egui::Key::from_name(pair.next().unwrap().trim()).unwrap();
        let description = pair.next().unwrap().to_string();
        Keybind(key, description)
    }
}

/// A list of keybinds divided into Duration and Frequency
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ksf {
    pub name: String,
    pub duration: Vec<Keybind>,
    pub frequency: Vec<Keybind>,
}

impl Default for Ksf {
    fn default() -> Self {
        Self {
            name: String::from("DEFAULT"),
            duration: vec![Keybind::from_string("4,Toy Engage")],
            frequency: vec![
                Keybind::from_string("M,Mand"),
                Keybind::from_string("A,Agression"),
                Keybind::from_string("S,SIB"),
            ],
        }
    }
}

impl Ksf {
    pub fn from_file(file_path: PathBuf) -> Result<Self> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }

    pub fn pretty_print(&self) -> String {
        let mut out = String::from("Duration Keys\n");
        out.push_str(&self.duration.iter().map(|kb| kb.to_string()).join("\n"));
        out.push_str("\n\nFrequency Keys\n");
        out.push_str(&self.frequency.iter().map(|kb| kb.to_string()).join("\n"));
        out
    }
}
