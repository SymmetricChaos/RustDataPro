use crate::{
    app::DisplayInfo,
    data::{
        Data, DataType, Timer, TimerStatus, output_data::OutputData, timeline::Timeline,
        view_simple_timer,
    },
    utils::{ClickedKeys, DataProUiElements, date_time_string, rounded_f32},
};
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use egui::{Color32, Key, Layout, RichText, Ui};
use egui_extras::Column;
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
        $self.unpress_available = true;
    };
}

/// Need to use a macro to pass around a string literal
macro_rules! timer_format {
    () => {
        "{:7.2}"
    };
}

macro_rules! yellow_timer {
    ($ui:ident, $timer:expr) => {
        $ui.col(|ui| {
            ui.monospace(RichText::new(format!(timer_format!(), $timer)).color(Color32::YELLOW));
        });
    };
}

macro_rules! default_timer {
    ($ui:ident, $timer:expr) => {
        $ui.col(|ui| {
            ui.monospace(RichText::new(format!(timer_format!(), $timer)));
        });
    };
}

macro_rules! timer_display {
    (bright, $row:ident, $desc:ident, $key:ident, $time1:expr, $time2:expr, $bouts:expr) => {
        $row.col(|ui| {
            ui.monospace(RichText::new($desc).color(Color32::YELLOW));
        });
        $row.col(|ui| {
            ui.monospace(RichText::new($key.name()).color(Color32::YELLOW));
        });
        yellow_timer!($row, $time1);
        yellow_timer!($row, $time2);
        $row.col(|ui| {
            ui.monospace(RichText::new($bouts.to_string()).color(Color32::YELLOW));
        });
    };
    (dim, $row:ident, $desc:ident, $key:ident, $time1:expr, $time2:expr, $bouts:expr) => {
        $row.col(|ui| {
            ui.monospace(RichText::new($desc));
        });
        $row.col(|ui| {
            ui.monospace(RichText::new($key.name()));
        });
        default_timer!($row, $time1);
        default_timer!($row, $time2);
        $row.col(|ui| {
            ui.monospace(RichText::new($bouts.to_string()));
        });
    };
}

const DESCRIPTION_WIDTH: f32 = 100.0;
const KEY_WIDTH: f32 = 40.0;

pub struct SessionPage {
    timers: Vec<(Timer, Key, String)>,
    counters: Vec<(u32, Key, String)>,
    session_timer: Timer,
    session_start: DateTime<Local>,
    timeline: Timeline,
    keypresses_display: VecDeque<&'static str>,
    clicked_keys: ClickedKeys,
    save_discard_open: bool,
    unpress_available: bool,
}

impl SessionPage {
    pub fn new() -> Self {
        Self {
            session_timer: Timer::default(),
            session_start: Local::now(),
            timers: Vec::new(),
            counters: Vec::new(),
            timeline: Timeline::default(),
            keypresses_display: VecDeque::from(["_"; 11]),
            clicked_keys: ClickedKeys::new(),
            save_discard_open: false,
            unpress_available: false,
        }
    }

