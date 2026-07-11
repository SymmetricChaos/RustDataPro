use crate::{
    data::{Timer, view_simple_countdown_timer, view_simple_timer},
    utils::ClickedKeys,
};
use egui::{
    Key::{self},
    Ui,
};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TimerType {
    Countdown,
    Stopwatch,
}

impl Display for TimerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerType::Countdown => write!(f, "Countdown"),
            TimerType::Stopwatch => write!(f, "Stopwatch"),
        }
    }
}

const NUM_TIMERS: usize = 5;

pub struct Timers {
    timers: [Timer; NUM_TIMERS],
    linked_timers: [bool; NUM_TIMERS],
    clicked_keys: ClickedKeys,
    timer_type: TimerType,
}

impl Default for Timers {
    fn default() -> Self {
        Self {
            timers: [Timer::default(); NUM_TIMERS],
            linked_timers: [true, true, false, false, false],
            clicked_keys: ClickedKeys::new(),
            timer_type: TimerType::Stopwatch,
        }
    }
}

impl Timers {
    pub fn pause_all_timers(&mut self) {
        for timer in self.timers.iter_mut() {
            if timer.was_started() {
                timer.pause();
            }
        }
    }

    pub fn reset_all_timers(&mut self) {
        for timer in self.timers.iter_mut() {
            timer.reset();
        }
    }

    fn timers_and_links(&mut self) -> impl Iterator<Item = (&mut Timer, &mut bool)> {
        self.timers.iter_mut().zip(self.linked_timers.iter_mut())
    }

    pub fn view(&mut self, ui: &mut Ui, open: &mut bool) {
        let timer_type = self.timer_type;
        let mut allow_keys = *open;

        egui::Window::new("Timers").open(open).show(ui, |ui| {
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Reset All").clicked() {
                    self.reset_all_timers()
                }
                if ui.button("Pause All").clicked() {
                    self.pause_all_timers()
                }
                egui::ComboBox::from_id_salt("timer_type_selector")
                    .selected_text(self.timer_type.to_string())
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(
                                &mut self.timer_type,
                                TimerType::Stopwatch,
                                "Stopwatch",
                            )
                            .clicked()
                        {
                            self.reset_all_timers();
                        }
                        if ui
                            .selectable_value(
                                &mut self.timer_type,
                                TimerType::Countdown,
                                "Countdown",
                            )
                            .clicked()
                        {
                            self.reset_all_timers();
                        };
                    });
            });
            ui.add_space(10.0);

            ui.strong("Keyboard Controls:");
            ui.label(
                "1-5 to toggle timers.\n0 to toggle linked timers.\nSpace to stop all timers.",
            );
            ui.add_space(10.0);

            egui::Grid::new("timers_page_grid")
                .striped(true)
                .show(ui, |ui| {
                    for (n, (timer, linked)) in self.timers_and_links().enumerate() {
                        ui.horizontal(|ui| {
                            ui.monospace(format!("{})", n + 1));
                            ui.add_space(10.0);
                            if timer_type == TimerType::Countdown {
                                view_simple_countdown_timer(ui, timer);
                            } else {
                                view_simple_timer(ui, timer);
                            }
                            ui.add_space(5.0);
                            if ui.button("❌").on_hover_text("reset").clicked() {
                                timer.reset();
                            }
                            ui.add_space(5.0);
                            if *linked {
                                if ui.button("☑").on_hover_text("linked").clicked() {
                                    *linked = !*linked;
                                }
                            } else {
                                if ui.button("☐").on_hover_text("unlinked").clicked() {
                                    *linked = !*linked;
                                }
                            }
                            ui.add_space(5.0);
                            if timer_type == TimerType::Countdown {
                                let draginfo = ui.add(
                                    egui::DragValue::new(&mut timer.countdown_from)
                                        .range(0.0..=9999.0),
                                );
                                if draginfo.has_focus() {
                                    allow_keys = false;
                                }
                                if draginfo.changed() {
                                    timer.reset();
                                }
                            }
                        });

                        ui.end_row();
                    }
                });
            ui.add_space(10.0);
        });
        if allow_keys {
            ui.ctx().input_mut(|input| {
                self.clicked_keys.update(input);

                if self.clicked_keys.contains(&Key::Space) {
                    self.pause_all_timers();
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
