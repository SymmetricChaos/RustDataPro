use crate::data::{
    ClientData,
    DataType::{self},
    KsfData, SessionData,
    timeline::Timeline,
};
use anyhow::{Context, Result};
use egui::Key;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::Path};

/// Output of a single session. Includes the Client and Session data along with the recorded keypresses and times, and the KSF to translate those.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutputData {
    pub datetime: String,
    pub client: ClientData,
    pub session: SessionData,
    pub session_duration: f32,
    pub frequency: IndexMap<Key, u32>,
    pub duration: IndexMap<Key, (u32, f32)>,
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
    use rand::seq::IndexedRandom;
    use rand::{RngExt, make_rng, rngs::StdRng};

    let mut rng: StdRng = make_rng();

    for session in 11..16 {
        let mut client_data = ClientData::default();
        client_data.current_session = session;

        let mut session_data = SessionData::default();
        session_data.data_type = DataType::Primary;

        let ksf = KsfData::default();
        let mut fkeys = Vec::new();

        let mut frequency: IndexMap<Key, u32> = IndexMap::new();
        for (k, _desc) in ksf.frequency.iter() {
            frequency.insert(*k, 0);
            fkeys.push(*k);
        }
        let mut duration: IndexMap<Key, (u32, f32)> = IndexMap::new();
        let mut dkeys = Vec::new();
        for (k, _desc) in ksf.duration.iter() {
            let n: u32 = rng.random_range(..50);
            let f: f32 = rng.random::<f32>() * 50.0;
            duration.insert(*k, (n, f));
            dkeys.push(*k);
        }

        let mut timeline = Timeline::default();
        let mut session_time = 0.0;
        timeline.push((Key::Tab, session_time));
        for _ in 0..200 {
            session_time = session_time + rng.random::<f32>() * 2.0;
            if rng.random_bool(0.9) {
                if rng.random_bool(0.5) {
                    let k = fkeys.choose(&mut rng).unwrap();
                    *frequency.get_mut(k).unwrap() += 1;
                    timeline.push((*k, session_time));
                } else {
                    let k = dkeys.choose(&mut rng).unwrap();
                    timeline.push((*k, session_time));
                };
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
            *t += (rng.random::<f32>() - 0.5) * 0.7;
        }
        // Jitter the durations
        for (k, _desc) in ksf.duration.iter() {
            let f: f32 = (rng.random::<f32>() - 0.5) * 5.0;
            let d = duration.get_mut(k).unwrap();
            d.1 += f;
            if d.1.is_sign_negative() {
                d.1 = 0.0;
            }
        }
        // Jitter the jitter the counts
        for (k, _desc) in ksf.frequency.iter() {
            let f: u32 = rng.random_range(..5);
            if rng.random_bool(0.5) {
                *frequency.get_mut(k).unwrap() += f;
            } else {
                *frequency.get_mut(k).unwrap() = frequency.get_mut(k).unwrap().saturating_sub(f);
            }
        }
        for (k, _desc) in ksf.duration.iter() {
            let f: u32 = rng.random_range(..5);
            if rng.random_bool(0.5) {
                duration.get_mut(k).unwrap().0 += f;
            } else {
                duration.get_mut(k).unwrap().0 = duration.get_mut(k).unwrap().0.saturating_sub(f);
            }
        }

        let prim = OutputData {
            datetime: String::from("TEST FILE"),
            client: client_data,
            session: session_data,
            session_duration: session_time,
            frequency: frequency,
            duration: duration,
            timeline: timeline,
            ksf: ksf,
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
