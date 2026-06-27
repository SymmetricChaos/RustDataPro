use anyhow::{Context, Result};
use csv::StringRecord;
use itertools::Itertools;
use std::{fmt::Display, fs::File};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataKind {
    Frequency,
    Duration,
}

impl Display for DataKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataKind::Frequency => write!(f, "f"),
            DataKind::Duration => write!(f, "d"),
        }
    }
}

/// A keybind consists of a Key, whether the binding is for Frequency or Duration, and a description string
#[derive(Debug, Clone)]
pub struct Keybind {
    pub data_kind: DataKind,
    pub key: egui::Key,
    pub description: String,
}

impl Keybind {
    pub fn build(mut record: StringRecord) -> Result<Self> {
        record.trim();
        if record.len() != 3 {
            return Err(anyhow::anyhow!(
                "each KSF line must have exactly three items"
            ));
        }

        let data_kind = match record.get(0).unwrap() {
            "d" => DataKind::Duration,
            "f" => DataKind::Frequency,
            _ => return Err(anyhow::anyhow!("invalid data_kind, must be either d of f")),
        };
        let key =
            egui::Key::from_name(record.get(1).unwrap()).context("invalid key specification")?;
        let description = record.get(2).unwrap().to_string();

        Ok(Keybind {
            data_kind,
            key,
            description,
        })
    }
}

impl Display for Keybind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.data_kind,
            self.key.name(),
            self.description
        )
    }
}

/// A list of keybinds.
#[derive(Debug, Clone)]
pub struct Ksf {
    pub keybinds: Vec<Keybind>,
}

impl Ksf {
    pub fn new() -> Self {
        Ksf {
            keybinds: Vec::new(),
        }
    }

    pub fn pretty_print(&self) -> String {
        self.keybinds.iter().map(|kb| kb.to_string()).join("\n")
    }
}

pub fn read_ksf(file_path: &str) -> Result<Ksf> {
    let file = File::open(file_path).context("file name not found")?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut ksf = Ksf::new();
    for result in rdr.records() {
        let record: StringRecord = result.unwrap();
        ksf.keybinds.push(Keybind::build(record)?);
    }

    Ok(ksf)
}

#[test]
fn test() {
    println!(
        "{}",
        read_ksf("src/example_ksf.txt").unwrap().pretty_print()
    );
}
