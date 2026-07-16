use crate::{app::DisplayInfo, data::ClientData, utils::DataProUiElements};
use anyhow::Result;
use egui_file_dialog::FileDialog;
use std::{
    fs::File,
    io::{BufWriter, Write},
};

pub struct NewClient {
    client: ClientData,
    assessments: String,
    conditions: String,
    file_dialog: FileDialog,
    error: String,
}

impl Default for NewClient {
    fn default() -> Self {
        Self {
            client: ClientData::blank(),
            assessments: String::new(),
            conditions: String::new(),
            file_dialog: FileDialog::new().default_file_name("ClientName.txt"),
            error: String::new(),
        }
    }
}

impl NewClient {
    pub fn save_file(&mut self) -> Result<()> {
        if let Some(path) = self.file_dialog.take_picked() {
            let mut writer = BufWriter::new(File::create_new(path)?);
            writer.write_all(
                self.client
                    .to_json()
                    .expect("failed to create json")
                    .as_bytes(),
            )?;
            writer.flush()?;
        }
        Ok(())
    }

    pub fn view(&mut self, ui: &mut egui::Ui, display_info: &mut DisplayInfo) {
        self.file_dialog.update(ui.ctx());

        egui::CentralPanel::default().show(ui, |ui| {
            egui::Grid::new("client_and_session_info_grid")
                .min_col_width(150.0)
                .show(ui, |ui| {
                    ui.monospace("Client Name");
                    ui.text_edit_singleline(&mut self.client.name);
                    ui.end_row();

                    ui.monospace("ID");
                    ui.text_edit_singleline(&mut self.client.id);
                    ui.end_row();

                    ui.monospace("Case Manager");
                    ui.text_edit_singleline(&mut self.client.case_manager);
                    ui.end_row();

                    ui.monospace("Primary Therapist");
                    ui.text_edit_singleline(&mut self.client.primary_therapist);
                    ui.end_row();

                    ui.monospace("Assessments\n(separate with commas)");
                    if ui.text_edit_multiline(&mut self.assessments).changed() {
                        self.client.assessments = self
                            .assessments
                            .split(",")
                            .map(|s| String::from(s.trim()))
                            .collect();
                    }
                    ui.end_row();

                    ui.monospace("Conditions\n(separate with commas)");
                    if ui.text_edit_multiline(&mut self.conditions).changed() {
                        self.client.conditions = self
                            .conditions
                            .split(",")
                            .map(|s| String::from(s.trim()))
                            .collect();
                    }
                    ui.end_row();
                });

            if ui.large_green_button("Save").clicked() {
                self.file_dialog.save_file();
                match self.save_file() {
                    Ok(_) => self.error.clear(),
                    Err(e) => self.error = e.to_string(),
                }
            }

            if ui.large_red_button("Return").clicked() {
                display_info.go_to_about();
            }
        });
    }
}
