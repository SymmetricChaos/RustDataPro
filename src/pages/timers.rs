use crate::{
    data::{Timer, view_simple_countdown_timer, view_simple_timer},
    utils::ClickedKeys,
};
use egui::{
    Key::{self},
    TextStyle, Ui,
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
    timers: Vec<(Timer, bool, String)>,
    clicked_keys: ClickedKeys,
    timer_type: TimerType,
}

impl Default for Timers {
    fn default() -> Self {
        let mut timers = Vec::new();
        for _ in 0..NUM_TIMERS {
            timers.push((Timer::default(), false, String::new()));
        }
        Self {
            timers,
            clicked_keys: ClickedKeys::new(),
            timer_type: TimerType::Countdown,
        }
    }
}

impl Timers {
    pub fn pause_all_timers(&mut self) {
        for (timer, _, _) in self.timers.iter_mut() {
            if timer.was_started() {
                timer.pause();
            }
        }
    }

    pub fn reset_all_timers(&mut self) {
        for (timer, _, _) in self.timers.iter_mut() {
            timer.reset();
        }
    }

    pub fn view(&mut self, ui: &mut Ui, open: &mut bool) {
        let timer_type = self.timer_type;
        let mut accept_keyboard_controls = *open;

        egui::Window::new("Timers").open(open).show(ui, |ui| {
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("Reset All").clicked() {
                    self.reset_all_timers()
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
                "1-5 to toggle timers.\n0 to toggle linked timers.\nSpace to pause all timers.",
            );
            ui.add_space(10.0);

            egui::Grid::new("timers_page_grid")
                .striped(true)
                .show(ui, |ui| {
                    for (n, (timer, linked, name)) in self.timers.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            if ui
                                .add_sized(
                                    (110.0, 20.0),
                                    egui::TextEdit::singleline(name)
                                        .prefix(format!("{})", n + 1))
                                        .char_limit(11)
                                        .font(TextStyle::Monospace),
                                )
                                .has_focus()
                            {
                                accept_keyboard_controls = false;
                            };
                            ui.add_space(10.0);
                            if timer_type == TimerType::Countdown {
                                view_simple_countdown_timer(ui, timer);
                            } else {
                                view_simple_timer(ui, timer);
                            }
                            ui.add_space(5.0);
                            if ui.button("⟳").on_hover_text("reset").clicked() {
                                timer.reset();
                            }
                            ui.add_space(5.0);
                            if *linked {
                                ui.checkbox(linked, "").on_hover_text("linked");
                            } else {
                                ui.checkbox(linked, "").on_hover_text("unlinked");
                            }
                            ui.add_space(5.0);
                            if timer_type == TimerType::Countdown {
                                let draginfo = ui.add(
                                    egui::DragValue::new(&mut timer.countdown_from)
                                        .range(0.0..=9999.0),
                                );
                                if draginfo.has_focus() {
                                    accept_keyboard_controls = false;
                                }
                                if draginfo.changed() {
                                    timer.reset();
                                }
                            }
                            ui.add_space(5.0);
                        });

                        ui.end_row();
                    }
                });
            ui.add_space(10.0);
        });
        if accept_keyboard_controls {
            ui.ctx().input_mut(|input| {
                self.clicked_keys.update(input);

                if self.clicked_keys.contains(&Key::Space) {
                    self.pause_all_timers();
                }

                // Detect toggle linked
                if self.clicked_keys.contains(&Key::Num0) {
                    for (timer, linked, _) in self.timers.iter_mut() {
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
                        self.timers[idx].0.toggle()
                    }
                }
            });
        }
    }
}
