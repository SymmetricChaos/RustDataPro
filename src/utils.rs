use std::collections::HashSet;

use chrono::{DateTime, Datelike, Local, Timelike};
use egui::{Color32, InputState, Key, Response, RichText, Ui};

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
pub struct ClickedKeys(HashSet<Key>);

impl ClickedKeys {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn contains(&self, key: &Key) -> bool {
        self.0.contains(key)
    }

    pub fn update(&mut self, input: &InputState) {
        self.clear();

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
                    self.0.insert(*key);
                }
            }
        }
    }
}

pub trait DataProUiElements {
    fn large_green_button(&mut self, text: &'static str) -> Response;
    fn large_red_button(&mut self, text: &'static str) -> Response;
}

impl DataProUiElements for Ui {
    fn large_green_button(&mut self, text: &'static str) -> Response {
        self.add_sized(
            [110.0, 40.0],
            egui::Button::new(RichText::new(text).color(Color32::GREEN)),
        )
    }
    fn large_red_button(&mut self, text: &'static str) -> Response {
        self.add_sized(
            [110.0, 40.0],
            egui::Button::new(RichText::new(text).color(Color32::RED)),
        )
    }
}
