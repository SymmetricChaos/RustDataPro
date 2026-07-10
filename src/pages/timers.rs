use crate::{
    data::{Timer, view_simple_countdown_timer, view_simple_timer},
    utils::ClickedKeys,
};
use egui::{Key, Ui};

const NUM_TIMERS: usize = 5;

pub struct Timers {
    timers: [Timer; NUM_TIMERS],
    linked_timers: [bool; NUM_TIMERS],
    countdown: bool,
    clicked_keys: ClickedKeys,
}

impl Default for Timers {
    fn default() -> Self {
        Self {
            timers: [
                Timer::default(),
                Timer::default(),
                Timer::default(),
                Timer::default(),
                Timer::default(),
            ],
            linked_timers: [true, true, false, false, false],
            countdown: false,
            clicked_keys: ClickedKeys::new(),
        }
    }
}

impl Timers {
    pub fn stop_all_timers(&mut self) {
        for timer in self.timers.iter_mut() {
            timer.stop();
        }
    }

    pub fn reset_all_timers(&mut self) {
        for timer in self.timers.iter_mut() {
            timer.reset();
        }
    }

    pub fn switch_countdown(&mut self) {
        self.reset_all_timers();
        self.countdown = !self.countdown;
    }

    fn timers_and_links(&mut self) -> impl Iterator<Item = (&mut Timer, &mut bool)> {
        self.timers.iter_mut().zip(self.linked_timers.iter_mut())
    }

    pub fn view(&mut self, ui: &mut Ui, open: &mut bool) {
        let countdown = self.countdown;
        let mut allow_keys = *open;

        egui::Window::new("Timers").open(open).show(ui, |ui| {
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Reset All").clicked() {
                    self.reset_all_timers()
                }
                if ui.button("Stop All").clicked() {
                    self.stop_all_timers()
                }
                if self.countdown {
                    if ui.button("Switch to Countup").clicked() {
                        self.switch_countdown()
                    }
                } else {
                    if ui.button("Switch to Countdown").clicked() {
                        self.switch_countdown()
                    }
                }
            });
            ui.add_space(10.0);

            ui.label(
                "Keyboard Controls:\n1-5 to toggle timers.\n0 to toggle linked timers.\nSpace to stop all timers.",
            );
            ui.add_space(10.0);

            egui::Grid::new("timers_page_grid")
                .striped(true)
                .show(ui, |ui| {
                    for (n, (timer, linked)) in self.timers_and_links().enumerate() {
                        ui.monospace(format!("{})", n + 1));

                        if countdown {
                            let draginfo = ui.add(egui::DragValue::new(&mut timer.countdown_from).range(0.0..=9999.0));
                            if draginfo.has_focus() {
                                allow_keys = false;
                            }
                            if draginfo.changed() {
                                timer.reset();
                            }
                            view_simple_countdown_timer(ui, timer);
                        } else {
                            view_simple_timer(ui, timer);
                        }

                        if ui.button("❌").on_hover_text("reset").clicked() {
                            timer.reset();
                        }
                        if *linked {
                            ui.checkbox(linked, "").on_hover_text("linked");
                        } else {
                            ui.checkbox(linked, "").on_hover_text("unlinked");
                        }

                        ui.end_row();
                    }
                });
            ui.add_space(10.0);
        });
        if allow_keys {
            ui.ctx().input_mut(|input| {
                self.clicked_keys.update(input);

                if self.clicked_keys.contains(&Key::Space) {
                    self.stop_all_timers();
                }

                // Detect toggle linked
                if self.clicked_keys.contains(&Key::Num0) {
                    for (timer, linked) in self.timers_and_links() {
                        if *linked {
                            timer.toggle();
                        }
                    }
                }

                // Detect toggle each
                for (idx, key) in [Key::Num1, Key::Num2, Key::Num3, Key::Num4, Key::Num5]
                    .iter()
                    .enumerate()
                {
                    if self.clicked_keys.contains(key) {
                        self.timers[idx].toggle()
                    }
                }
            });
        }
    }
}
