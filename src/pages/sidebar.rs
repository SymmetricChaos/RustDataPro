use crate::{app::DataPro, utils::DataProUiElements};
use egui::{Ui, warn_if_debug_build};

pub struct Sidebar {}

impl Sidebar {
    pub fn view(app: &mut DataPro, ui: &mut Ui) {
        egui::Panel::left("welcome_panel")
            .default_size(200.0)
            .min_size(200.0)
            .show(ui, |ui| {
                warn_if_debug_build(ui);
                ui.strong("Welcome to RustDataPro!");
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("Powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });

                ui.add_space(20.0);
                ui.horizontal(|ui| {
                    ui.label("Visual Scaling");
                    if ui
                        .add(
                            egui::DragValue::new(&mut app.display_info.zoom)
                                .range(1.0..=2.0)
                                .speed(0.1)
                                .fixed_decimals(1),
                        )
                        .lost_focus()
                    {
                        ui.ctx().set_pixels_per_point(app.display_info.zoom);
                    }
                });

                ui.add_space(20.0);
                ui.label("Other Useful Functionality");
                ui.add_space(5.0);
                if ui.large_blue_button("Randomness").clicked() {
                    app.display_info.toggle_random_display();
                }

                ui.add_space(5.0);
                if ui.large_blue_button("Timers").clicked() {
                    app.display_info.toggle_timer_display();
                }

                ui.add_space(5.0);
                if ui.large_blue_button("Calculate IOA").clicked() {
                    app.display_info.go_to_reliability();
                }

                ui.add_space(5.0);
                if ui.large_blue_button("Create Client").clicked() {
                    app.display_info.go_to_new_client();
                }
            });
    }
}
