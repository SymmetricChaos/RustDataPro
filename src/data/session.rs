use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct SessionData {
    pub assessment: String,
    pub condition: String,
    pub data_type: DataType,
}

impl Default for SessionData {
    fn default() -> Self {
        Self {
            assessment: "NONE".into(),
            condition: "NONE".into(),
            data_type: DataType::Primary,
        }
    }
}
