use crate::{ksf::Ksf, timer::Timer, utils::date_time_string};
use chrono::{DateTime, Local};
use egui::Ui;
use egui_file_dialog::FileDialog;

const MAX_DUR: usize = 20;
const MAX_FREQ: usize = 20;

pub struct Session {
    ksf: Ksf,
    timers: [Timer; MAX_DUR],
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
            timers: [
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
                Timer::new().split(),
            ],
            counters: [0; MAX_FREQ],
            file_dialog: FileDialog::new().default_file_name("SaveData.txt"),
            output_file_contents: String::new(),
            session_active: false,
        }
    }
}

impl Session {
    pub fn load_ksf(&mut self, ksf: &Ksf) {
        self.ksf = ksf.clone();
        for (keybind, timer) in self.ksf.duration.iter().zip(self.timers.iter_mut()) {
            timer.keybind = Some(keybind.clone())
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
        for (idx, keybind) in self.ksf.frequency.iter().enumerate() {
            self.output_file_contents
                .push_str(&format!("{} {}\n", keybind.description, self.counters[idx]));
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
                for timer in self.timers.iter_mut() {
                    timer.reset();
                }
                self.counters = [0; MAX_FREQ];
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
        });
    }
}
