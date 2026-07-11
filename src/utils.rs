use chrono::{DateTime, Datelike, Local, Timelike};
use egui::{Color32, InputState, Key, Response, RichText, Ui};
use std::{borrow::Cow, collections::HashSet, ffi::OsStr, path::Path};

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

// Round an f32 to one decimal
pub fn rounded_f32(n: f32) -> f32 {
    (n * 10.0).trunc() / 10.0
}

pub fn date_time_string(dt: &DateTime<Local>) -> String {
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

/// Quick time stamp as YearMonthDayHourMinute
pub fn time_stamp() -> String {
    let dt = Local::now();
    format!(
        "{}{:02}{:02}{:02}{:02}",
        dt.year(),
        dt.month(),
        dt.day(),
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

pub fn quick_file_name(pathbuf: &Path) -> Cow<'_, str> {
    pathbuf
        .file_name()
        .unwrap_or(&OsStr::new("INVALID FILE NAME"))
        .to_string_lossy()
}

const DEFAULT_LARGE_BUTTOM_DIMS: (f32, f32) = (120.0, 40.0);
pub trait DataProUiElements {
    fn large_button(&mut self, text: &'static str) -> Response;
    fn large_green_button(&mut self, text: &'static str) -> Response;
    fn green_button(&mut self, text: &'static str) -> Response;
    fn large_red_button(&mut self, text: &'static str) -> Response;
    fn red_button(&mut self, text: &'static str) -> Response;
    fn large_yellow_button(&mut self, text: &'static str) -> Response;
    fn yellow_button(&mut self, text: &'static str) -> Response;
    fn large_blue_button(&mut self, text: &'static str) -> Response;
    fn blue_button(&mut self, text: &'static str) -> Response;
}

macro_rules! simple_custom_button {
    ($ui:expr, $text:ident, $fill:expr) => {
        $ui.add(
            egui::Button::new(RichText::new($text).monospace().color(Color32::BLACK)).fill($fill),
        )
    };
    (large, $ui:expr, $text:ident, $fill:expr) => {
        $ui.add_sized(
            DEFAULT_LARGE_BUTTOM_DIMS,
            egui::Button::new(RichText::new($text).color(Color32::BLACK)).fill($fill),
        )
    };
}

impl DataProUiElements for Ui {
    fn large_button(&mut self, text: &'static str) -> Response {
        self.add_sized(DEFAULT_LARGE_BUTTOM_DIMS, egui::Button::new(text))
    }

    fn large_green_button(&mut self, text: &'static str) -> Response {
        simple_custom_button!(large, self, text, Color32::LIGHT_GREEN)
    }

    fn green_button(&mut self, text: &'static str) -> Response {
        simple_custom_button!(self, text, Color32::LIGHT_GREEN)
    }

    fn large_red_button(&mut self, text: &'static str) -> Response {
        simple_custom_button!(large, self, text, Color32::LIGHT_RED)
    }

    fn red_button(&mut self, text: &'static str) -> Response {
        simple_custom_button!(self, text, Color32::LIGHT_RED)
    }

    fn large_yellow_button(&mut self, text: &'static str) -> Response {
        simple_custom_button!(large, self, text, Color32::GOLD)
    }

    fn yellow_button(&mut self, text: &'static str) -> Response {
        simple_custom_button!(self, text, Color32::GOLD)
    }

    fn large_blue_button(&mut self, text: &'static str) -> Response {
        simple_custom_button!(large, self, text, Color32::LIGHT_BLUE)
    }

    fn blue_button(&mut self, text: &'static str) -> Response {
        simple_custom_button!(self, text, Color32::LIGHT_BLUE)
    }
}
