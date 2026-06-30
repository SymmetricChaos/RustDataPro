use crate::ksf::Keybind;
use chrono::{DateTime, Duration, Local};
use egui::Ui;

#[derive(Debug, Clone)]
pub struct Timer {
    pub keybind: Option<Keybind>,
    pub start_time: DateTime<Local>,
    pub saved_time: Duration,
    pub active: bool,
    pub split: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            keybind: None,
            start_time: Local::now(),
            saved_time: Duration::zero(),
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
            self.saved_time += Local::now() - self.start_time;
        }
    }

    /// Stop if active and set total time to zero.
    pub fn reset(&mut self) {
        self.active = false;
        self.saved_time = Duration::zero();
    }

    /// The amount of time currently saved in seconds.
    pub fn saved_time(&self) -> f32 {
        self.saved_time.as_seconds_f32()
    }

    /// How long the timer has been running since it was last started.
    pub fn current_time(&self) -> f32 {
        (Local::now() - self.start_time).as_seconds_f32()
    }

    /// The total time recorded. Sum of .saved_time() and .current_time().
    pub fn total_time(&self) -> f32 {
        (Local::now() - self.start_time + self.saved_time).as_seconds_f32()
    }

    pub fn view(&mut self, ui: &mut Ui) {
        if let Some(kb) = &self.keybind {
            ui.label(&kb.description);
            ui.label(kb.key.name());
        }
        if self.active {
            ui.request_repaint();
            if self.split {
                ui.horizontal(|ui| {
                    ui.monospace(format!("{:6.2}", self.saved_time()));
                    ui.monospace(format!("{:6.2}", self.current_time()));
                });
            } else {
                ui.monospace(format!("{:6.2}", self.total_time()));
            }
        } else {
            if self.split {
                ui.horizontal(|ui| {
                    ui.monospace(format!("{:6.2}", self.saved_time()));
                    ui.monospace(format!("{:6.2}", 0.0));
                });
            } else {
                ui.monospace(format!("{:6.2}", self.saved_time()));
            }
        }
    }
}
