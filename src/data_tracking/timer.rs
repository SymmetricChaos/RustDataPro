use crate::data::Keybind;
use egui::{Color32, Key, RichText, Ui};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

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

macro_rules! timer_display_negative {
    ($ui:ident, $timer:expr) => {
        $ui.monospace(RichText::new(format!(timer_format!(), $timer)).color(Color32::RED))
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
    pub countdown_from: f32,
    pub bouts: u32,
    pub status: TimerStatus,
    pub key: Option<Key>,
    pub description: String,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            saved_time: Duration::ZERO,
            countdown_from: 5.0, // DO NOT LEAVE AS DEFAULT
            bouts: 0,
            status: TimerStatus::Stopped,
            key: None,
            description: String::new(),
        }
    }
}

impl Timer {
    /// Build a timer with an associated keybind.
    pub fn with_keybind(mut self, keybind: &Keybind) -> Self {
        self.key = Some(keybind.0);
        self.description = keybind.1.clone();
        self
    }

    /// Build a timer with an associated key.
    pub fn with_key(mut self, key: Key) -> Self {
        self.key = Some(key);
        self
    }

    /// Build a timer with a description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
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
                self.saved_time += self.start_time.elapsed();
            }
            TimerStatus::Stopped => (),
            TimerStatus::Paused => {
                self.status = TimerStatus::Active;
                self.start_time = Instant::now();
            }
        }
    }

    /// If active set status to Paused. Does not update bouts.
    pub fn pause(&mut self) {
        if self.status.is_active() {
            self.status = TimerStatus::Paused;
            self.saved_time += self.start_time.elapsed();
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
            self.saved_time += self.start_time.elapsed();
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

    /// How long the timer has been running since it was last started in seconds.
    pub fn current_time(&self) -> f32 {
        (Instant::now() - self.start_time).as_secs_f32()
    }

    /// The total time recorded in seconds. Sum of .saved_time() and .current_time().
    pub fn total_time(&self) -> f32 {
        (self.start_time.elapsed() + self.saved_time).as_secs_f32()
    }

    /// Time remaining in the countdown.
    pub fn remaining_time(&self) -> f32 {
        self.countdown_from - self.total_time()
    }

    pub fn view_split(&mut self, ui: &mut Ui) {
        ui.monospace(&self.description);
        if let Some(key) = self.key {
            ui.monospace(key.name());
        }
        if self.status.is_active() {
            ui.request_repaint();
            timer_display_on!(ui, self.saved_time());
            timer_display_on!(ui, self.current_time());
        } else {
            timer_display_off!(ui, self.saved_time());
            timer_display_off!(ui, 0.0);
        }
        ui.monospace(self.bouts.to_string());
    }

    pub fn view_unsplit(&mut self, ui: &mut Ui) {
        if self.status.is_active() {
            ui.request_repaint();
            timer_display_on!(ui, self.total_time());
        } else {
            timer_display_off!(ui, self.saved_time());
        }
    }

    pub fn view_countdown(&mut self, ui: &mut Ui) {
        ui.add(egui::DragValue::new(&mut self.countdown_from));
        if self.status.is_active() {
            ui.request_repaint();
            let t = self.remaining_time();
            if t.is_sign_positive() {
                timer_display_on!(ui, self.remaining_time());
            } else {
                timer_display_negative!(ui, self.remaining_time());
            }
        } else {
            timer_display_off!(ui, self.countdown_from - self.saved_time());
        }
    }
}
