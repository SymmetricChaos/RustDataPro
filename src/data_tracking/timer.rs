use crate::ksf::Keybind;
use chrono::{DateTime, Duration, Local};
use egui::Ui;

#[derive(Debug, Clone)]
pub struct Timer {
    pub keybind: Option<Keybind>,
    pub start_time: DateTime<Local>,
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

    /// Build a timer with a keybind.
    pub fn with_keybind(mut self, keybind: Keybind) -> Self {
        self.keybind = Some(keybind);
        self
    }

    /// Buold a timer with the total and current time on split displays.
    pub fn with_split(mut self) -> Self {
        self.split = true;
        self
    }

    /// Switch between active and inactive.
    pub fn toggle(&mut self) {
        if self.active {
            self.stop();
        } else {
            self.start();
        }
    }

    /// Start if inactive. Otherwise do nothing.
    pub fn start(&mut self) {
        if !self.active {
            self.active = true;
            self.start_time = Local::now();
        }
    }

    /// Stop if active. Otherwise do nothing.
    pub fn stop(&mut self) {
        if self.active {
            self.active = false;
            self.total_time += Local::now() - self.start_time;
        }
    }

    /// Stop if active and set total time to zero.
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
