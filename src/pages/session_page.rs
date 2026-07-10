use crate::{
    app::DisplayInfo,
    data::{Data, DataType, output_data::OutputData, timeline::Timeline},
    data_tracking::{
        TimerStatus, counter::Counter, timer::Timer, view_session_page_timer, view_simple_timer,
    },
    utils::{ClickedKeys, DataProUiElements, date_time_string, rounded_f32},
};
use anyhow::Result;
use chrono::{DateTime, Local};
use egui::{Color32, Key, RichText, Ui};
use indexmap::IndexMap;
use itertools::Itertools;
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufWriter, Write},
};

macro_rules! record_keypress {
    ($self:ident, $key:expr, $time:expr) => {
        $self.timeline.push(($key, rounded_f32($time)));
        $self.keypresses_display.pop_front();
        $self.keypresses_display.push_back($key.name());
    };
}

pub struct SessionPage {
    timers: Vec<(Timer, Key, String)>,
    counters: Vec<Counter>,
    session_timer: Timer,
    session_start: DateTime<Local>,
    timeline: Timeline,
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
            timeline: Timeline::default(),
            keypresses_display: VecDeque::from(["_"; 10]),
            clicked_keys: ClickedKeys::new(),
            save_discard_open: false,
        }
    }

    fn reset(&mut self) {
        self.timers.clear();
        self.counters.clear();
        self.session_timer.reset();
        self.timeline.clear();
        self.keypresses_display = VecDeque::from(["_"; 10]);
        self.clicked_keys.clear();
        self.save_discard_open = false;
    }

    /// Stop all timers, including the session timer.
    fn stop_all_timers(&mut self) {
        for (timer, _, _) in self.timers.iter_mut() {
            timer.stop();
        }
        self.session_timer.stop();
    }

    /// Pause all timers, including the session timer.
    fn toggle_pause_all_timers(&mut self) {
        for (timer, _, _) in self.timers.iter_mut() {
            timer.toggle_pause_partial();
        }
        self.session_timer.toggle_pause();
    }

    pub fn load_ksf(&mut self, data: &Data) {
        for kb in data.ksf.duration.iter() {
            self.timers.push((Timer::default(), kb.0, kb.1.clone()));
        }
        for kb in data.ksf.frequency.iter() {
            self.counters.push(Counter::default().with_keybind(kb));
        }
    }

    fn start_session(&mut self) {
        self.session_timer.start();
        self.session_start = Local::now();
        self.timeline.push((Key::Tab, 0.0));
        self.keypresses_display.pop_front();
        self.keypresses_display.push_back("ST");
    }

    fn save_session(&mut self, data: &mut Data, client_data_path: &Option<String>) {
        self.timeline
            .push((Key::Escape, rounded_f32(self.session_timer.total_time())));
        self.keypresses_display.pop_front();
        self.keypresses_display.push_back("END");
        self.save_output(data).unwrap();
        self.update_client_file(data, client_data_path).unwrap()
    }

    fn end_session(&mut self, display_info: &mut DisplayInfo) {
        self.reset();
        display_info.go_to_about();
    }

    /// Write the output data into a human readable format.
    fn write_data(&mut self, data: &mut Data) -> String {
        let mut output = String::new();

        output.push_str("---Session---\n");
        output.push_str(&data.client.to_string());
        output.push('\n');
        output.push_str(&format!(
            "\nStart {}\nDuration {:.1}\n",
            date_time_string(&self.session_start),
            self.session_timer.total_time()
        ));
        output.push('\n');
        output.push_str(&data.session.to_string());

        output.push_str("\n\n--Duration--\n");

        for (timer, _, desc) in self.timers.iter() {
            output.push_str(&format!("{} {:.1}\n", desc, timer.saved_time()));
        }

        output.push_str("\n--Frequency--\n");
        for counter in self.counters.iter() {
            output.push_str(&format!("{} {}\n", counter.description, counter.counter));
        }

        output.push_str("\n--Raw Inputs--\n");
        output.push_str(
            &self
                .timeline
                .iter()
                .map(|(k, t)| format!("{} {:.1}", k.name(), t))
                .join("\n"),
        );

        output
    }

    /// Write the output data into a JSON format. Not especially human readable.
    fn write_json(&self, data: &mut Data) -> String {
        let mut dur_map: IndexMap<String, (u32, f32)> = IndexMap::new();
        for (t, _, d) in self.timers.iter() {
            dur_map.insert(d.clone(), (t.bouts, rounded_f32(t.total_time())));
        }

        serde_json::to_string(&OutputData {
            datetime: date_time_string(&self.session_start),
            session_duration: rounded_f32(self.session_timer.total_time()),
            client: data.client.clone(),
            session: data.session.clone(),
            duration: dur_map,
            frequency: self
                .counters
                .iter()
                .map(|c| (c.description.clone(), c.counter))
                .collect(),
            timeline: self.timeline.clone(),
            ksf: data.ksf.clone(),
        })
        .unwrap()
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
        }
        data.client.current_session += 1;
        Ok(())
    }

    fn save_output(&mut self, data: &mut Data) -> Result<()> {
        let file = File::create(&format!(
            "{}{}_{}.txt",
            data.client
                .name
                .chars()
                .filter(|c| c.is_ascii_uppercase())
                .join(""),
            data.client.current_session,
            data.session.data_type
        ))?;
        let mut writer = BufWriter::new(file);
        writer.write_all(self.write_data(data).as_bytes())?;
        writer.flush()?;

        let file = File::create(&format!(
            "{}{}_{}_raw.txt",
            data.client
                .name
                .chars()
                .filter(|c| c.is_ascii_uppercase())
                .join(""),
            data.client.current_session,
            data.session.data_type
        ))?;
        let mut writer = BufWriter::new(file);
        writer.write_all(self.write_json(data).as_bytes())?;
        writer.flush()?;

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

        // ### Duration Keys are Tracked Here ###
        if self.session_timer.status.is_active() {
            for (timer, key, _) in self.timers.iter_mut() {
                if self.clicked_keys.contains(key) {
                    timer.toggle();
                    record_keypress!(self, *key, self.session_timer.total_time());
                }
            }
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("Client: {}", data.client.name));
                        ui.label(format!("Session Number: {}", data.client.current_session));
                        ui.label(format!("KSF: {}", display_info.ksf_name))
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
                view_simple_timer(ui, &mut self.session_timer);
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

                                    for (timer, key, desc) in self.timers.iter_mut() {
                                        view_session_page_timer(ui, timer, key, desc);
                                        ui.end_row();
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
                egui::Window::new("Confirm Exit").show(ui, |ui| {
                    ui.columns(2, |columns| {
                        columns[0].set_height(60.0);
                        columns[0].add_enabled_ui(self.session_timer.was_started(), |ui| {
                            if ui.large_green_button("SAVE").clicked() {
                                self.save_session(data, client_data_path);
                                self.end_session(display_info);
                            }
                        });
                        columns[1].set_height(60.0);
                        if columns[1].large_red_button("DISCARD").clicked() {
                            self.end_session(display_info);
                        }
                    });
                });
            }
        });
    }
}