    fn reset(&mut self) {
        self.timers.clear();
        self.counters.clear();
        self.session_timer.reset();
        self.timeline.clear();
        self.keypresses_display = VecDeque::from(["_"; 11]);
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

    fn unpress_key(&mut self) {
        if self.unpress_available {
            self.keypresses_display.push_front("_");
            self.keypresses_display.pop_back();
            if let Some((removed_key, _time)) = self.timeline.pop() {
                for (timer, key, _) in self.timers.iter_mut() {
                    if key == &removed_key {
                        if timer.is_active() {
                            timer.unstart();
                        } else {
                            timer.start_without_bout();
                        }
                    }
                }
                for (counter, key, _) in self.counters.iter_mut() {
                    if key == &removed_key {
                        *counter = counter.saturating_sub(1);
                    }
                }
            };
            self.unpress_available = false;
        }
    }

    pub fn load_ksf(&mut self, data: &Data) {
        for kb in data.ksf.duration.iter() {
            self.timers.push((Timer::default(), kb.0, kb.1.clone()));
        }
        for kb in data.ksf.frequency.iter() {
            self.counters.push((0, kb.0, kb.1.clone()));
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
    fn write_output_pretty(&self, data: &mut Data) -> String {
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
        for (counter, _, desc) in self.counters.iter() {
            output.push_str(&format!("{} {}\n", desc, counter));
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
    fn write_output_json(&self, data: &mut Data) -> Result<String> {
        let mut fre_map: IndexMap<String, u32> = IndexMap::new();
        for (t, _, d) in self.counters.iter() {
            fre_map.insert(d.clone(), *t);
        }
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
            frequency: fre_map,
            timeline: self.timeline.clone(),
            ksf: data.ksf.clone(),
        })
        .context("failure to create json")
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
        writer.write_all(self.write_output_pretty(data).as_bytes())?;
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
        writer.write_all(self.write_output_json(data)?.as_bytes())?;
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

        // ### Permanent Keys ###
        // Starting is only allowed when session is Stopped.
        if self.clicked_keys.contains(&egui::Key::Tab) {
            if self.session_timer.status.is_stopped() {
                self.start_session();
            }
        }
        // Stop and quit at any time.
        if self.clicked_keys.contains(&egui::Key::Escape) {
            self.save_discard_open = true;
            self.stop_all_timers();
        }
        // Pausing can be toggled. Definition of pause prevents this from being used when Stopped.
        if self.clicked_keys.contains(&egui::Key::Space) {
            self.toggle_pause_all_timers();
        }
        if self.clicked_keys.contains(&egui::Key::Backspace) {
            if self.session_timer.status.is_active() {
                self.unpress_key();
            }
        }

        // ### Duration Frequency Keys ###
        if self.session_timer.status.is_active() {
            for (timer, key, _) in self.timers.iter_mut() {
                if self.clicked_keys.contains(key) {
                    timer.toggle();
                    record_keypress!(self, *key, self.session_timer.total_time());
                }
            }
            for (counter, key, _) in self.counters.iter_mut() {
                if self.clicked_keys.contains(key) {
                    *counter += 1;
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

            ui.label("TAB to start. ESC return to end session. SPACE to pause/unpause.");
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
            ui.add_enabled_ui(self.session_timer.status.is_active(), |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.group(|ui| {
                            ui.heading("Frequency Keys");
                            egui_extras::TableBuilder::new(ui)
                                .id_salt("frequency")
                                .column(Column::exact(DESCRIPTION_WIDTH))
                                .column(Column::exact(KEY_WIDTH))
                                .column(Column::exact(40.0))
                                .striped(true)
                                .cell_layout(Layout::default().with_cross_align(egui::Align::RIGHT))
                                .body(|mut body| {
                                    body.row(20.0, |mut row| {
                                        row.col(|ui| {
                                            ui.strong("Description");
                                        });
                                        row.col(|ui| {
                                            ui.strong("Key");
                                        });
                                        row.col(|ui| {
                                            ui.strong("Count");
                                        });
                                    });
                                    for (counter, key, desc) in self.counters.iter() {
                                        body.row(20.0, |mut row| {
                                            row.col(|ui| {
                                                ui.monospace(desc);
                                            });
                                            row.col(|ui| {
                                                ui.monospace(key.name());
                                            });
                                            row.col(|ui| {
                                                ui.monospace(counter.to_string());
                                            });
                                        });
                                    }
                                });
                        })
                    });
                    ui.vertical(|ui| {
                        ui.group(|ui| {
                            ui.heading("Duration Keys");
                            egui_extras::TableBuilder::new(ui)
                                .id_salt("durationkeys")
                                .column(Column::exact(DESCRIPTION_WIDTH))
                                .column(Column::exact(KEY_WIDTH))
                                .column(Column::exact(60.0))
                                .column(Column::exact(60.0))
                                .column(Column::exact(40.0))
                                .striped(true)
                                .cell_layout(Layout::default().with_cross_align(egui::Align::RIGHT))
                                .body(|mut body| {
                                    body.row(20.0, |mut row| {
                                        row.col(|ui| {
                                            ui.strong("Description");
                                        });
                                        row.col(|ui| {
                                            ui.strong("Key");
                                        });
                                        row.col(|ui| {
                                            ui.strong("Total");
                                        });
                                        row.col(|ui| {
                                            ui.strong("Current");
                                        });
                                        row.col(|ui| {
                                            ui.strong("Bouts");
                                        });
                                    });
                                    for (timer, key, desc) in self.timers.iter() {
                                        body.row(20.0, |mut row| match timer.status {
                                            TimerStatus::Active => {
                                                timer_display!(
                                                    bright,
                                                    row,
                                                    desc,
                                                    key,
                                                    timer.saved_time(),
                                                    timer.current_time(),
                                                    timer.bouts
                                                );
                                            }
                                            TimerStatus::Stopped => {
                                                timer_display!(
                                                    dim,
                                                    row,
                                                    desc,
                                                    key,
                                                    timer.saved_time(),
                                                    0.0,
                                                    timer.bouts
                                                );
                                            }
                                            TimerStatus::Paused => {
                                                timer_display!(
                                                    bright,
                                                    row,
                                                    desc,
                                                    key,
                                                    timer.saved_time(),
                                                    timer.stashed_time(),
                                                    timer.bouts
                                                );
                                            }
                                        });
                                    }
                                });
                        })
                    });
                });
            });
            ui.add_space(10.0);

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    for k in self.keypresses_display.make_contiguous()[1..11].iter() {
                        ui.monospace(*k);
                    }
                });
            });
            ui.label("BACKSPACE to delete last entry.");

            if self.save_discard_open {
                egui::Window::new("Confirm Exit").show(ui, |ui| {
                    ui.columns(2, |columns| {
                        columns[0].set_height(50.0);
                        columns[0].add_enabled_ui(self.session_timer.was_started(), |ui| {
                            if ui
                                .large_green_button("SAVE")
                                .on_disabled_hover_text("no data to save")
                                .clicked()
                            {
                                self.save_session(data, client_data_path);
                                self.end_session(display_info);
                            }
                        });
                        columns[1].set_height(50.0);
                        if columns[1].large_red_button("DISCARD").clicked() {
                            self.end_session(display_info);
                        }
                    });
                });
            }
        });
    }
}
