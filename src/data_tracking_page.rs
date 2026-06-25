use chrono::{DateTime, Duration, Local};
use egui::Ui;

pub struct DataTrackingPage {
    init_times: [DateTime<Local>; 5],
    total_times: [Duration; 5],
    timers_active: [bool; 5],
}

impl Default for DataTrackingPage {
    fn default() -> Self {
        Self {
            init_times: [Local::now(); 5],
            total_times: [Duration::zero(); 5],
            timers_active: [false; 5],
        }
    }
}

impl DataTrackingPage {
    pub fn view(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Press Space to Stop/Start Time 1");
            ui.ctx().input(|i| {
                if i.key_pressed(egui::Key::Space) {
                    if self.timers_active[0] {
                        self.total_times[0] += Local::now() - self.init_times[0];
                        self.timers_active[0] = false;
                    } else {
                        self.init_times[0] = Local::now();
                        self.timers_active[0] = true;
                    }
                }
            });
            for i in 0..5 {
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
