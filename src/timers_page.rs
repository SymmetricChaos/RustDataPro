use egui::Ui;
use crate::timer::Timer;

const NUM_TIMERS: usize = 10;

pub struct Timers {
    timers: [Timer; NUM_TIMERS],
}

impl Default for Timers {
    fn default() -> Self {
        Self { timers: [Timer::new(), Timer::new(), Timer::new(), Timer::new(), Timer::new(), Timer::new(), Timer::new(), Timer::new(), Timer::new(), Timer::new()] }
    }
}

impl Timers {
    pub fn view(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Timers");


            if ui.button("Start All Timers").clicked() {
                for timer in self.timers.iter_mut() {
                    timer.start();
                }
            };
            if ui.button("Stop All Timers").clicked() {
                for timer in self.timers.iter_mut() {
                    timer.stop();
                }
            };
                            
            for timer in self.timers.iter_mut() {
                if ui.button("toggle").clicked() {
                    timer.toggle();
                }
                timer.view(ui)
            }
            
        });
    }
}