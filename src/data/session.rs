use std::fmt::Display;

pub struct SessionData {
    pub first_name: String,
    pub last_name: String,
    pub client_id: String,
    pub assessment: String,
    pub condition: String,
    pub data_type: String,
    pub session_number: u32,
}

impl Default for SessionData {
    fn default() -> Self {
        Self {
            first_name: "MISSING".into(),
            last_name: "MISSING".into(),
            client_id: "MISSING".into(),
            assessment: "MISSING".into(),
            condition: "MISSING".into(),
            data_type: "MISSING".into(),
            session_number: u32::MAX,
        }
    }
}

impl Display for SessionData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Client: {} {}\nID: {}\nAssessment: {}\nCondition: {}\nData Type: {}\nSession Number: {}",
            self.first_name,
            self.last_name,
            self.client_id,
            self.assessment,
            self.condition,
            self.data_type,
            self.session_number
        )
    }
}

impl SessionData {
    pub fn blank() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn new(
        first_name: &str,
        last_name: &str,
        client_id: &str,
        assessment: &str,
        condition: &str,
        data_type: &str,
        session_number: u32,
    ) -> Self {
        Self {
            first_name: first_name.into(),
            last_name: last_name.into(),
            client_id: client_id.into(),
            assessment: assessment.into(),
            condition: condition.into(),
            data_type: data_type.into(),
            session_number,
        }
    }
}
