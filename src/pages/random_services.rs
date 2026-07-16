use egui::{DragValue, TextEdit, Ui};
use itertools::Itertools;
use rand::{make_rng, rngs::StdRng, seq::SliceRandom};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Service {
    Numbers,
    Shuffle,
}

impl Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Service::Numbers => write!(f, "Numbers"),
            Service::Shuffle => write!(f, "Shuffle"),
        }
    }
}

pub struct RandomServices {
    prng: StdRng,
    min_rand: usize,
    max_rand: usize,
    random_nums: String,
    shuffle_list: String,
    service: Service,
}

impl Default for RandomServices {
    fn default() -> Self {
        Self {
            prng: make_rng(),
            min_rand: 1,
            max_rand: 5,
            random_nums: String::new(),
            shuffle_list: String::from("a, b, c, 1, 2, 3"),
            service: Service::Shuffle,
        }
    }
}

impl RandomServices {
    fn view_number_generation(&mut self, ui: &mut Ui) {
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
        ui.add_space(5.0);

        if ui.button("Generate").clicked() {
            let mut v = Vec::new();
            for i in self.min_rand..=self.max_rand {
                v.push(i);
            }

            v.shuffle(&mut self.prng);

            let list = v.iter().map(|n| n.to_string()).join(", ");

            if !self.random_nums.is_empty() {
                self.random_nums.push('\n');
            }

            self.random_nums.push_str(&list);
        }
        ui.add(
            TextEdit::multiline(&mut self.random_nums)
                .desired_width(300.0)
                .desired_rows(4),
        );
        ui.add_space(10.0);
    }

    fn view_shuffler(&mut self, ui: &mut Ui) {
        ui.heading("Shuffle a List");
        ui.label("Separate items with commas.");
        if ui.button("Shuffle").clicked() {
            let mut list: Vec<&str> = self.shuffle_list.split(',').collect();
            list.shuffle(&mut self.prng);
            let rep = list
                .iter()
                .map(|s| s.trim())
                .filter(|s| s.len() > 0)
                .join(", ");
            self.shuffle_list = rep;
        }
        ui.add(
            TextEdit::multiline(&mut self.shuffle_list)
                .desired_width(300.0)
                .desired_rows(4),
        );
    }

    pub fn view(&mut self, ui: &mut Ui, open: &mut bool) {
        egui::Window::new("Random").open(open).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("Service:");
                egui::ComboBox::from_id_salt("random_service_selector")
                    .selected_text(self.service.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.service, Service::Shuffle, "Shuffle");
                        ui.selectable_value(&mut self.service, Service::Numbers, "Numbers");
                    });
            });

            match self.service {
                Service::Numbers => self.view_number_generation(ui),
                Service::Shuffle => self.view_shuffler(ui),
            }
        });
    }
}
