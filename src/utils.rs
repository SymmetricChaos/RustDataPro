use chrono::{DateTime, Datelike, Local, Timelike};
use egui::{Color32, InputState, Key, Response, RichText, Ui};
use std::collections::HashSet;

pub struct Prng {
    pub state: u64,
}

impl Prng {
    pub fn new(state: u64) -> Self {
        Self { state }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = (self.state).wrapping_mul(1664525).wrapping_add(1013904223);
        (self.state ^ (self.state >> 22)) >> (22 + (self.state >> 61))
    }

    pub fn shuffle<T>(&mut self, v: &mut Vec<T>) {
        for i in 0..v.len() {
            let swap_pos = self.next_u64() as usize % v.len();
            v.swap(i, swap_pos);
        }
    }
}

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

const LARGE_BUTTON_WIDTH: f32 = 110.0;
const LARGE_BUTTON_HEIGHT: f32 = 40.0;
pub trait DataProUiElements {
    fn large_button(&mut self, text: &'static str) -> Response;
    fn large_color_button(&mut self, text: &'static str, color: Color32) -> Response;
    fn large_green_button(&mut self, text: &'static str) -> Response;
    fn large_red_button(&mut self, text: &'static str) -> Response;
}

impl DataProUiElements for Ui {
    fn large_button(&mut self, text: &'static str) -> Response {
        self.add_sized(
            [LARGE_BUTTON_WIDTH, LARGE_BUTTON_HEIGHT],
            egui::Button::new(text),
        )
    }

    fn large_color_button(&mut self, text: &'static str, color: Color32) -> Response {
        self.add_sized(
            [LARGE_BUTTON_WIDTH, LARGE_BUTTON_HEIGHT],
            egui::Button::new(RichText::new(text).color(color)),
        )
    }

    fn large_green_button(&mut self, text: &'static str) -> Response {
        self.add_sized(
            [LARGE_BUTTON_WIDTH, LARGE_BUTTON_HEIGHT],
            egui::Button::new(RichText::new(text).color(Color32::GREEN)),
        )
    }

    fn large_red_button(&mut self, text: &'static str) -> Response {
        self.add_sized(
            [LARGE_BUTTON_WIDTH, LARGE_BUTTON_HEIGHT],
            egui::Button::new(RichText::new(text).color(Color32::RED)),
        )
    }
}
