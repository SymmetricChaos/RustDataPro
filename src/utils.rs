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

pub fn clicked_keys(input: &InputState, clicked: &mut HashSet<Key>) {
    clicked.clear();
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
                clicked.insert(*key);
            }
        }
    }
}
