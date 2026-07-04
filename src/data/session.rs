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
    pub therapist: String,
    pub data_collector: String,
}

impl Default for SessionData {
    fn default() -> Self {
        Self {
            assessment: String::from("None"),
            condition: String::from("None"),
            therapist: String::from("None"),
            data_collector: String::from("None"),
            data_type: DataType::Primary,
        }
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
