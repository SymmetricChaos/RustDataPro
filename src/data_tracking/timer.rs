use egui::{Color32, Key, RichText, Ui};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use crate::data::Keybind;

macro_rules! timer_format {
    () => {
        "{:6.2}"
    };
}

macro_rules! timer_display_on {
    ($ui:ident, $timer:expr) => {
        $ui.monospace(RichText::new(format!(timer_format!(), $timer)).color(Color32::YELLOW))
    };
}

macro_rules! timer_display_off {
    ($ui:ident, $timer:expr) => {
        $ui.monospace(RichText::new(format!(timer_format!(), $timer)))
    };
}

macro_rules! bout_display {
    ($ui:ident, $active:expr, $bouts:expr) => {
        if $active {
            $ui.centered_and_justified(|ui| ui.monospace(RichText::new(format!("{:>2}", $bouts))));
        }
    };
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerStatus {
    Active,
    Stopped,
    Paused,
}

impl Default for TimerStatus {
    fn default() -> Self {
        Self::Stopped
    }
}

impl TimerStatus {
    pub fn is_active(&self) -> bool {
        *self == TimerStatus::Active
    }

    pub fn is_inactive(&self) -> bool {
        *self != TimerStatus::Active
    }

    pub fn is_paused(&self) -> bool {
        *self == TimerStatus::Paused
    }

    pub fn is_stopped(&self) -> bool {
        *self == TimerStatus::Stopped
    }
}

#[derive(Debug, Clone)]
pub struct Timer {
    pub start_time: Instant,
    pub saved_time: Duration,
    pub bouts: u32,
    pub status: TimerStatus,
    pub key: Option<Key>,
    pub description: Option<String>,
    pub show_bouts: bool,
    pub show_split: bool,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            saved_time: Duration::ZERO,
            bouts: 0,
            status: TimerStatus::Stopped,
            key: None,
            description: None,
            show_bouts: false,
            show_split: false,
        }
    }
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            saved_time: Duration::ZERO,
            bouts: 0,
            status: TimerStatus::Stopped,
            key: None,
            description: None,
            show_bouts: false,
            show_split: false,
        }
    }

    pub fn new_splits_and_bouts() -> Self {
        Self {
            key: None,
            description: None,
            start_time: Instant::now(),
            saved_time: Duration::ZERO,
            bouts: 0,
            show_bouts: true,
            show_split: true,
            status: TimerStatus::Stopped,
        }
    }

    /// Build a timer with an associated keybind.
    pub fn with_keybind(mut self, keybind: &Keybind) -> Self {
        self.key = Some(keybind.0);
        self.description = Some(keybind.1.clone());
        self
    }

    /// Build a timer with an associated key.
    pub fn with_key(mut self, key: Key) -> Self {
        self.key = Some(key);
        self
    }

    /// Build a timer with a description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Build a timer that shows how many time it has been started.
    pub fn with_bouts(mut self) -> Self {
        self.show_bouts = true;
        self
    }

    /// Stop or start. Does nothing if the timer is Paused.
    pub fn toggle(&mut self) {
        match self.status {
            TimerStatus::Active => {
                self.stop();
            }
            TimerStatus::Stopped => self.start(),
            TimerStatus::Paused => (),
        }
    }

    /// Pause or unpause. Does nothing is the timer is Stopped. Does not update bouts.
    pub fn toggle_pause(&mut self) {
        match self.status {
            TimerStatus::Active => {
                self.status = TimerStatus::Paused;
                self.saved_time += Instant::now() - self.start_time;
            }
            TimerStatus::Stopped => (),
            TimerStatus::Paused => {
                self.status = TimerStatus::Active;
                self.start_time = Instant::now();
            }
        }
    }

    /// If inactive, start, set the start to to Local::now(), and increment bouts by 1. Otherwise do nothing.
    pub fn start(&mut self) {
        if self.status.is_inactive() {
            self.status = TimerStatus::Active;
            self.start_time = Instant::now();
            self.bouts += 1;
        }
    }

    /// If active, decrement bouts by 1 and stop without updating the saved time. Otherwise do nothing.
    pub fn unstart(&mut self) {
        if self.status.is_active() {
            self.status = TimerStatus::Stopped;
            self.bouts = self.bouts.saturating_sub(1); // prevents potential overflow
        }
    }

    /// If active or paused, stop and update the saved time. Otherwise do nothing.
    pub fn stop(&mut self) {
        if !self.status.is_stopped() {
            self.status = TimerStatus::Stopped;
            self.saved_time += Instant::now() - self.start_time;
        }
    }

    /// Set the status to stopped, set saved_time and bouts to zero.
    pub fn reset(&mut self) {
        self.status = TimerStatus::Stopped;
        self.saved_time = Duration::ZERO;
        self.bouts = 0;
    }

    /// The amount of time currently saved in seconds.
    pub fn saved_time(&self) -> f32 {
        self.saved_time.as_secs_f32()
    }

    pub fn saved_time_raw(&self) -> Duration {
        self.saved_time
    }

    /// How long the timer has been running since it was last started in seconds.
    pub fn current_time(&self) -> f32 {
        (Instant::now() - self.start_time).as_secs_f32()
    }

    pub fn current_time_raw(&self) -> Duration {
        Instant::now() - self.start_time
    }

    /// The total time recorded in seconds. Sum of .saved_time() and .current_time().
    pub fn total_time(&self) -> f32 {
        (Instant::now() - self.start_time + self.saved_time).as_secs_f32()
    }

    pub fn total_time_raw(&self) -> Duration {
        Instant::now() - self.start_time + self.saved_time
    }

    fn view_split(&mut self, ui: &mut Ui) {
        if self.status.is_active() {
            ui.request_repaint();
            timer_display_on!(ui, self.saved_time());
            timer_display_on!(ui, self.current_time());
        } else {
            timer_display_off!(ui, self.saved_time());
            timer_display_off!(ui, 0.0);
        }
    }

    fn view_unsplit(&mut self, ui: &mut Ui) {
        if self.status.is_active() {
            ui.request_repaint();
            timer_display_on!(ui, self.total_time());
        } else {
            timer_display_off!(ui, self.saved_time());
        }
    }

    pub fn view(&mut self, ui: &mut Ui) {
        if let Some(description) = &self.description {
            ui.label(description);
        }
        if let Some(key) = &self.key {
            ui.label(key.name());
        }
        if self.show_split {
            self.view_split(ui);
        } else {
            self.view_unsplit(ui);
        }
        bout_display!(ui, self.show_bouts, self.bouts);
    }
}
