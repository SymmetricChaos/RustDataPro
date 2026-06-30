use crate::ksf::Keybind;
use chrono::{DateTime, Duration, Local};
use egui::Ui;

#[derive(Debug, Clone)]
pub struct Timer {
    pub keybind: Option<Keybind>,
    start_time: DateTime<Local>,
    pub total_time: Duration,
    pub active: bool,
    pub split: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            keybind: None,
            start_time: Local::now(),
            total_time: Duration::zero(),
            active: false,
            split: false,
        }
    }

    pub fn with_keybind(keybind: Keybind) -> Self {
        Self {
            keybind: Some(keybind),
            start_time: Local::now(),
            total_time: Duration::zero(),
            active: false,
            split: false,
        }
    }

    pub fn split(mut self) -> Self {
        self.split = true;
        self
    }

    pub fn toggle(&mut self) {
        if self.active {
            self.stop();
        } else {
            self.start();
        }
    }

    pub fn start(&mut self) {
        if !self.active {
            self.active = true;
            self.start_time = Local::now();
        }
    }

    pub fn stop(&mut self) {
        if self.active {
            self.active = false;
            self.total_time += Local::now() - self.start_time;
        }
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.total_time = Duration::zero();
    }

    pub fn view(&mut self, ui: &mut Ui) {
        if self.active {
            ui.request_repaint();
            if let Some(kb) = &self.keybind {
                ui.label(&kb.description);
                ui.label(kb.key.name());
            }
            if self.split {
                ui.horizontal(|ui| {
                    ui.monospace(format!("{:6.2}", (self.total_time).as_seconds_f32()));
                    ui.monospace(format!(
                        "{:6.2}",
                        (Local::now() - self.start_time).as_seconds_f32()
                    ));
                });
            } else {
                ui.monospace(format!(
                    "{:6.2}",
                    (Local::now() - self.start_time + self.total_time).as_seconds_f32()
                ));
            }
        } else {
            if let Some(kb) = &self.keybind {
                ui.label(&kb.description);
                ui.label(kb.key.name());
            }
            if self.split {
                ui.horizontal(|ui| {
                    ui.monospace(format!("{:6.2}", (self.total_time).as_seconds_f32()));
                    ui.monospace(format!("{:6.2}", Duration::zero().as_seconds_f32()));
                });
            } else {
                ui.monospace(format!("{:6.2}", (self.total_time).as_seconds_f32()));
            }
        }
    }
}
