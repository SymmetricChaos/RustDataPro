use crate::ksf::Ksf;
use chrono::{DateTime, Duration, Local};
use egui::Ui;

const MAX_DUR: usize = 20;
const MAX_FREQ: usize = 20;

pub struct Session {
    ksf: Ksf,
    init_times: [DateTime<Local>; MAX_DUR],
    total_times: [Duration; MAX_DUR],
    timers_active: [bool; MAX_DUR],
    counters: [u32; MAX_FREQ],
}

impl Default for Session {
    fn default() -> Self {
        Self {
            ksf: Ksf::new(),
            init_times: [Local::now(); MAX_DUR],
            total_times: [Duration::zero(); MAX_DUR],
            timers_active: [false; MAX_DUR],
            counters: [0; MAX_FREQ],
        }
    }
}

impl Session {
    pub fn load_ksf(&mut self, ksf: &Ksf) {
        self.ksf = ksf.clone()
    }

    pub fn view(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Timers");

            egui::Grid::new("timer_grid").striped(true).show(ui, |ui| {
                ui.label("Description");
                ui.label("Key");
                ui.label("Current");
                ui.label("Total");
                ui.end_row();

                for (idx, keybind) in self.ksf.duration.iter().enumerate() {
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

                    if self.timers_active[idx] {
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
                        ui.ctx().input(|i| {
                            if i.num_presses(keybind.key) > 0 {
                                self.counters[idx] += 1;
                            }
                        });

                        ui.label(&keybind.description);
                        ui.label(keybind.key.name());
                        ui.label(self.counters[idx].to_string());

                        ui.end_row();
                    }
                });
        });
    }
}
