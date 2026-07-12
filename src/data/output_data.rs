use crate::data::{
    ClientData,
    DataType::{self},
    KsfData, SessionData,
    timeline::Timeline,
};
use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputData {
    pub datetime: String,
    pub client: ClientData,
    pub session: SessionData,
    pub session_duration: f32,
    pub frequency: IndexMap<String, u32>,
    pub duration: IndexMap<String, (u32, f32)>,
    pub timeline: Timeline,
    pub ksf: KsfData,
}

impl OutputData {
    pub fn session_number(&self) -> u32 {
        self.client.current_session
    }

    pub fn data_type(&self) -> DataType {
        self.session.data_type
    }

    pub fn client_initials(&self) -> String {
        self.client.initials()
    }

    pub fn from_file(file_path: &Path) -> Result<Self> {
        let mut file = File::open(&file_path)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(serde_json::from_str(&s)?)
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(&self).context("unable to convert session data to json")
    }
}

#[test]
fn create_test_data() {
    use egui::Key;
    use rand::{RngExt, make_rng, rngs::StdRng};
    let mut rng: StdRng = make_rng();

    for session in 11..15 {
        let mut client_data = ClientData::default();
        client_data.current_session = session;

        let mut session_data = SessionData::default();
        session_data.data_type = DataType::Primary;

        let ksf = KsfData::default();
        let mut frequency: IndexMap<String, u32> = IndexMap::new();
        for (_k, desc) in ksf.frequency.iter() {
            let n: u32 = rng.random_range(0..50);
            frequency.insert(desc.clone(), n);
        }
        let mut duration: IndexMap<String, (u32, f32)> = IndexMap::new();
        for (_k, desc) in ksf.duration.iter() {
            let n: u32 = rng.random_range(..50);
            let f: f32 = rng.random::<f32>() * 50.0;
            duration.insert(desc.clone(), (n, f));
        }

        let mut timeline = Timeline::default();
        let mut session_time = 0.0;
        timeline.push((Key::Tab, session_time));

        for _ in 0..100 {
            session_time = session_time + rng.random::<f32>() * 2.0;
            if rng.random_bool(0.9) {
                timeline.push((Key::M, session_time));
            }
        }

        timeline.push((Key::Escape, session_time));
        let prim = OutputData {
            datetime: String::from("TEST FILE"),
            client: client_data.clone(),
            session: session_data.clone(),
            session_duration: session_time,
            frequency: frequency.clone(),
            duration: duration.clone(),
            timeline: timeline.clone(),
            ksf: ksf.clone(),
        };

        let file = File::create(&format!(
            "{}{}_{}_raw.txt",
            prim.client.initials(),
            prim.client.current_session,
            prim.session.data_type
        ))
        .unwrap();
        let mut writer = std::io::BufWriter::new(file);
        std::io::Write::write_all(&mut writer, prim.to_json().unwrap().as_bytes()).unwrap();
        std::io::Write::flush(&mut writer).unwrap();

        // Jitter the timing for the keypresses
        session_data.data_type = DataType::Reliability;
        for (_k, t) in timeline.iter_mut() {
            *t += (rng.random::<f32>() - 0.5) * 0.1;
        }
        // Jitter the durations
        for (_k, desc) in ksf.duration.iter() {
            let f: f32 = (rng.random::<f32>() - 0.5) * 5.0;
            duration.get_mut(desc).unwrap().1 += f;
        }
        // Jitter the jitter the counds
        for (_k, desc) in ksf.frequency.iter() {
            let f: u32 = rng.random_range(..5);
            if rng.random_bool(0.5) {
                *frequency.get_mut(desc).unwrap() += f;
            } else {
                *frequency.get_mut(desc).unwrap() =
                    frequency.get_mut(desc).unwrap().saturating_sub(f);
            }
        }
        for (_k, desc) in ksf.duration.iter() {
            let f: u32 = rng.random_range(..5);
            if rng.random_bool(0.5) {
                duration.get_mut(desc).unwrap().0 += f;
            } else {
                duration.get_mut(desc).unwrap().0 =
                    duration.get_mut(desc).unwrap().0.saturating_sub(f);
            }
        }

        let prim = OutputData {
            datetime: String::from("TEST FILE"),
            client: client_data.clone(),
            session: session_data.clone(),
            session_duration: session_time,
            frequency: frequency.clone(),
            duration: duration.clone(),
            timeline: timeline,
            ksf: ksf.clone(),
        };

        let file = File::create(&format!(
            "{}{}_{}_raw.txt",
            prim.client.initials(),
            prim.client.current_session,
            prim.session.data_type
        ))
        .unwrap();
        let mut writer = std::io::BufWriter::new(file);
        std::io::Write::write_all(&mut writer, prim.to_json().unwrap().as_bytes()).unwrap();
        std::io::Write::flush(&mut writer).unwrap();
    }
}
