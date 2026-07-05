use crate::data::Keybind;
use egui::{Key, Ui};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Counter {
    pub key: Option<Key>,
    pub description: String,
    pub counter: u32,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            key: Default::default(),
            description: String::new(),
            counter: 0,
        }
    }
}

impl Counter {
    /// Build a counter with an associated keybind.
    pub fn with_keybind(mut self, keybind: &Keybind) -> Self {
        self.key = Some(keybind.0);
        self.description = keybind.1.clone();
        self
    }

    /// Build a counter with a key.
    pub fn with_key(mut self, key: Key) -> Self {
        self.key = Some(key);
        self
    }

    /// Build a counter with a description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
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
        if !self.description.is_empty() {
            ui.label(&self.description);
        }
        if let Some(k) = &self.key {
            ui.label(k.name());
        }
        ui.monospace(format!("{:>3}", self.counter));
    }
}
