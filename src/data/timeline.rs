use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::Read,
    ops::{Deref, DerefMut},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Timeline(Vec<(String, f32)>);

impl Default for Timeline {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Deref for Timeline {
    type Target = Vec<(String, f32)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Timeline {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Timeline {
    pub fn example() -> Self {
        serde_json::from_str(
            r#"{
                "timeline": [["Tab",0.0],["Num4",1.1079073],["Num4",3.5078146],["Num4",3.6745386],["V",6.391463]]
            }"#,
        )
        .unwrap()
    }

    pub fn from_file(file_path: &Path) -> Result<Self> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self).context("unable to convert timeline data to json")
    }
}

#[test]
fn test_time() {
    println!("{:?}", Timeline::example().0)
}
