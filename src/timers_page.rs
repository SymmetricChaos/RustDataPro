use crate::timer::Timer;
use egui::{Key, Ui};

const NUM_TIMERS: usize = 5;

pub struct Timers {
    timers: [Timer; NUM_TIMERS],
    linked_timers: [bool; NUM_TIMERS],
}

impl Default for Timers {
    fn default() -> Self {
        Self {
            timers: [
                Timer::new(),
                Timer::new(),
                Timer::new(),
                Timer::new().split(),
                Timer::new().split(),
            ],
            linked_timers: [false; NUM_TIMERS],
        }
    }
}

impl Timers {
    pub fn view(&mut self, ui: &mut Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Timers");

            ui.ctx().input(|i| {
                if i.num_presses(Key::Num1) > 0 {
                    self.timers[0].toggle()
                }
                if i.num_presses(Key::Num2) > 0 {
                    self.timers[1].toggle()
                }
                if i.num_presses(Key::Num3) > 0 {
                    self.timers[2].toggle()
                }
                if i.num_presses(Key::Num4) > 0 {
                    self.timers[3].toggle()
                }
                if i.num_presses(Key::Num5) > 0 {
                    self.timers[4].toggle()
                }
            });

            ui.group(|ui| {
                ui.label("Linked");
                ui.horizontal(|ui| {
                    if ui.button("Start").clicked() {
                        for (timer, linked) in
                            self.timers.iter_mut().zip(self.linked_timers.iter_mut())
                        {
                            if *linked {
                                timer.start();
                            }
                        }
                    };
                    if ui.button("Stop").clicked() {
                        for (timer, linked) in
                            self.timers.iter_mut().zip(self.linked_timers.iter_mut())
                        {
                            if *linked {
                                timer.stop();
                            }
                        }
                    };
                    if ui.button("Toggle").clicked() {
                        for (timer, linked) in
                            self.timers.iter_mut().zip(self.linked_timers.iter_mut())
                        {
                            if *linked {
                                timer.toggle();
                            }
                        }
                    };
                });
            });

            for (n, (timer, linked)) in self
                .timers
                .iter_mut()
                .zip(self.linked_timers.iter_mut())
                .enumerate()
            {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(5.0);
                    ui.label(format!("{})", n + 1));
                    timer.view(ui);
                    if ui.button("⏱️").clicked() {
                        timer.toggle();
                    }
                    ui.checkbox(linked, "");
                });
            }
        });
    }
}
