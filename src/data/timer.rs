use egui::{Color32, Key, RichText, Ui};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

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
    pub stashed_time: Duration,
    pub bouts: u32,
    pub status: TimerStatus,
    started: bool,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            saved_time: Duration::ZERO,
            countdown_from: 5.0, // DO NOT LEAVE AS DEFAULT
            stashed_time: Duration::ZERO,
            bouts: 0,
            status: TimerStatus::Stopped,
            started: false,
        }
    }
}

impl Timer {
    /// Stop or start. Does nothing if the timer is Paused.
    pub fn toggle(&mut self) {
        match self.status {
            TimerStatus::Active => self.stop(),
            TimerStatus::Stopped => self.start(),
            TimerStatus::Paused => (),
        }
    }

    /// Pause or unpause. Does nothing is the timer is Stopped. Does not update bouts.
    pub fn toggle_pause(&mut self) {
        match self.status {
            TimerStatus::Active => self.pause(),
            TimerStatus::Paused => self.unpause(),
            TimerStatus::Stopped => (),
        }
    }

    /// This is needed for session page so that both the saved time and current time can be paused at once.
    pub fn toggle_pause_partial(&mut self) {
        match self.status {
            TimerStatus::Active => {
                self.status = TimerStatus::Paused;
                self.stashed_time += Instant::now() - self.start_time;
            }
            TimerStatus::Paused => {
                self.status = TimerStatus::Active;
                self.start_time = Instant::now();
            }
            TimerStatus::Stopped => (),
        }
    }

    /// If Active set status to Paused. Does not update bouts.
    pub fn pause(&mut self) {
        if self.status.is_active() {
            self.status = TimerStatus::Paused;
            self.saved_time += self.start_time.elapsed();
        }
    }

    /// If Paused set status to Active.
    pub fn unpause(&mut self) {
        if self.status.is_paused() {
            self.status = TimerStatus::Active;
            self.start_time = Instant::now();
        }
    }

    /// If inactive set status to Active, set the start_time to to Local::now(), and increment bouts by 1. Otherwise do nothing.
    pub fn start(&mut self) {
        if !self.status.is_active() {
            self.status = TimerStatus::Active;
            self.start_time = Instant::now();
            self.bouts += 1;
            self.started = true;
        }
    }

    /// If Active, decrement bouts by 1 and Stop without updating the saved time. Otherwise do nothing.
    pub fn unstart(&mut self) {
        if self.status.is_active() {
            self.status = TimerStatus::Stopped;
            self.bouts = self.bouts.saturating_sub(1); // prevents potential overflow
        }
    }

    /// If Active or Paused, Stop and update the saved time. Otherwise do nothing.
    pub fn stop(&mut self) {
        if !self.status.is_stopped() {
            self.status = TimerStatus::Stopped;
            self.saved_time += self.start_time.elapsed() + self.stashed_time;
            self.stashed_time = Duration::ZERO;
        }
    }

    /// Reset all values except countdown_from.
    pub fn reset(&mut self) {
        *self = Self {
            countdown_from: self.countdown_from,
            ..Default::default()
        };
    }

    /// Is the timer currently in the Active state.
    pub fn is_active(&self) -> bool {
        self.status.is_active()
    }

    /// Has the timer been started at least once since it was last reset?
    pub fn was_started(&self) -> bool {
        self.started
    }

    /// Time since the timer was last started
    pub fn elapsed_time(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    /// The amount of time currently saved in seconds.
    pub fn saved_time(&self) -> f32 {
        self.saved_time.as_secs_f32()
    }

    /// How long the timer has been running since it was last started in seconds. Ignores time spent paused.
    pub fn current_time(&self) -> f32 {
        (Instant::now() - self.start_time + self.stashed_time).as_secs_f32()
    }

    /// Amount of time stashed during pauses in seconds.
    pub fn stashed_time(&self) -> f32 {
        self.stashed_time.as_secs_f32()
    }

    /// The total time recorded in seconds. Sum of .saved_time() and .current_time().
    pub fn total_time(&self) -> f32 {
        (self.start_time.elapsed() + self.saved_time).as_secs_f32()
    }

    /// Time remaining in the countdown. May be negative.
    pub fn remaining_time(&self) -> f32 {
        self.countdown_from - self.total_time()
    }
}

pub fn view_simple_timer(ui: &mut Ui, timer: &mut Timer) {
    match timer.status {
        TimerStatus::Active => {
            ui.request_repaint();
            timer_display_yellow!(ui, timer.total_time());
        }
        TimerStatus::Stopped => {
            timer_display_default!(ui, timer.saved_time());
        }
        TimerStatus::Paused => {
            timer_display_yellow!(ui, timer.saved_time());
        }
    }
}

pub fn view_simple_countdown_timer(ui: &mut Ui, timer: &mut Timer) {
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
        TimerStatus::Stopped | TimerStatus::Paused => {
            let t = timer.countdown_from - timer.saved_time();
            if timer.started {
                if t.is_sign_positive() {
                    timer_display_yellow!(ui, t);
                } else {
                    timer_display_red!(ui, -t);
                }
            } else {
                timer_display_default!(ui, t.abs());
            }
        }
    }
}

pub fn view_session_page_timer(ui: &mut Ui, timer: &mut Timer, key: &Key, description: &String) {
    ui.monospace(description);
    ui.monospace(key.name());

    match timer.status {
        TimerStatus::Active => {
            ui.request_repaint();
            timer_display_yellow!(ui, timer.saved_time());
            timer_display_yellow!(ui, timer.current_time());
            ui.monospace(RichText::new(timer.bouts.to_string()).color(Color32::YELLOW));
        }
        TimerStatus::Stopped => {
            timer_display_default!(ui, timer.saved_time());
            timer_display_default!(ui, 0.0);
            ui.monospace(timer.bouts.to_string());
        }
        TimerStatus::Paused => {
            timer_display_yellow!(ui, timer.saved_time());
            timer_display_yellow!(ui, timer.stashed_time());
            ui.monospace(RichText::new(timer.bouts.to_string()).color(Color32::YELLOW));
        }
    }
}
