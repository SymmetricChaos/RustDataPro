use chrono::{DateTime, Duration, Local};
use egui::{Color32, Key, RichText, Ui};

macro_rules! timer_format {
    () => {
        "{:6.2}"
    };
}

macro_rules! timer_display_on {
    ($timer:expr) => {
        RichText::new(format!(timer_format!(), $timer)).color(Color32::YELLOW)
    };
}

macro_rules! timer_display_off {
    ($timer:expr) => {
        RichText::new(format!(timer_format!(), $timer))
    };
}

macro_rules! bout_display {
    ($bouts:expr) => {
        RichText::new(format!("{:>2}", $bouts))
    };
}

#[derive(Debug, Clone)]
pub struct Timer {
    pub key: Option<Key>,
    pub description: Option<String>,
    pub start_time: DateTime<Local>,
    pub saved_time: Duration,
    pub bouts: u32,
    pub show_bouts: bool,
    pub active: bool,
    pub split: bool,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            key: None,
            description: None,
            start_time: Local::now(),
            saved_time: Duration::zero(),
            bouts: 0,
            show_bouts: false,
            active: false,
            split: false,
        }
    }

    pub fn new_split() -> Self {
        Self {
            key: None,
            description: None,
            start_time: Local::now(),
            saved_time: Duration::zero(),
            bouts: 0,
            show_bouts: false,
            active: false,
            split: true,
        }
    }

    /// Build a timer with a keybind.
    pub fn with_key(mut self, key: Key) -> Self {
        self.key = Some(key);
        self
    }

    /// Build a timer with a description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Build a timer with a description.
    pub fn with_bouts(mut self) -> Self {
        self.show_bouts = true;
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
            self.bouts += 1;
        }
    }

    /// If active stop without updating the saved time.
    pub fn unstart(&mut self) {
        if self.active {
            self.active = false;
            self.bouts = self.bouts.saturating_sub(1); // prevents potential overflow
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
        self.bouts = 0;
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

    fn view_split(&mut self, ui: &mut Ui) {
        if self.active {
            ui.request_repaint();
            ui.horizontal(|ui| {
                ui.monospace(timer_display_on!(self.saved_time()));
                ui.monospace(timer_display_on!(self.current_time()));
            });
            if self.show_bouts {
                ui.monospace(bout_display!(self.bouts));
            }
        } else {
            ui.horizontal(|ui| {
                ui.monospace(timer_display_off!(self.saved_time()));
                ui.monospace(timer_display_off!(0.0));
            });
            if self.show_bouts {
                ui.monospace(bout_display!(self.bouts));
            }
        }
    }

    fn view_unsplit(&mut self, ui: &mut Ui) {
        if self.active {
            ui.request_repaint();
            ui.monospace(timer_display_on!(self.total_time()));
            if self.show_bouts {
                ui.monospace(bout_display!(self.bouts));
            }
        } else {
            ui.monospace(timer_display_off!(self.saved_time()));
            if self.show_bouts {
                ui.monospace(bout_display!(self.bouts));
            }
        }
    }

    pub fn view(&mut self, ui: &mut Ui) {
        if let Some(des) = &self.description {
            ui.label(des);
        }
        if let Some(k) = &self.key {
            ui.label(k.name());
        }
        if self.split {
            self.view_split(ui);
        } else {
            self.view_unsplit(ui);
        }
    }
}
