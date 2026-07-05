use crate::{
    app::DisplayInfo,
    data::{Data, DataType},
    data_tracking::{TimerStatus, counter::Counter, timer::Timer},
    utils::{ClickedKeys, DataProUiElements, date_time_string},
};
use anyhow::Result;
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
    ($self:ident, $key:expr, $time:expr) => {
        $self.keypresses.push(($key, $time));
        $self.keypresses_display.pop_front();
        $self.keypresses_display.push_back($key.name());
    };
}

pub struct SessionPage {
    timers: Vec<Timer>,
    counters: Vec<Counter>,
    session_timer: Timer,
    session_start: DateTime<Local>,
    keypresses: Vec<(Key, f32)>,
    keypresses_display: VecDeque<&'static str>,
    clicked_keys: ClickedKeys,
    save_discard_open: bool,
}

impl SessionPage {
    pub fn new() -> Self {
        Self {
            session_timer: Timer::default(),
            session_start: Local::now(),
            timers: Vec::new(),
            counters: Vec::new(),
            keypresses: Vec::new(),
            keypresses_display: VecDeque::from(["_"; 10]),
            clicked_keys: ClickedKeys::new(),
            save_discard_open: false,
        }
    }

    fn reset(&mut self) {
        for timer in self.timers.iter_mut() {
            timer.reset();
        }
        for counter in self.counters.iter_mut() {
            counter.reset();
        }
        self.session_timer.reset();
        self.keypresses.clear();
        self.keypresses_display = VecDeque::from(["_"; 10]);
        self.clicked_keys.clear();
        self.save_discard_open = false;
    }

    fn stop_all_timers(&mut self) {
        for timer in self.timers.iter_mut() {
            timer.stop();
        }
        self.session_timer.stop();
    }

    fn toggle_pause_all_timers(&mut self) {
        for timer in self.timers.iter_mut() {
            timer.toggle_pause();
        }
        self.session_timer.toggle_pause();
    }

    fn start_session(&mut self) {
        self.session_timer.start();
        self.session_start = Local::now();
        self.keypresses.push((Key::Tab, 0.0));
        self.keypresses_display.pop_front();
        self.keypresses_display.push_back("ST");
    }

    fn save_session(&mut self, data: &mut Data, client_data_path: &Option<String>) {
        self.keypresses
            .push((Key::Escape, self.session_timer.total_time()));
        self.keypresses_display.pop_front();
        self.keypresses_display.push_back("END");
        self.save_output(data).unwrap();
        self.update_client_file(data, client_data_path).unwrap()
    }

    fn end_session(&mut self, display_info: &mut DisplayInfo) {
        self.reset();
        display_info.go_to_about();
    }

    // fn write_json(&mut self, data: &mut Data) -> String {
    //     todo!()
    // }

    fn write_data(&mut self, data: &mut Data) -> String {
        let mut output = String::new();

        output.push_str("---Session---\n");
        output.push_str(&data.client.to_string());
        output.push('\n');
        output.push_str(&format!(
            "\nStart {}\nDuration {:.1}\n",
            date_time_string(self.session_start),
            self.session_timer.total_time()
        ));
        output.push('\n');
        output.push_str(&data.session.to_string());

        output.push_str("\n\n--Duration--\n");
        for timer in self.timers.iter_mut() {
            output.push_str(&format!(
                "{} {:.1}\n",
                timer.description,
                timer.saved_time(),
            ));
        }

        output.push_str("\n--Frequency--\n");
        for counter in self.counters.iter() {
            output.push_str(&format!("{} {}\n", counter.description, counter.counter));
        }

        output.push_str("\n--Raw Inputs--\n");
        output.push_str(
            &self
                .keypresses
                .iter()
                .map(|(k, t)| format!("{} {:.1}", k.name(), t))
                .join("\n"),
        );

        output
    }

    fn update_client_file(
        &mut self,
        data: &mut Data,
        client_data_path: &Option<String>,
    ) -> Result<()> {
        if data.session.data_type == DataType::Primary {
            if let Some(path) = client_data_path {
                std::fs::write(path, &data.client.to_json()?)?;
            }
            data.client.session_number += 1;
        }
        Ok(())
    }

