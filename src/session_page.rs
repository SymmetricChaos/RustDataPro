use crate::{
    data::{ksf::Ksf, session::SessionData},
    data_tracking::{counter::Counter, timer::Timer},
    utils::{ClickedKeys, date_time_string},
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

pub struct SessionPage {
    session_data: SessionData,
    ksf_name: String,
    timers: [Timer; MAX_DUR],
    counters: [Counter; MAX_FREQ],
    session_timer: Timer,
    file_dialog: FileDialog,
    output_file_contents: String,
    keypresses: Vec<Key>,
    keypresses_display: VecDeque<&'static str>,
    clicked_keys: ClickedKeys,
}

impl SessionPage {
    pub fn new() -> Self {
        Self {
            session_data: SessionData::default(),
            ksf_name: String::new(),
            session_timer: Timer::new(),
            timers: [
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
                Timer::new_splits_and_bouts(),
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
            clicked_keys: ClickedKeys::new(),
        }
    }

    fn reset(&mut self) {
        self.timers.iter_mut().for_each(|t| t.reset());
        self.counters.iter_mut().for_each(|c| c.reset());
        self.session_timer.reset();
        self.keypresses.clear();
        self.keypresses_display = VecDeque::from(["_"; 10]);
        self.ksf_name.clear();
        self.session_data = SessionData::blank();
        self.clicked_keys = ClickedKeys::new();
    }

    pub fn load(&mut self, ksf: &Ksf) {
        self.ksf_name = ksf.name.clone();
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
        // Stop all timers
        for timer in self.timers.iter_mut() {
            timer.stop();
        }
        self.session_timer.stop();

        // Reset the output
        self.output_file_contents.clear();

        // Save session information
        self.output_file_contents
            .push_str(&self.session_data.to_string());
        self.output_file_contents.push('\n');

        // Save session time information
        self.output_file_contents.push_str(&format!(
            "Start {}\nEnd {}\nDuration {}\n",
            date_time_string(self.session_timer.start_time),
            date_time_string(Local::now()),
            (Local::now() - self.session_timer.start_time).as_seconds_f32()
        ));
        self.output_file_contents.push('\n');

        // Save duration data
        self.output_file_contents.push_str("Duration Data\n");
        for timer in self.timers.iter_mut() {
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
        self.output_file_contents.push_str("Frequency Data\n");
        for counter in self.counters.iter() {
            if let Some(description) = &counter.description {
                self.output_file_contents
                    .push_str(&format!("{} {}\n", description, counter.counter));
            }
        }

        // Reset timers, counters, and session information
        self.reset();

        // Open save dialog
        self.file_dialog.save_file();
        if let Some(path) = self.file_dialog.take_picked() {
            if std::fs::write(&path, &self.output_file_contents).is_ok() {
                println!("Successfully saved to: {:?}", path);
            }
        }

        // *self.active_page.borrow_mut() = Page::About;
    }

    pub fn view(&mut self, ui: &mut Ui) {
        ui.ctx().input(|i| {
            self.clicked_keys.update(i);
        });

        if self.clicked_keys.contains(&Key::Tab) {
            self.session_timer.start();
        }
        if self.clicked_keys.contains(&Key::Escape) {
            self.save_session();
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!(
                            "Client: {} {}",
                            self.session_data.first_name, self.session_data.last_name
                        ));
                        ui.label(format!(
                            "Session Number: {}",
                            self.session_data.session_number
                        ));
                        ui.label(format!("KSF: {}", self.ksf_name))
                    });
                });
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("Assessment: {}", self.session_data.assessment));
                        ui.label(format!("Condition: {}", self.session_data.condition));
                        ui.label(format!("Data Type: {}", self.session_data.data_type));
                    });
                });
            });

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
            self.file_dialog.update(ui.ctx());

            ui.add_space(10.0);

            self.session_timer.view(ui);
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.label("Frequency Keys");
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
                                            if self.session_timer.active
                                                && self.clicked_keys.contains(&key)
                                            {
                                                counter.inc();
                                                record_keypress!(self, key);
                                            }
                                            counter.view(ui);
                                            ui.end_row();
                                        }
                                    }
                                });
                        });
                    })
                });
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.label("Duration Keys");
                        ui.add_enabled_ui(self.session_timer.active, |ui| {
                            egui::Grid::new("timer_grid").striped(true).show(ui, |ui| {
                                ui.label("Description");
                                ui.label("Key");
                                ui.label("Total");
                                ui.label("Current");
                                ui.label("Bouts");
                                ui.end_row();

                                for timer in self.timers.iter_mut() {
                                    if let Some(key) = timer.key {
                                        if self.session_timer.active
                                            && self.clicked_keys.contains(&key)
                                        {
                                            timer.toggle();
                                            record_keypress!(self, key);
                                        }
                                        timer.view(ui);
                                        ui.end_row();
                                    }
                                }
                            });
                        });
                    })
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
