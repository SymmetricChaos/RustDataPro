use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataType {
    Primary,
    Reliability,
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Primary => write!(f, "Primary"),
            DataType::Reliability => write!(f, "Reliability"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub assessment: String,
    pub condition: String,
    pub data_type: DataType,
    pub therapist: String,
    pub data_collector: String,
}

impl Default for SessionData {
    fn default() -> Self {
        serde_json5::from_str(
            r#"{
                "assessment": "None",
                "condition": "None",
                "therapist": "None",
                "data_collector": "None",
                "data_type": "Primary"
            }"#,
        )
        .unwrap()
    }
}

impl Display for SessionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Assessment: {}\nCondition: {}\nTherapist: {}\nData Collector: {}\nData Type: {}",
            self.assessment, self.condition, self.therapist, self.data_collector, self.data_type,
        )
    }
}

impl SessionData {
    pub fn from_file(file_path: PathBuf) -> Result<Self> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(serde_json5::from_str(&s)?)
    }
}
