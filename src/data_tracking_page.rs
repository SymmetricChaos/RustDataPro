use crate::ksf::{Keybind, Ksf};
use chrono::{DateTime, Duration, Local};
use egui::Ui;

const NUM_TIMERS: usize = 10;

pub struct DataTrackingPage {
    ksf: Ksf,
    init_times: [DateTime<Local>; NUM_TIMERS],
    total_times: [Duration; NUM_TIMERS],
    timers_active: [bool; NUM_TIMERS],
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
            init_times: [Local::now(); NUM_TIMERS],
            total_times: [Duration::zero(); NUM_TIMERS],
            timers_active: [false; NUM_TIMERS],
        }
    }
}

impl DataTrackingPage {
    pub fn view(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Press Keys to Start/Stop Timers");
            ui.ctx().input(|i| {
                for (idx, key) in self
                    .ksf
                    .duration
                    .iter()
                    .map(|kb| kb.key)
                    .into_iter()
                    .enumerate()
                {
                    if i.key_released(key) {
                        if self.timers_active[idx] {
                            self.total_times[idx] += Local::now() - self.init_times[idx];
                            self.timers_active[idx] = false;
                        } else {
                            self.init_times[idx] = Local::now();
                            self.timers_active[idx] = true;
                        }
                    }
                }
            });
            for (idx, keybind) in self.ksf.duration.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{} [{}]", &keybind.description, keybind.key.name()));
                    ui.label(format!(
                        "Total: {:.1}",
                        self.total_times[idx].as_seconds_f32()
                    ));
                    ui.add_space(3.0);
                    if self.timers_active[idx] {
                        ui.request_repaint();
                        ui.label(format!(
                            "Current: {:.1}",
                            (Local::now() - self.init_times[idx]).as_seconds_f32()
                        ));
                    } else {
                        ui.label(format!("Current: 0"));
                    }
                });
                ui.add_space(5.0);
            }
        });
    }
}
