use crate::{
    app::{CLIENT_DATA_FILE_NAME, DisplayInfo},
    data::ClientData,
    utils::DataProUiElements,
};
use anyhow::Result;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

pub struct NewClient {
    client: ClientData,
    assessments: String,
    conditions: String,
    error: String,
}

impl Default for NewClient {
    fn default() -> Self {
        Self {
            client: ClientData::default(),
            assessments: String::new(),
            conditions: String::new(),
            error: String::new(),
        }
    }
}

impl NewClient {
    fn save_file_to_path(&mut self, client_directory_path: &PathBuf) -> Result<()> {
        if self.client.id.is_empty() {
            return Err(anyhow::anyhow!(
                "client must have an ID assigned in order to create a new file"
            ));
        }

        // Create a new directory for the client inside the client director
        let mut p = client_directory_path.clone();
        p.push(&format!("{}", self.client.id));
        std::fs::create_dir(&p)?;

        // Create the client file inside the new directory, title it client_data.txt
        p.push(CLIENT_DATA_FILE_NAME);
        let mut writer = BufWriter::new(File::create_new(p)?);
        writer.write_all(self.client.to_json()?.as_bytes())?;
        writer.flush()?;

        Ok(())
    }

    pub fn view(
        &mut self,
        ui: &mut egui::Ui,
        display_info: &mut DisplayInfo,
        client_directory_path: &PathBuf,
    ) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Create a New Client");
            ui.add_space(10.0);
            egui::Grid::new("client_and_session_info_grid")
                .min_col_width(150.0)
                .spacing((10.0, 10.0))
                .show(ui, |ui| {
                    ui.monospace("Client Name");
                    ui.text_edit_singleline(&mut self.client.name);
                    ui.end_row();

                    ui.monospace("Client ID");
                    ui.text_edit_singleline(&mut self.client.id);
                    ui.end_row();

                    ui.monospace("Case Manager");
                    ui.text_edit_singleline(&mut self.client.case_manager);
                    ui.end_row();

                    ui.monospace("Primary Therapist");
                    ui.text_edit_singleline(&mut self.client.primary_therapist);
                    ui.end_row();

                    ui.monospace("Location");
                    ui.text_edit_singleline(&mut self.client.location);
                    ui.end_row();

                    ui.monospace("Date of Admission\n(YYYY-MM-DD)");
                    ui.text_edit_singleline(&mut self.client.date_of_admission);
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
                match self.save_file_to_path(client_directory_path) {
                    Ok(_) => self.error.clear(),
                    Err(e) => self.error = e.to_string(),
                }
            }

            if ui.large_red_button("Return").clicked() {
                display_info.go_to_begin_session();
            }

            ui.strong(self.error.to_string());
        });
    }
}
