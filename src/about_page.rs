use chrono::{Datelike, Local, Timelike};
use egui::{RichText, Ui, warn_if_debug_build};
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

pub struct About {
    file_dialog: FileDialog,
    picked_file: Option<PathBuf>,
}

impl Default for About {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::new(),
            picked_file: None,
        }
    }
}

impl About {
    pub fn view(&mut self, ui: &mut Ui) {
        egui::Panel::left("about_display_panel")
            .default_size(500.0)
            .min_size(200.0)
            .show_inside(ui, |ui| {
                warn_if_debug_build(ui);
                let hello = RichText::new("Welcome to RustDataPro!").strong();
                ui.label(hello);
                ui.add_space(20.0);
                // ui.hyperlink_to(
                //     "source code",
                //     "https://github.com/SymmetricChaos/crypto-gui",
                // );
                // ui.add_space(10.0);
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
            });
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.label(
                RichText::new("Data Collection and Management")
                    .heading()
                    .strong(),
            );
            let dt = Local::now();
            ui.request_repaint_after_secs(5.0);
            ui.label(format!(
                "The Current Date/Time is {} {}/{}/{} {:02}:{:02}",
                dt.weekday(),
                dt.month(),
                dt.day(),
                dt.year(),
                dt.hour(),
                dt.minute(),
            ));

            if ui.button("Pick file").clicked() {
                // Open the file dialog to pick a file.
                self.file_dialog.pick_file();
            }

            ui.label(format!("Picked file: {:?}", self.picked_file));

            // Check if the user picked a file.
            if let Some(path) = self.file_dialog.take_picked() {
                self.picked_file = Some(path.to_path_buf());
            }
        });
    }
}
