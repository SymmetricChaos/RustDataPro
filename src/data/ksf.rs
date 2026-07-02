use anyhow::{Context, Result};
use csv::StringRecord;
use itertools::Itertools;
use std::{fmt::Display, fs::File, path::PathBuf};

/// A keybind consists of a Key and a description string.
#[derive(Debug, Clone)]
pub struct Keybind {
    pub key: egui::Key,
    pub description: String,
}

impl Display for Keybind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.key.name(), self.description)
    }
}

impl Keybind {
    pub fn from_string_record(record: StringRecord) -> Result<Self> {
        Ok(Keybind {
            key: egui::Key::from_name(record.get(1).context("missing key for keybind")?)
                .context("invalid key specification for keybind")?,
            description: record
                .get(2)
                .context("missing description for keybind")?
                .to_string(),
        })
    }

    /// Only for development use. Users should always build from a KSF file.
    pub fn from_string(s: &str) -> Self {
        let mut pair = s.split(",");
        let key = egui::Key::from_name(pair.next().unwrap().trim()).unwrap();
        let description = pair.next().unwrap().to_string();
        Keybind { key, description }
    }
}

/// A list of keybinds divided into Duration and Frequency
#[derive(Debug, Clone)]
pub struct Ksf {
    pub name: String,
    pub duration: Vec<Keybind>,
    pub frequency: Vec<Keybind>,
}

impl Default for Ksf {
    fn default() -> Self {
        Self {
            name: String::from("Default KSF Loaded"),
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
    pub fn new() -> Self {
        Ksf {
            name: String::new(),
            duration: Vec::new(),
            frequency: Vec::new(),
        }
    }

    pub fn from_file(file_path: PathBuf) -> Result<Ksf> {
        let file = File::open(&file_path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut ksf = Ksf::new();
        ksf.name = file_path
            .file_name()
            .context("file has no name")?
            .to_str()
            .context("failed to convert file name to string")?
            .to_string();
        for result in rdr.records() {
            let mut record: StringRecord = result.context("StringRecord invalid")?;
            record.trim();
            if record.len() != 3 {
                return Err(anyhow::anyhow!(
                    "each KSF line must have exactly three items"
                ));
            }
            match record.get(0).unwrap() {
                "d" => ksf.duration.push(Keybind::from_string_record(record)?),
                "f" => ksf.frequency.push(Keybind::from_string_record(record)?),
                _ => return Err(anyhow::anyhow!("invalid data_kind, must be either d of f")),
            };
        }

        Ok(ksf)
    }

    pub fn pretty_print(&self) -> String {
        let mut out = String::from("Duration Keys\n");
        out.push_str(&self.duration.iter().map(|kb| kb.to_string()).join("\n"));
        out.push_str("\n\nFrequency Keys\n");
        out.push_str(&self.frequency.iter().map(|kb| kb.to_string()).join("\n"));
        out
    }
}