    fn save_output(&mut self, data: &mut Data) -> Result<()> {
        let file = File::create(&format!(
            "{}{}{}.txt",
            data.client
                .name
                .chars()
                .filter(|c| c.is_ascii_uppercase())
                .join(""),
            data.client.session_number,
            data.session.data_type
        ))?;
        let mut writer = BufWriter::new(file);
        writer.write_all(self.write_data(data).as_bytes())?;
        writer.flush()?;

        // let file = File::create(&format!(
        //     "{}{}{}.raw",
        //     data.client
        //         .name
        //         .chars()
        //         .filter(|c| c.is_ascii_uppercase())
        //         .join(""),
        //     data.client.session_number,
        //     data.session.data_type
        // ))?;
        // let mut writer = BufWriter::new(file);
        // writer.write_all(self.write_json(data).as_bytes())?;
        // writer.flush()?;

        Ok(())
    }

    pub fn view(
        &mut self,
        ui: &mut Ui,
        display_info: &mut DisplayInfo,
        data: &mut Data,
        client_data_path: &Option<String>,
    ) {
        // Itercept key presses to detect clicks and then delete all of them to prevent egui from reusing them.
        ui.ctx().input_mut(|i| {
            self.clicked_keys.update(i);
            i.events.clear();
        });

        // ### Permanent keys are tracked here ###
        // ### KSF keys are tracked where the counters and timers are drawn ###
        // Toggle pausing as needed
        if self.clicked_keys.contains(&egui::Key::Space) {
            self.toggle_pause_all_timers();
        }
        // Allowed at any time in order to close Session in opened incorrectly
        if self.clicked_keys.contains(&egui::Key::Escape) {
            self.save_discard_open = true;
            self.stop_all_timers();
        }
        // Starting is only allowed when session is Stopped.
        if self.clicked_keys.contains(&egui::Key::Tab) {
            if self.session_timer.status.is_stopped() {
                self.start_session();
            }
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("Client: {}", data.client.name));
                        ui.label(format!("Session Number: {}", data.client.session_number));
                        ui.label(format!("KSF: {}", data.ksf.name))
                    });
                });
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("Assessment: {}", data.session.assessment));
                        ui.label(format!("Condition: {}", data.session.condition));
                        ui.label(format!("Data Type: {}", data.session.data_type));
                    });
                });
            });
            ui.add_space(10.0);

            ui.label("TAB to start. ESC return to about page. SPACE to pause/unpause.");
            match self.session_timer.status {
                TimerStatus::Active => {
                    ui.label(RichText::new("ACTIVE").monospace().color(Color32::GREEN))
                }
                TimerStatus::Stopped => {
                    ui.label(RichText::new("STOPPED").monospace().color(Color32::RED))
                }
                TimerStatus::Paused => {
                    ui.label(RichText::new("PAUSED").monospace().color(Color32::YELLOW))
                }
            };
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
                        ui.add_enabled_ui(self.session_timer.status.is_active(), |ui| {
                            egui::Grid::new("frequency_grid")
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.label("Description");
                                    ui.label("Key");
                                    ui.label("Total");
                                    ui.end_row();

                                    for counter in self.counters.iter_mut() {
                                        if let Some(key) = counter.key {
                                            if self.session_timer.status.is_active()
                                                && self.clicked_keys.contains(&key)
                                            {
                                                counter.inc();
                                                record_keypress!(
                                                    self,
                                                    key,
                                                    self.session_timer.total_time()
                                                );
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
                        ui.add_enabled_ui(self.session_timer.status.is_active(), |ui| {
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
                                            if self.session_timer.status.is_active()
                                                && self.clicked_keys.contains(&key)
                                            {
                                                timer.toggle();
                                                record_keypress!(
                                                    self,
                                                    key,
                                                    self.session_timer.total_time()
                                                );
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
            if self.save_discard_open {
                egui::Window::new("Save/Discard").show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui.large_green_button("SAVE").clicked() {
                            self.save_session(data, client_data_path);
                            self.end_session(display_info);
                        }
                        ui.add_space(20.0);
                        if ui.large_red_button("DISCARD").clicked() {
                            self.end_session(display_info);
                        }
                    });
                });
            }
        });
    }
}
