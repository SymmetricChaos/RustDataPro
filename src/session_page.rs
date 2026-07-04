use crate::{
    data::{SessionData, client::ClientData, ksf::Ksf},
    data_tracking::{counter::Counter, timer::Timer},
    pages::Page,
    utils::{ClickedKeys, date_time_string},
};
use chrono::{DateTime, Local};
use egui::{
    Color32,
    Key::{self},
    RichText, Ui,
};
use itertools::Itertools;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufWriter, Write},
};

macro_rules! record_keypress {
    ($self:ident, $key:expr) => {
        $self.keypresses.push($key);
        $self.keypresses_display.pop_front();
        $self.keypresses_display.push_back($key.name());
    };
}

pub struct SessionPage {
    ksf_name: String,
    timers: [Timer; 20],
    counters: [Counter; 20],
    session_timer: Timer,
    session_start: DateTime<Local>,
    output_file_contents: String,
    keypresses: Vec<Key>,
    keypresses_display: VecDeque<&'static str>,
    clicked_keys: ClickedKeys,
    save_discard_open: bool,
}

impl SessionPage {
    pub fn new() -> Self {
        Self {
            ksf_name: String::new(),
            session_timer: Timer::new(),
            session_start: Local::now(),
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
            output_file_contents: String::new(),
            keypresses: Vec::new(),
            keypresses_display: VecDeque::from(["_"; 10]),
            clicked_keys: ClickedKeys::new(),
            save_discard_open: false,
        }
    }

    pub fn load_ksf(&mut self, ksf: &Ksf) {
        self.ksf_name = ksf.name.clone();
        for (keybind, timer) in ksf.duration.iter().cloned().zip(self.timers.iter_mut()) {
            timer.key = Some(keybind.0);
            timer.description = Some(keybind.1);
        }
        for (keybind, counter) in ksf.frequency.iter().cloned().zip(self.counters.iter_mut()) {
            counter.key = Some(keybind.0);
            counter.description = Some(keybind.1);
        }
    }

    fn start_session(&mut self, ksf: &Ksf) {
        if !self.session_timer.active {
            self.load_ksf(ksf);
            self.session_timer.start();
            self.session_start = Local::now();
            self.keypresses.push(Key::Tab);
            self.keypresses_display.pop_front();
            self.keypresses_display.push_back("ST");
        }
    }

    fn end_session(
        &mut self,
        client_data: &mut ClientData,
        active_page: &mut Page,
        session_data: &SessionData,
        client_data_path: &Option<String>,
    ) {
        if self.session_timer.active {
            self.keypresses.push(Key::Escape);
            self.keypresses_display.pop_front();
            self.keypresses_display.push_back("END");
            self.record_data(client_data, session_data, client_data_path);
            self.reset();
            *active_page = Page::About;
        }
    }

    fn reset(&mut self) {
        self.session_timer.reset();
        self.keypresses.clear();
        self.keypresses_display = VecDeque::from(["_"; 10]);
        self.clicked_keys.clear();
    }

    fn stop_all_timers(&mut self) {
        for timer in self.timers.iter_mut() {
            timer.stop();
        }
        self.session_timer.stop();
    }

    fn record_data(
        &mut self,
        client_data: &mut ClientData,
        session_data: &SessionData,
        client_data_path: &Option<String>,
    ) {
        // Stop all timers

        // Reset the output
        self.output_file_contents.clear();

        self.output_file_contents.push_str("---Session---\n");
        self.output_file_contents.push_str(&client_data.to_string());
        self.output_file_contents.push_str(&format!(
            "\nStart {}\nDuration {}\n",
            date_time_string(self.session_start),
            self.session_timer.total_time()
        ));
        self.output_file_contents
            .push_str(&session_data.to_string());

        self.output_file_contents.push_str("\n\n--Duration--\n");
        for timer in self.timers.iter_mut() {
            if let Some(description) = &timer.description {
                self.output_file_contents.push_str(&format!(
                    "{} {}\n",
                    description,
                    timer.saved_time(),
                ));
            }
        }

        self.output_file_contents.push_str("\n--Frequency--\n");
        for counter in self.counters.iter() {
            if let Some(description) = &counter.description {
                self.output_file_contents
                    .push_str(&format!("{} {}\n", description, counter.counter));
            }
        }

        self.output_file_contents.push_str("\n--Raw Inputs--\n");
        self.output_file_contents
            .push_str(&self.keypresses.iter().map(|k| k.name()).join(" "));

        // Create the file and save it
        let file = File::create(&format!(
            "{}{}.txt",
            client_data
                .name
                .chars()
                .filter(|c| c.is_ascii_uppercase())
                .join(""),
            client_data.session_number,
        ))
        .expect("failed to create file");
        let mut writer = BufWriter::new(file);

        writer
            .write_all(self.output_file_contents.as_bytes())
            .expect("failed to write file");
        writer.flush().expect("failed to flush file");

        // Update client data file
        if let Some(path) = client_data_path {
            std::fs::write(path, serde_json::to_string_pretty(&client_data).unwrap())
                .expect("failed to write to client file");
        }

        // Increment session number
        client_data.session_number += 1;
    }

    pub fn view(
        &mut self,
        ui: &mut Ui,
        active_page: &mut Page,
        client_data: &mut ClientData,
        session_data: &mut SessionData,
        ksf: &mut Ksf,
        client_data_path: &Option<String>,
    ) {
        ui.ctx().input_mut(|i| {
            // Need to consume Esc in order to prevent egui from using it for other purposes
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Escape) {
                self.save_discard_open = true;
                self.stop_all_timers();
            }
            if i.consume_key(egui::Modifiers::NONE, egui::Key::Tab) {
                self.start_session(&ksf);
            }
            self.clicked_keys.update(i);
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("Client: {}", client_data.name));
                        ui.label(format!("Session Number: {}", client_data.session_number));
                        ui.label(format!("KSF: {}", self.ksf_name))
                    });
                });
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("Assessment: {}", session_data.assessment));
                        ui.label(format!("Condition: {}", session_data.condition));
                        ui.label(format!("Data Type: {}", session_data.data_type));
                    });
                });
            });
            ui.add_space(10.0);

            ui.label("TAB to start. ESC to save and exit.");
            if self.session_timer.active {
                ui.label(RichText::new("ACTIVE").monospace().color(Color32::GREEN));
            } else {
                if self.session_timer.saved_time() == 0.0 {
                    ui.label(RichText::new("NOT STARTED").monospace().color(Color32::RED));
                } else {
                    ui.label(RichText::new("PAUSED").monospace().color(Color32::YELLOW));
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
                            egui::Grid::new("frequency_grid")
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
                            egui::Grid::new("duration_grid")
                                .striped(true)
                                .show(ui, |ui| {
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

        if self.save_discard_open {
            egui::Window::new("Save/Discard").show(ui, |ui| {
                if ui.button("SAVE").clicked() {
                    for timer in self.timers.iter_mut() {
                        timer.stop();
                    }
                    self.session_timer.stop();
                    self.end_session(client_data, active_page, session_data, client_data_path);
                }
                if ui.button("DISCARD").clicked() {
                    self.reset();
                    *active_page = Page::About;
                }
            });
        }
    }
}
