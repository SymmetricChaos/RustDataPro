use crate::{
    data::{ksf::Ksf, session::SessionData},
    data_tracking::{counter::Counter, timer::Timer},
    pages::Page,
    utils::{ClickedKeys, date_time_string},
};
use chrono::Local;
use egui::{
    Color32,
    Key::{self},
    RichText, Ui,
};
use egui_file_dialog::FileDialog;
use itertools::Itertools;
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
    output_file_dialog: FileDialog,
    output_file_contents: String,
    keypresses: Vec<Key>,
    keypresses_display: VecDeque<&'static str>,
    clicked_keys: ClickedKeys,
}

impl SessionPage {
    pub fn new() -> Self {
        Self {
            session_data: SessionData::new(),
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
            output_file_dialog: FileDialog::new().default_file_name("SaveData.txt"),
            output_file_contents: String::new(),
            keypresses: Vec::new(),
            keypresses_display: VecDeque::from(["_"; 10]),
            clicked_keys: ClickedKeys::new(),
        }
    }

    pub fn load_ksf(&mut self, ksf: &Ksf) {
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

    pub fn load_session_data(&mut self, session_data: SessionData) {
        self.session_data = session_data;
    }

    fn reset(&mut self) {
        self.timers.iter_mut().for_each(|t| t.reset());
        self.counters.iter_mut().for_each(|c| c.reset());
        self.session_timer.reset();
        self.keypresses.clear();
        self.keypresses_display = VecDeque::from(["_"; 10]);
        self.ksf_name.clear();
        self.session_data = SessionData::new();
        self.clicked_keys.clear();
    }

    fn record_data(&mut self) {
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

        self.output_file_contents.push_str("\n---Session Time---\n");

        // Save session time information
        self.output_file_contents.push_str(&format!(
            "\nStart {}\nEnd {}\nDuration {}\n",
            date_time_string(self.session_timer.start_time),
            date_time_string(Local::now()),
            (Local::now() - self.session_timer.start_time).as_seconds_f32()
        ));

        self.output_file_contents.push_str("\n---Data---\n");

        self.output_file_contents.push_str("\nDuration\n");
        for timer in self.timers.iter_mut() {
            if let Some(description) = &timer.description {
                self.output_file_contents.push_str(&format!(
                    "{} {}\n",
                    description,
                    timer.saved_time.as_seconds_f32()
                ));
            }
        }

        self.output_file_contents.push_str("\nFrequency\n");
        for counter in self.counters.iter() {
            if let Some(description) = &counter.description {
                self.output_file_contents
                    .push_str(&format!("{} {}\n", description, counter.counter));
            }
        }

        self.output_file_contents.push_str("\nRaw Inputs\n");
        self.output_file_contents
            .push_str(&self.keypresses.iter().map(|k| k.name()).join(" "));
    }

    pub fn view(&mut self, ui: &mut Ui, active_page: &mut Page) {
        ui.ctx().input_mut(|i| {
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                if self.session_timer.active {
                    self.session_timer.stop();
                    self.keypresses.push(Key::Escape);
                    self.keypresses_display.pop_front();
                    self.keypresses_display.push_back("END");
                    self.record_data();
                    self.output_file_dialog.save_file();
                }
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Tab) {
                if !self.session_timer.active {
                    self.session_timer.start();
                    self.keypresses.push(Key::Tab);
                    self.keypresses_display.pop_front();
                    self.keypresses_display.push_back("ST");
                }
            }
            self.clicked_keys.update(i);
        });

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

            ui.add_space(10.0);
            ui.label("Tab to start. Esc to save and exit.");
            if self.session_timer.active {
                if ui
                    .button(RichText::new(" END ").monospace().color(Color32::RED))
                    .clicked()
                {
                    self.session_timer.stop();
                    self.keypresses.push(Key::Escape);
                    self.keypresses_display.pop_front();
                    self.keypresses_display.push_back("ESC");
                    self.record_data();
                }
            } else {
                if ui
                    .button(RichText::new("START").monospace().color(Color32::GREEN))
                    .clicked()
                {
                    self.session_timer.start();
                    self.keypresses.push(Key::Tab);
                    self.keypresses_display.pop_front();
                    self.keypresses_display.push_back("ST");
                    self.session_timer.start();
                }
            }
            self.output_file_dialog.update(ui.ctx());
            if let Some(path) = self.output_file_dialog.take_picked() {
                if std::fs::write(&path, &self.output_file_contents).is_ok() {
                    self.reset();
                    *active_page = Page::About;
                }
            }
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Session Time:");
                self.session_timer.view(ui);
            });

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
            ui.add_space(10.0);

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
