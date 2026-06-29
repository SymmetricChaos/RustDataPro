use crate::{ksf::Ksf, utils::date_time_string};
use chrono::{DateTime, Duration, Local};
use egui::Ui;
use egui_file_dialog::FileDialog;

const MAX_DUR: usize = 20;
const MAX_FREQ: usize = 20;

pub struct Session {
    ksf: Ksf,
    init_times: [DateTime<Local>; MAX_DUR],
    total_times: [Duration; MAX_DUR],
    timers_active: [bool; MAX_DUR],
    counters: [u32; MAX_FREQ],
    session_start_time: DateTime<Local>,
    file_dialog: FileDialog,
    output_file_contents: String,
    session_active: bool,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            ksf: Ksf::new(),
            session_start_time: Local::now(),
            init_times: [Local::now(); MAX_DUR],
            total_times: [Duration::zero(); MAX_DUR],
            timers_active: [false; MAX_DUR],
            counters: [0; MAX_FREQ],
            file_dialog: FileDialog::new().default_file_name("SaveData.txt"),
            output_file_contents: String::new(),
            session_active: false
        }
    }
}

impl Session {
    pub fn load_ksf(&mut self, ksf: &Ksf) {
        self.ksf = ksf.clone()
    }

    pub fn view(&mut self, ui: &mut Ui) {
        egui::Panel::top("data_top_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {


                if ui.button("Start Session").clicked() {
                    self.session_start_time = Local::now();
                    self.session_active = true;
                }

                if ui.button("End Session").clicked() {
                    self.session_active = false;
                    self.output_file_contents.clear();

                    self.output_file_contents.push_str( 
                        &format!("Start {}\nEnd {}\nDuration {}\n", date_time_string(self.session_start_time), date_time_string(Local::now()), (Local::now()-self.session_start_time).as_seconds_f32())
                    );

                    // Save duration data
                    for (idx, keybind) in self.ksf.duration.iter().enumerate() {
                        // Stop timers if they are running and update the total
                        if self.timers_active[idx] {
                            self.total_times[idx] += Local::now() - self.init_times[idx];
                            self.timers_active[idx] = false;
                        }
                        self.output_file_contents.push_str(&format!("{} {}\n",keybind.description, self.total_times[idx].as_seconds_f32()));
                    };
                    // Save frequency data
                    for (idx, keybind) in self.ksf.frequency.iter().enumerate() {
                        self.output_file_contents.push_str(&format!("{} {}\n",keybind.description, self.counters[idx]));
                    };

                    // Open save dialog
                    self.file_dialog.save_file();

                }
                
                self.file_dialog.update(ui.ctx());

                if let Some(path) = self.file_dialog.take_picked() {
                    if std::fs::write(&path, &self.output_file_contents).is_ok() {
                        println!("Successfully saved to: {:?}", path);
                    }
                }

            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Timers");

            egui::Grid::new("timer_grid").striped(true).show(ui, |ui| {
                ui.label("Description");
                ui.label("Key");
                ui.label("Current");
                ui.label("Total");
                ui.end_row();

                for (idx, keybind) in self.ksf.duration.iter().enumerate() {
                    if self.session_active {
                        ui.ctx().input(|i| {
                            if i.num_presses(keybind.key) > 0 {
                                if self.timers_active[idx] {
                                    self.total_times[idx] += Local::now() - self.init_times[idx];
                                    self.timers_active[idx] = false;
                                } else {
                                    self.init_times[idx] = Local::now();
                                    self.timers_active[idx] = true;
                                }
                            }
                        });
                    }

                    if self.timers_active[idx] && self.session_active {
                        ui.request_repaint();
                        ui.label(&keybind.description);
                        ui.label(keybind.key.name());
                        ui.label(format!("{:.1}", self.total_times[idx].as_seconds_f32()));
                        ui.label(format!(
                            "{:.1}",
                            (Local::now() - self.init_times[idx]).as_seconds_f32()
                        ));
                    } else {
                        ui.label(&keybind.description);
                        ui.label(keybind.key.name());
                        ui.label(format!("{:.1}", self.total_times[idx].as_seconds_f32()));
                        ui.label("0.0");
                    }
                    ui.end_row();
                }
            });
            ui.add_space(10.0);

            ui.heading("Counters");
            egui::Grid::new("counter_grid")
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Description");
                    ui.label("Key");
                    ui.label("Total");
                    ui.end_row();

                    for (idx, keybind) in self.ksf.frequency.iter().enumerate() {
                        if self.session_active {
                            ui.ctx().input(|i| {
                                if i.num_presses(keybind.key) > 0 {
                                    self.counters[idx] += 1;
                                }
                            });
                        }
                        ui.label(&keybind.description);
                        ui.label(keybind.key.name());
                        ui.label(self.counters[idx].to_string());

                        ui.end_row();
                    }
                });
        });
    }
}
