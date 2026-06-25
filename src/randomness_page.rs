use crate::prng::Prng;
use chrono::Local;
use egui::{CentralPanel, Context, DragValue, TextEdit};
use itertools::Itertools;

pub struct RandomnessPage {
    prng: Prng,
    min_rand: usize,
    max_rand: usize,
    random_nums: String,
    shuffle_list: String,
}

impl Default for RandomnessPage {
    fn default() -> Self {
        Self {
            prng: Prng::new(Local::now().timestamp_micros() as u64),
            min_rand: 1,
            max_rand: 5,
            random_nums: String::new(),
            shuffle_list: String::new(),
        }
    }
}

impl RandomnessPage {
    pub fn view(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Random Numbers in a Range");
            ui.horizontal(|ui| {
                ui.add_space(2.0);
                ui.label("min");
                if ui
                    .add(DragValue::new(&mut self.min_rand).range(0..=99))
                    .changed()
                {
                    if self.min_rand >= self.max_rand {
                        self.max_rand = self.min_rand + 1;
                    }
                };
                ui.add_space(2.0);
                ui.label("max");
                if ui
                    .add(DragValue::new(&mut self.max_rand).range(1..=100))
                    .changed()
                {
                    if self.min_rand >= self.max_rand {
                        self.min_rand = self.max_rand - 1;
                    }
                };
            });

            if ui.button("Generate").clicked() {
                let mut v = Vec::new();
                for i in self.min_rand..=self.max_rand {
                    v.push(i);
                }

                self.prng.shuffle(&mut v);

                let list = v.iter().map(|n| n.to_string()).join(", ");

                if !self.random_nums.is_empty() {
                    self.random_nums.push('\n');
                }

                self.random_nums.push_str(&list);
            }
            ui.add(
                TextEdit::multiline(&mut self.random_nums)
                    .desired_width(400.0)
                    .desired_rows(2),
            );
            ui.add_space(10.0);

            ui.heading("Shuffle a List");
            ui.label("Separate items with commas.");
            if ui.button("Shuffle").clicked() {
                let substrs = self.shuffle_list.split(',');
                let mut list: Vec<&str> = substrs.collect();
                self.prng.shuffle(&mut list);
                let rep = list
                    .iter()
                    .map(|s| s.trim())
                    .filter(|s| s.len() > 0)
                    .join(", ");
                self.shuffle_list = rep;
            }
            ui.add(
                TextEdit::multiline(&mut self.shuffle_list)
                    .desired_width(400.0)
                    .desired_rows(8),
            );
        });
    }
}
