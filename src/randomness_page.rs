use crate::prng::Prng;
use chrono::Local;
use egui::{CentralPanel, Context, Slider, TextEdit};
use itertools::Itertools;

pub struct RandomnessPage {
    prng: Prng,
    text: String,
    min_rand: usize,
    max_rand: usize,
    random_nums: String,
}

impl Default for RandomnessPage {
    fn default() -> Self {
        Self {
            prng: Prng::new(Local::now().timestamp() as u64), // TODO: seed from system time
            text: String::new(),
            min_rand: 1,
            max_rand: 5,
            random_nums: String::new(),
        }
    }
}

impl RandomnessPage {
    pub fn view(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            //     ui.add(
            //         TextEdit::multiline(&mut self.text)
            //             .desired_width(800.0)
            //             .desired_rows(10),
            //     );
            //     if ui.button("📋").clicked() {
            //         ui.ctx().copy_text(self.text.to_string());
            //     };

            ui.label(format!(
                "Random Numbers in the Range {} to {}",
                self.min_rand, self.max_rand
            ));
            if ui.add(Slider::new(&mut self.min_rand, 0..=19)).changed() {
                if self.min_rand >= self.max_rand {
                    self.max_rand = self.min_rand + 1;
                }
            };
            if ui.add(Slider::new(&mut self.max_rand, 1..=20)).changed() {
                if self.min_rand >= self.max_rand {
                    self.min_rand = self.max_rand - 1;
                }
            };
            if ui.button("Generate").clicked() {
                let range = self.max_rand - self.min_rand + 1;
                let mut v = Vec::new();
                for i in self.min_rand..=self.max_rand {
                    v.push(i);
                }

                for i in 0..range {
                    let swap_pos = self.prng.next_u64() as usize % range;
                    v.swap(i, swap_pos);
                }

                self.random_nums = v.iter().map(|n| n.to_string()).join(", ");
            }
            ui.add(
                TextEdit::multiline(&mut self.random_nums)
                    .desired_width(400.0)
                    .desired_rows(2),
            );
        });
    }
}
