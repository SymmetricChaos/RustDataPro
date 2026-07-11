use egui::{Color32, RichText, Ui};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    time::{Duration, Instant},
};

/// Need to use a macro to pass around a string literal
macro_rules! timer_format {
    () => {
        "{:7.2}"
    };
}

macro_rules! timer_display_yellow {
    ($ui:ident, $timer:expr) => {
        $ui.monospace(RichText::new(format!(timer_format!(), $timer)).color(Color32::YELLOW))
    };
}

macro_rules! timer_display_default {
    ($ui:ident, $timer:expr) => {
        $ui.monospace(RichText::new(format!(timer_format!(), $timer)))
    };
}

macro_rules! timer_display_red {
    ($ui:ident, $timer:expr) => {
        $ui.monospace(RichText::new(format!(timer_format!(), $timer)).color(Color32::RED))
    };
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerStatus {
    NotStarted,
    Active,
    Stopped,
    Paused,
}

impl Default for TimerStatus {
    fn default() -> Self {
        Self::NotStarted
    }
}

impl Display for TimerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerStatus::NotStarted => write!(f, "NotStarted"),
            TimerStatus::Active => write!(f, "Active"),
            TimerStatus::Stopped => write!(f, "Stopped"),
            TimerStatus::Paused => write!(f, "Paused"),
        }
    }
}

impl TimerStatus {
    pub fn was_started(&self) -> bool {
        *self != TimerStatus::NotStarted
    }

    pub fn is_active(&self) -> bool {
        *self == TimerStatus::Active
    }

    pub fn is_paused(&self) -> bool {
        *self == TimerStatus::Paused
    }

    pub fn is_stopped(&self) -> bool {
        *self == TimerStatus::Stopped
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Timer {
    start_time: Instant,
    saved_time: Duration,
    stashed_time: Duration,
    pub countdown_from: f32,
    status: TimerStatus,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            saved_time: Duration::ZERO,
            stashed_time: Duration::ZERO,
            countdown_from: 30.0,
            status: TimerStatus::NotStarted,
        }
    }
}

impl Timer {
    /// Set status to Active and set the start_time to to Local::now().
    pub fn start(&mut self) {
        if !self.status.is_active() {
            self.status = TimerStatus::Active;
            self.start_time = Instant::now();
        }
    }

    /// Set status to Paused and update the stashed time.
    pub fn pause(&mut self) {
        if self.is_active() {
            self.status = TimerStatus::Paused;
            self.stashed_time += self.start_time.elapsed();
        }
    }

    /// In Active or Paused, set status to Stopped, update the saved time, and clear the stashed time.
    pub fn stop(&mut self) {
        if self.is_active() || self.is_paused() {
            self.status = TimerStatus::Stopped;
            self.stashed_time += self.start_time.elapsed();
            self.saved_time += self.stashed_time;
            self.stashed_time = Duration::ZERO;
        }
    }

    /// Set status to Stopped without updating the stashed time.
    pub fn unstart(&mut self) {
        if self.status.is_active() {
            self.status = TimerStatus::Stopped;
        }
    }

    /// Pause or unpause. Does nothing if the timer is Stopped.
    pub fn toggle(&mut self) {
        match self.status {
            TimerStatus::Active => self.pause(),
            TimerStatus::Paused | TimerStatus::NotStarted => self.start(),
            TimerStatus::Stopped => (),
        }
    }

    /// Stop or start. Does nothing if the timer is Paused.
    pub fn stop_start(&mut self) {
        match self.status {
            TimerStatus::Active => self.stop(),
            TimerStatus::Stopped | TimerStatus::NotStarted => self.start(),
            TimerStatus::Paused => (),
        }
    }

    /// Reset all values except countdown_from.
    pub fn reset(&mut self) {
        *self = Self {
            countdown_from: self.countdown_from,
            ..Default::default()
        };
    }

    pub fn status(&self) -> TimerStatus {
        self.status
    }

    /// Is the timer currently in the Active state.
    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }

    /// Is the timer currently in the Paused state.
    pub fn is_paused(&self) -> bool {
        self.status.is_paused()
    }

    /// Is the timer currently in the Stopped state.
    pub fn is_stopped(&self) -> bool {
        self.status.is_stopped()
    }

    /// Has the timer been started at least once since it was last reset?
    pub fn was_started(&self) -> bool {
        self.status.was_started()
    }

    // /// Time since the timer was last started in seconds.
    // pub fn elapsed_time(&self) -> f32 {
    //     self.start_time.elapsed().as_secs_f32()
    // }

    /// The amount of time currently saved in seconds.
    pub fn saved_time(&self) -> f32 {
        self.saved_time.as_secs_f32()
    }

    /// How long the timer has been running since it was last started, ignoring time paused.
    pub fn current_time(&self) -> f32 {
        match self.status {
            TimerStatus::Active => (self.start_time.elapsed() + self.stashed_time).as_secs_f32(),
            TimerStatus::Stopped => self.stashed_time(),
            TimerStatus::Paused => self.stashed_time(),
            TimerStatus::NotStarted => 0.0,
        }
    }

    /// Amount of time stashed during the current pause in seconds.
    pub fn stashed_time(&self) -> f32 {
        self.stashed_time.as_secs_f32()
    }

    /// The total time recorded in seconds. Sum of .saved_time() and .current_time().
    pub fn total_time(&self) -> f32 {
        self.saved_time() + self.current_time()
    }

    /// Time remaining in the countdown. May be negative.
    pub fn remaining_time(&self) -> f32 {
        self.countdown_from - self.total_time()
    }
}

pub fn view_simple_timer(ui: &mut Ui, timer: &Timer) {
    match timer.status {
        TimerStatus::Active => {
            ui.request_repaint();
            timer_display_yellow!(ui, timer.total_time());
        }
        TimerStatus::Stopped => {
            timer_display_yellow!(ui, timer.saved_time());
        }
        TimerStatus::Paused => {
            timer_display_yellow!(ui, timer.stashed_time());
        }
        TimerStatus::NotStarted => {
            timer_display_default!(ui, 0.0);
        }
    }
}

pub fn view_simple_countdown_timer(ui: &mut Ui, timer: &Timer) {
    match timer.status {
        TimerStatus::Active => {
            ui.request_repaint();
            let t = timer.remaining_time();
            if t.is_sign_positive() {
                timer_display_yellow!(ui, t);
            } else {
                timer_display_red!(ui, -t);
            }
        }
        TimerStatus::Stopped => {
            let t = timer.countdown_from - timer.saved_time();
            if t.is_sign_positive() {
                timer_display_yellow!(ui, t);
            } else {
                timer_display_red!(ui, -t);
            }
        }
        TimerStatus::Paused => {
            let t = timer.countdown_from - timer.stashed_time();
            if t.is_sign_positive() {
                timer_display_yellow!(ui, t);
            } else {
                timer_display_red!(ui, -t);
            }
        }
        TimerStatus::NotStarted => {
            timer_display_default!(ui, timer.countdown_from);
        }
    }
}
