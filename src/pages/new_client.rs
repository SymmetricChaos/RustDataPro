use crate::{
    app::{
        ASSESSMENTS_FILE_NAME, CLIENT_DATA_FILE_NAME, DataPro, IOA_DATA_FOLDER_NAME,
        SESSION_DATA_FOLDER_NAME,
    },
    data::{ClientData, KsfData},
    utils::DataProUiElements,
};
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    hash::Hasher,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

pub struct NewClient {
    client: ClientData,
    error: String,
}

impl Default for NewClient {
    fn default() -> Self {
        Self {
            client: ClientData::default(),
            error: String::new(),
        }
    }
}

impl NewClient {
    fn save_file_to_path(&mut self, root_directory: &PathBuf) -> Result<()> {
        let p = Path::new(root_directory).join(self.client.id.to_string());

        // Create a new directory for the client inside the root
        std::fs::create_dir(&p)?;

        // Create the client records folder where OutputData will be stored
        std::fs::create_dir(Path::new(&p.join(SESSION_DATA_FOLDER_NAME)))?;

        // Create the IOA folder
        std::fs::create_dir(Path::new(&p.join(IOA_DATA_FOLDER_NAME)))?;

        // Create the client file inside the new directory, title it client_data.txt
        let mut writer =
            BufWriter::new(File::create_new(Path::new(&p.join(CLIENT_DATA_FILE_NAME)))?);
        writer.write_all(self.client.to_json()?.as_bytes())?;
        writer.flush()?;

        // Create a blank assessments file
        File::create_new(Path::new(&p.join(ASSESSMENTS_FILE_NAME)))?;

        // Create a template KSF file
        let mut writer = File::create_new(Path::new(&p.join("TEMPLATEKSF.txt")))?;
        writer.write_all(KsfData::_test_ksf().to_json()?.as_bytes())?;
        writer.flush()?;

        Ok(())
    }

    pub fn view(app: &mut DataPro, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Create a New Client");
            ui.add_space(10.0);
            egui::Grid::new("client_and_session_info_grid")
                .min_col_width(150.0)
                .spacing((10.0, 10.0))
                .show(ui, |ui| {
                    ui.monospace("Client ID");
                    ui.text_edit_singleline(&mut app.new_client_page.client.id);
                    if ui
                        .button("suggest")
                        .on_hover_text(
                            "derived from Client Name, Case Manager, Primary Therapist, and Date of Admission",
                        )
                        .clicked()
                    {
                        let hash = Sha256::new()
                            .chain_update(&app.new_client_page.client.name)
                            .chain_update(&app.new_client_page.client.case_manager)
                            .chain_update(&app.new_client_page.client.primary_therapist)
                            .chain_update(&app.new_client_page.client.date_of_admission)
                            .finalize();
                        app.new_client_page.client.id = format!(
                            "{:0<10}",
                            u64::from_be_bytes(hash[0..8].try_into().unwrap())%10000000000
                        )
                    }
                    ui.end_row();

                    ui.monospace("Client Name");
                    ui.text_edit_singleline(&mut app.new_client_page.client.name);
                    ui.end_row();

                    ui.monospace("Case Manager");
                    ui.text_edit_singleline(&mut app.new_client_page.client.case_manager);
                    ui.end_row();

                    ui.monospace("Primary Therapist");
                    ui.text_edit_singleline(&mut app.new_client_page.client.primary_therapist);
                    ui.end_row();

                    ui.monospace("Date of Admission\n(YYYY-MM-DD)");
                    ui.text_edit_singleline(&mut app.new_client_page.client.date_of_admission);
                    ui.end_row();

                    ui.monospace("Location");
                    ui.text_edit_singleline(&mut app.new_client_page.client.location);
                    ui.end_row();
                });

            ui.add_enabled_ui(!app.new_client_page.client.id.is_empty(), |ui| {
                if ui
                    .large_green_button("Save")
                    .on_disabled_hover_text("client must have an ID assigned")
                    .clicked()
                {
                    match app.new_client_page.save_file_to_path(&app.root_directory) {
                        Ok(_) => app.new_client_page.error.clear(),
                        Err(e) => app.new_client_page.error = e.to_string(),
                    }
                }
            });

            if ui.large_red_button("Return").clicked() {
                app.display_info.go_to_prep_session();
            }

            ui.strong(app.new_client_page.error.to_string());
        });
    }
}
