use crate::{data_tracking::timer::Timer, utils::ClickedKeys};
use egui::{Key, Ui};

const NUM_TIMERS: usize = 5;

pub struct Timers {
    timers: [Timer; NUM_TIMERS],
    linked_timers: [bool; NUM_TIMERS],
    clicked_keys: ClickedKeys,
}

impl Default for Timers {
    fn default() -> Self {
        Self {
            timers: [
                Timer::new().with_bouts(),
                Timer::new().with_bouts(),
                Timer::new().with_bouts(),
                Timer::new().with_bouts(),
                Timer::new().with_bouts(),
            ],
            linked_timers: [true, true, false, false, false],
            clicked_keys: ClickedKeys::new(),
        }
    }
}

impl Timers {
    fn timers_and_links(&mut self) -> impl Iterator<Item = (&mut Timer, &mut bool)> {
        self.timers.iter_mut().zip(self.linked_timers.iter_mut())
    }

    pub fn view(&mut self, ui: &mut Ui, open: &mut bool) {
        egui::Window::new("Timers").open(open).show(ui, |ui| {
            ui.ctx().input_mut(|input| {
                self.clicked_keys.update(input);

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

            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui.button("Reset All").clicked() {
                    for timer in self.timers.iter_mut() {
                        timer.reset();
                    }
                }
                if ui.button("Stop All").clicked() {
                    for timer in self.timers.iter_mut() {
                        timer.stop();
                    }
                }
                if ui.button("Start All").clicked() {
                    for timer in self.timers.iter_mut() {
                        timer.start();
                    }
                }
            });
            ui.add_space(10.0);
            ui.label("Press 1-5 to toggle timers.\n\nPress 0 to toggle linked timers.");
            ui.add_space(10.0);
            egui::Grid::new("timers_page_grid")
                .striped(true)
                .show(ui, |ui| {
                    ui.label("");
                    ui.label("Time");
                    ui.label("Bouts");
                    ui.label("");
                    ui.label("");
                    ui.end_row();

                    for (n, (timer, linked)) in self.timers_and_links().enumerate() {
                        ui.label(format!("{})", n + 1));
                        timer.view(ui);
                        if ui.button("x").on_hover_text("reset").clicked() {
                            timer.reset();
                        }
                        ui.checkbox(linked, "").on_disabled_hover_text("linked");
                        ui.end_row();
                    }
                });
        });
    }
}
