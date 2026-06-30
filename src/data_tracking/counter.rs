use egui::Ui;

use crate::ksf::Keybind;

pub struct Counter {
    pub keybind: Option<Keybind>,
    pub counter: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            keybind: None,
            counter: 0,
        }
    }

    pub fn with_keybind(mut self, keybind: Keybind) -> Self {
        self.keybind = Some(keybind);
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
        if let Some(kb) = &self.keybind {
            ui.label(&kb.description);
            ui.label(kb.key.name());
            ui.monospace(format!("{:>3}", self.counter));
        }
    }
}
