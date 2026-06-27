use chrono::{DateTime, Duration, Local};
use egui::Ui;

const NUM_TIMERS: usize = 5;

pub struct DataTrackingPage {
    init_times: [DateTime<Local>; NUM_TIMERS],
    total_times: [Duration; NUM_TIMERS],
    timers_active: [bool; NUM_TIMERS],
}

impl Default for DataTrackingPage {
    fn default() -> Self {
        Self {
            init_times: [Local::now(); NUM_TIMERS],
            total_times: [Duration::zero(); NUM_TIMERS],
            timers_active: [false; NUM_TIMERS],
        }
    }
}

impl DataTrackingPage {
    pub fn view(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Press Number Keys to Start/Stop Timers");
            ui.ctx().input(|i| {
                for (idx, key) in [
                    egui::Key::Num1,
                    egui::Key::Num2,
                    egui::Key::Num3,
                    egui::Key::Num4,
                    egui::Key::Num5,
                ]
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
            for i in 0..NUM_TIMERS {
                ui.horizontal(|ui| {
                    ui.label(format!("Timer {}", i + 1));
                    ui.label(format!(
                        "Total: {:.1}",
                        self.total_times[i].as_seconds_f32()
                    ));
                    ui.add_space(3.0);
                    if self.timers_active[i] {
                        ui.request_repaint();
                        ui.label(format!(
                            "Current: {:.1}",
                            (Local::now() - self.init_times[i]).as_seconds_f32()
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
