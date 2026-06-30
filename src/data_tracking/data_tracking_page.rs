use crate::{ data_tracking::{counter::Counter, timer::Timer}, ksf::Ksf, utils::date_time_string};
use chrono::{DateTime, Local};
use egui::Ui;
use egui_file_dialog::FileDialog;

const MAX_DUR: usize = 20;
const MAX_FREQ: usize = 20;

pub struct Session {
    timers: [Timer; MAX_DUR],
    counters: [Counter; MAX_FREQ],
    session_start_time: DateTime<Local>,
    file_dialog: FileDialog,
    output_file_contents: String,
    session_active: bool,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            session_start_time: Local::now(),
            timers: [
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
                Timer::new().with_split(),
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
            session_active: false,
        }
    }
}

impl Session {
    pub fn load_ksf(&mut self, ksf: &Ksf) {
        for (keybind, timer) in ksf.duration.iter().zip(self.timers.iter_mut()) {
            timer.keybind = Some(keybind.clone())
        }
        for (keybind, counter) in ksf.frequency.iter().zip(self.counters.iter_mut()) {
            counter.keybind = Some(keybind.clone())
        }
    }

    pub fn save_session(&mut self) {
        self.session_active = false;
        self.output_file_contents.clear();

        self.output_file_contents.push_str(&format!(
            "Start {}\nEnd {}\nDuration {}\n",
            date_time_string(self.session_start_time),
            date_time_string(Local::now()),
            (Local::now() - self.session_start_time).as_seconds_f32()
        ));
        self.output_file_contents.push('\n');
        // Save duration data
        for timer in self.timers.iter_mut() {
            // Stop timers if they are running and update the total
            if let Some(keybind) = timer.keybind.clone() {
                timer.stop();
                self.output_file_contents.push_str(&format!(
                    "{} {}\n",
                    keybind.description,
                    timer.total_time.as_seconds_f32()
                ));
            }
        }
        self.output_file_contents.push('\n');
        // Save frequency data
        for counter in self.counters.iter() {
            if let Some(keybind) = counter.keybind.clone() {
                self.output_file_contents
                    .push_str(&format!("{} {}\n", keybind.description, counter.counter));
            }
        }

        // Open save dialog
        self.file_dialog.save_file();
    }

    pub fn view(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Session Controls");
            if ui.button("Start").clicked() {
                self.session_active = true;
            }
            if ui.button("End").clicked() {
                self.save_session();
            }
            ui.add_space(10.0);

            self.file_dialog.update(ui.ctx());

            if let Some(path) = self.file_dialog.take_picked() {
                if std::fs::write(&path, &self.output_file_contents).is_ok() {
                    println!("Successfully saved to: {:?}", path);
                }
                self.timers.iter_mut().for_each(|t| t.reset());
                self.counters.iter_mut().for_each(|c| c.reset());
                self.session_active = false;
            }

            ui.heading("Timers");
            ui.add_enabled_ui(self.session_active, |ui| {
                egui::Grid::new("timer_grid").striped(true).show(ui, |ui| {
                    ui.label("Description");
                    ui.label("Key");
                    ui.label("Current");
                    ui.label("Total");
                    ui.end_row();

                    for timer in self.timers.iter_mut() {
                        if let Some(keybind) = timer.keybind.clone() {
                            if self.session_active {
                                ui.ctx().input(|i| {
                                    if i.num_presses(keybind.key) > 0 {
                                        timer.toggle();
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
            ui.add_enabled_ui(self.session_active, |ui| {
                egui::Grid::new("counter_grid")
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Description");
                        ui.label("Key");
                        ui.label("Total");
                        ui.end_row();

                        for counter in self.counters.iter_mut() {
                            if let Some(keybind) = counter.keybind.clone() {
                                if self.session_active {
                                    ui.ctx().input(|i| {
                                        if i.num_presses(keybind.key) > 0 {
                                            counter.inc();
                                        }
                                    });
                                }
                                counter.view(ui);
                                ui.end_row();
                            }
                        }
                    });
            });
        });
    }
}
