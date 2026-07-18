use egui::{DragValue, TextEdit, Ui};
use itertools::Itertools;
use rand::{make_rng, rngs::StdRng, seq::SliceRandom};
use std::fmt::Display;

use crate::app::DataPro;

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

fn view_number_generation(page: &mut RandomServices, ui: &mut Ui) {
    ui.heading("Random Numbers in a Range");
    ui.horizontal(|ui| {
        ui.add_space(2.0);
        ui.label("min");
        if ui
            .add(DragValue::new(&mut page.min_rand).range(0..=99))
            .changed()
        {
            if page.min_rand >= page.max_rand {
                page.max_rand = page.min_rand + 1;
            }
        };
        ui.add_space(2.0);
        ui.label("max");
        if ui
            .add(DragValue::new(&mut page.max_rand).range(1..=100))
            .changed()
        {
            if page.min_rand >= page.max_rand {
                page.min_rand = page.max_rand - 1;
            }
        };
    });
    ui.add_space(5.0);

    if ui.button("Generate").clicked() {
        let mut v = Vec::new();
        for i in page.min_rand..=page.max_rand {
            v.push(i);
        }

        v.shuffle(&mut page.prng);

        let list = v.iter().map(|n| n.to_string()).join(", ");

        if !page.random_nums.is_empty() {
            page.random_nums.push('\n');
        }

        page.random_nums.push_str(&list);
    }
    ui.add(
        TextEdit::multiline(&mut page.random_nums)
            .desired_width(300.0)
            .desired_rows(4),
    );
    ui.add_space(10.0);
}

fn view_shuffler(page: &mut RandomServices, ui: &mut Ui) {
    ui.heading("Shuffle a List");
    ui.label("Separate items with commas.");
    if ui.button("Shuffle").clicked() {
        let mut list: Vec<&str> = page.shuffle_list.split(',').collect();
        list.shuffle(&mut page.prng);
        let rep = list
            .iter()
            .map(|s| s.trim())
            .filter(|s| s.len() > 0)
            .join(", ");
        page.shuffle_list = rep;
    }
    ui.add(
        TextEdit::multiline(&mut page.shuffle_list)
            .desired_width(300.0)
            .desired_rows(4),
    );
}

pub struct RandomServices {
    prng: StdRng, // ChaCha12 is more than enough for our purposes, initalized from SysRng
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
    pub fn view(app: &mut DataPro, ui: &mut Ui) {
        egui::Window::new("Random")
            .open(&mut app.display_info.random_open)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Service:");
                    egui::ComboBox::from_id_salt("random_service_selector")
                        .selected_text(app.randomness_page.service.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut app.randomness_page.service,
                                Service::Shuffle,
                                "Shuffle",
                            );
                            ui.selectable_value(
                                &mut app.randomness_page.service,
                                Service::Numbers,
                                "Numbers",
                            );
                        });
                });

                match app.randomness_page.service {
                    Service::Numbers => view_number_generation(&mut app.randomness_page, ui),
                    Service::Shuffle => view_shuffler(&mut app.randomness_page, ui),
                }
            });
    }
}
