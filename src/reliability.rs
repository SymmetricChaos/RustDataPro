use crate::{
    app::{Data, DisplayInfo},
    utils::DataProUiElements,
};
use egui::Ui;
use egui_file_dialog::FileDialog;

pub struct ReliabilityPage {
    primary_file_dialog: FileDialog,
    reli_file_dialog: FileDialog,
}

impl Default for ReliabilityPage {
    fn default() -> Self {
        Self {
            primary_file_dialog: Default::default(),
            reli_file_dialog: Default::default(),
        }
    }
}

impl ReliabilityPage {
    pub fn view(&mut self, ui: &mut Ui, display_info: &mut DisplayInfo) {
        egui::CentralPanel::default().show(ui, |ui| {
            if ui.large_button("Select Primary").clicked() {
                // TODO
            }
            if ui.large_button("Select Reliability").clicked() {
                // TODO
            }
        });
    }
}
