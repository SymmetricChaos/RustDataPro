use serde::{Deserialize, Serialize};
use egui::Key;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "(String,String)")]
pub struct Moment(pub Key, pub f32);

impl Display for Moment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0.name(), self.1)
    }
}

impl TryFrom<(String, String)> for Moment {
    type Error = anyhow::Error;

    fn try_from(value: (String, String)) -> std::prelude::v1::Result<Self, Self::Error> {
        Ok(Keybind(
            egui::Key::from_name(&value.0).context("invalid key specification for keybind")?,
            f32::from_str(&value.1)?,
        ))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timeline {
    data: Vec<Moment>,
}

impl Timeline {
    pub fn example() -> Self {
        serde_json::from_str(
            r#"{
                "data": [["a","1.0"],["b","1.1"],["c","1.5"],["d","12.0"],]
            }"#,
        )
        .unwrap()
    }
}

#[test]
fn test_time() {
    println!("{:?}",Timeline::example().data)
}
