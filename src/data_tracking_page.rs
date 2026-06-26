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
            ui.heading("Press Space to Stop/Start Time 1");
            ui.ctx().input(|i| {
                if i.key_released(egui::Key::Space) {
                    if self.timers_active[0] {
                        self.total_times[0] += Local::now() - self.init_times[0];
                        self.timers_active[0] = false;
                    } else {
                        self.init_times[0] = Local::now();
                        self.timers_active[0] = true;
                    }
                }
            });
            for i in 0..NUM_TIMERS {
                if ui.button(format!("Timer {}", i + 1)).clicked() {
                    if self.timers_active[i] {
                        self.total_times[i] += Local::now() - self.init_times[i];
                        self.timers_active[i] = false;
                    } else {
                        self.init_times[i] = Local::now();
                        self.timers_active[i] = true;
                    }
                }
                ui.horizontal(|ui| {
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
