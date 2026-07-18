use crate::{
    app::DataPro,
    data::{ClientData, KsfData},
    utils::DataProUiElements,
};
use egui::{Ui, warn_if_debug_build};
use egui_file_dialog::FileDialog;

pub struct Sidebar {}

impl Sidebar {
    pub fn view(app: &mut DataPro, ui: &mut Ui) {
        app.pick_root_directory.update(ui.ctx());
        if let Some(pathbuf) = app.pick_root_directory.take_picked() {
            // If we change root directory then we set the client picker to look there and reet the ksf picker entirely
            app.root_directory = pathbuf.clone();
            app.pick_client_folder = FileDialog::new().initial_directory(pathbuf);
            app.pick_ksf = FileDialog::new();
            // Also reset the client data and ksf data to avoid confusion
            app.data.client = ClientData::default();
            app.data.ksf = KsfData::default();
            app.data.ksf_name.clear();
        }
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

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                if ui.large_blue_button("Change Directory").clicked() {
                    app.pick_root_directory.pick_directory();
                }
                ui.add_space(5.0);

                if ui.large_blue_button("Calculate IOA").clicked() {
                    app.display_info.go_to_reliability();
                }
                ui.add_space(5.0);

                if ui.large_blue_button("Create Client").clicked() {
                    app.display_info.go_to_new_client();
                }
                ui.add_space(5.0);

                if ui.large_blue_button("Create KSF").clicked() {
                    app.display_info.go_to_new_ksf();
                }

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

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
            });
    }
}
