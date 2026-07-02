use std::collections::HashSet;

use chrono::{DateTime, Datelike, Local, Timelike};
use egui::{InputState, Key};

pub fn date_time_string(dt: DateTime<Local>) -> String {
    format!(
        "{} {}/{}/{} {:02}:{:02}",
        dt.weekday(),
        dt.month(),
        dt.day(),
        dt.year(),
        dt.hour(),
        dt.minute(),
    )
}

/// Detect keys that have been pressed and ignore repeated events.
pub struct ClickedKeys {
    keys: HashSet<Key>,
}

impl ClickedKeys {
    pub fn new() -> Self {
        Self {
            keys: HashSet::new(),
        }
    }

    pub fn contains(&self, key: &Key) -> bool {
        self.keys.contains(key)
    }

    pub fn update(&mut self, input: &InputState) {
        self.keys.clear();
        for event in &input.events {
            if let egui::Event::Key {
                key,
                physical_key: _,
                pressed,
                repeat,
                modifiers: _,
            } = event
            {
                if *repeat {
                    continue;
                }
                if *pressed {
                    self.keys.insert(*key);
                }
            }
        }
    }
}
