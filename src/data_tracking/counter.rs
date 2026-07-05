use crate::data::Keybind;
use egui::{Key, Ui};

pub struct Counter {
    pub key: Option<Key>,
    pub description: Option<String>,
    pub counter: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            key: None,
            description: None,
            counter: 0,
        }
    }

    /// Build a counter with an associated keybind.
    pub fn with_keybind(mut self, keybind: &Keybind) -> Self {
        self.key = Some(keybind.0);
        self.description = Some(keybind.1.clone());
        self
    }

    /// Build a counter with a key.
    pub fn with_key(mut self, key: Key) -> Self {
        self.key = Some(key);
        self
    }

    /// Build a counter with a description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn inc(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn dec(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }

    pub fn reset(&mut self) {
        self.counter = 0;
    }

    pub fn view(&mut self, ui: &mut Ui) {
        if let Some(des) = &self.description {
            ui.label(des);
        }
        if let Some(k) = &self.key {
            ui.label(k.name());
        }
        ui.monospace(format!("{:>3}", self.counter));
    }
}
