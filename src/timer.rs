use chrono::{DateTime, Duration, Local};
use egui::Ui;
use crate::ksf::Keybind;

#[derive(Debug,Clone)]
pub struct Timer {
    keybind: Option<Keybind>,
    description: Option<String>,
    start_time: DateTime<Local>,
    total_time: Option<Duration>,
    active: bool
}

impl Timer {
    pub fn new() -> Self {
        Self {
            keybind: None,
            description: None,
            start_time: Local::now(),
            total_time: None,
            active: false,
        }
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
            if let Some(tt) = &mut self.total_time {
                *tt = Duration::zero();
            }
        }
    }

    pub fn stop(&mut self) {
        if self.active {
            self.active = false;
            if let Some(tt) = &mut self.total_time {
                *tt += Local::now() - self.start_time;
            }
        }
    }

    pub fn view(&mut self, ui: &mut Ui) {
        if self.active {
            ui.request_repaint();
            if let Some(kb) = &self.keybind {
                ui.label(&kb.description);
                ui.label(kb.key.name());
            }
            if let Some(tt) = self.total_time {
                ui.label(format!("{:.1}", tt.as_seconds_f32()));
            }
            ui.label(format!(
                "{:.1}",
                (Local::now() - self.start_time).as_seconds_f32()
            ));
        } else {
            if let Some(kb) = &self.keybind {
                ui.label(&kb.description);
                ui.label(kb.key.name());
            }
            if let Some(tt) = self.total_time {
                ui.label(format!("{:.1}", tt.as_seconds_f32()));
            }
            ui.label("0.0");
        }
    }
}