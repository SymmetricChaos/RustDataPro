use crate::{
    data_tracking::{counter::Counter, timer::Timer},
    ksf::Ksf,
    utils::date_time_string,
};
use chrono::Local;
use egui::{Color32, Key, RichText, Ui};
use egui_file_dialog::FileDialog;
use std::collections::VecDeque;

const MAX_DUR: usize = 20;
const MAX_FREQ: usize = 20;

macro_rules! record_keypress {
    ($self:ident, $key:expr) => {
        $self.keypresses.push($key);
        $self.keypresses_display.pop_front();
        $self.keypresses_display.push_back($key.name());
    };
}

pub struct Session {
    timers: [Timer; MAX_DUR],
    counters: [Counter; MAX_FREQ],
    session_timer: Timer,
    file_dialog: FileDialog,
    output_file_contents: String,
    keypresses: Vec<Key>,
    keypresses_display: VecDeque<&'static str>,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            session_timer: Timer::new(),
            timers: [
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
                Timer::new_split(),
            ],
            counters: [
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
                Counter::new(),
            ],
            file_dialog: FileDialog::new().default_file_name("SaveData.txt"),
            output_file_contents: String::new(),
            keypresses: Vec::new(),
            keypresses_display: VecDeque::from(["_"; 10]),
        }
    }
}

impl Session {
    fn reset_trackers(&mut self) {
        self.timers.iter_mut().for_each(|t| t.reset());
        self.counters.iter_mut().for_each(|c| c.reset());
        self.session_timer.reset();
        self.keypresses.clear();
        self.keypresses_display = VecDeque::from(["_"; 10]);
    }

    pub fn load_ksf(&mut self, ksf: &Ksf) {
        for (keybind, timer) in ksf.duration.iter().zip(self.timers.iter_mut()) {
            timer.key = Some(keybind.key);
            timer.description = Some(keybind.description.clone());
        }
        for (keybind, counter) in ksf.frequency.iter().zip(self.counters.iter_mut()) {
            counter.key = Some(keybind.key);
            counter.description = Some(keybind.description.clone());
        }
    }

    fn save_session(&mut self) {
        self.session_timer.stop();
        self.output_file_contents.clear();

        // Save session time information
        self.output_file_contents.push_str(&format!(
            "Start {}\nEnd {}\nDuration {}\n",
            date_time_string(self.session_timer.start_time),
            date_time_string(Local::now()),
            (Local::now() - self.session_timer.start_time).as_seconds_f32()
        ));
        self.output_file_contents.push('\n');
        // Save duration data
        for timer in self.timers.iter_mut() {
            timer.stop();
        }
        for timer in self.timers.iter_mut() {
            // Stop timers if they are running and update the total
            if let Some(description) = &timer.description {
                self.output_file_contents.push_str(&format!(
                    "{} {}\n",
                    description,
                    timer.saved_time.as_seconds_f32()
                ));
            }
        }
        self.output_file_contents.push('\n');
        // Save frequency data
        for counter in self.counters.iter() {
            if let Some(description) = &counter.description {
                self.output_file_contents
                    .push_str(&format!("{} {}\n", description, counter.counter));
            }
        }

        self.reset_trackers();

        // Open save dialog
        self.file_dialog.save_file();
    }

    pub fn view(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Session Controls");
            ui.horizontal(|ui| {
                if ui
                    .button(RichText::new("START").color(Color32::GREEN))
                    .clicked()
                {
                    self.session_timer.start();
                }
                if ui
                    .button(RichText::new("END").color(Color32::RED))
                    .clicked()
                {
                    self.save_session();
                }
            });

            ui.add_space(10.0);
            self.session_timer.view(ui);

            ui.add_space(10.0);
            self.file_dialog.update(ui.ctx());

            if let Some(path) = self.file_dialog.take_picked() {
                if std::fs::write(&path, &self.output_file_contents).is_ok() {
                    println!("Successfully saved to: {:?}", path);
                }
            }

            ui.heading("Timers");
            ui.add_enabled_ui(self.session_timer.active, |ui| {
                egui::Grid::new("timer_grid").striped(true).show(ui, |ui| {
                    ui.label("Description");
                    ui.label("Key");
                    ui.label("Current");
                    ui.label("Total");
                    ui.end_row();

                    for timer in self.timers.iter_mut() {
                        if let Some(key) = timer.key {
                            if self.session_timer.active {
                                ui.ctx().input(|i| {
                                    if i.num_presses(key) > 0 {
                                        timer.toggle();
                                        record_keypress!(self, key);
                                    }
                                });
                            }

                            timer.view(ui);
                            ui.end_row();
                        }
                    }
                });
            });
            ui.add_space(10.0);

            ui.heading("Counters");
            ui.add_enabled_ui(self.session_timer.active, |ui| {
                egui::Grid::new("counter_grid")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Description");
                        ui.label("Key");
                        ui.label("Total");
                        ui.end_row();

                        for counter in self.counters.iter_mut() {
                            if let Some(key) = counter.key {
                                if self.session_timer.active {
                                    ui.ctx().input(|i| {
                                        if i.num_presses(key) > 0 {
                                            counter.inc();
                                            record_keypress!(self, key);
                                        }
                                    });
                                }
                                counter.view(ui);
                                ui.end_row();
                            }
                        }
                    });
            });

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    for k in self.keypresses_display.iter() {
                        ui.monospace(*k);
                    }
                });
            });
        });
    }
}
