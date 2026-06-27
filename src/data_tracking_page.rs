use crate::ksf::{Keybind, Ksf};
use chrono::{DateTime, Duration, Local};
use egui::Ui;

const MAX_DUR: usize = 15;
const MAX_FREQ: usize = 15;

pub struct DataTrackingPage {
    ksf: Ksf,
    init_times: [DateTime<Local>; MAX_DUR],
    total_times: [Duration; MAX_DUR],
    timers_active: [bool; MAX_DUR],
    counters: [u32; MAX_FREQ],
}

impl Default for DataTrackingPage {
    fn default() -> Self {
        let temp_ksf = Ksf {
            duration: vec![
                Keybind::from_string("1,Sr+"),
                Keybind::from_string("5,Sdelta"),
            ],
            frequency: vec![
                Keybind::from_string("a,Aggression"),
                Keybind::from_string("s,SIB"),
            ],
        };
        Self {
            ksf: temp_ksf,
            init_times: [Local::now(); MAX_DUR],
            total_times: [Duration::zero(); MAX_DUR],
            timers_active: [false; MAX_DUR],
            counters: [0; MAX_FREQ],
        }
    }
}

impl DataTrackingPage {
    pub fn view(&mut self, ui: &mut Ui) {
        egui::Window::new("Timers")
            .default_height(300.0)
            .default_width(200.0)
            .default_pos((0.0, 0.0))
            .constrain_to(ui.available_rect_before_wrap())
            .show(ui, |ui| {
                ui.monospace(format!("Description  Key  Current  Total"));
                ui.separator();
                for (idx, keybind) in self.ksf.duration.iter().enumerate() {
                    ui.ctx().input(|i| {
                        if i.key_released(keybind.key) {
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
                        ui.monospace(format!(
                            "{:<12} [{}]      {:.1}    {:.1}",
                            &keybind.description,
                            keybind.key.name(),
                            self.total_times[idx].as_seconds_f32(),
                            (Local::now() - self.init_times[idx]).as_seconds_f32()
                        ));
                    } else {
                        ui.monospace(format!(
                            "{:<12} [{}]      {:.1}    0.0",
                            &keybind.description,
                            keybind.key.name(),
                            self.total_times[idx].as_seconds_f32()
                        ));
                    }
                }
            });

        egui::Window::new("Counters")
            .default_pos((250.0, 0.0))
            .constrain_to(ui.available_rect_before_wrap())
            .show(ui, |ui| {
                ui.monospace("Description  Key  Total");
                ui.separator();
                for (idx, keybind) in self.ksf.frequency.iter().enumerate() {
                    ui.ctx().input(|i| {
                        if i.key_released(keybind.key) {
                            self.counters[idx] += 1;
                        }
                    });

                    ui.monospace(format!(
                        "{:<12} [{}]  {:>5}",
                        &keybind.description,
                        keybind.key.name(),
                        self.counters[idx]
                    ));

                    ui.add_space(5.0);
                }
            });
    }
}
