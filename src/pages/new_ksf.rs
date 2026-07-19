use crate::{
    app::DataPro,
    data::{Data, KsfData},
    utils::{DataProUiElements, windows_error_dialog},
};
use anyhow::{Context, Result};
use egui::Key;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

fn parse_line(s: &str) -> Result<(Key, String)> {
    let (k, d) = s.split_once(",").context("no comma in key specification")?;
    let key = match Key::from_name(k.trim()) {
        Some(key) => key,
        None => {
            return Err(anyhow::anyhow!("{} is not a valid key name", k));
        }
    };
    let desc = match d.contains(",") {
        true => {
            return Err(anyhow::anyhow!(
                "descriptions cannot contain a comma, start new specifications on new lines"
            ));
        }
        false => d.trim().to_string(),
    };
    Ok((key, desc))
}

fn entry_row(
    ui: &mut egui::Ui,
    string: &mut String,
    vector: &mut Vec<(Key, String)>,
    error: &mut String,
) {
    if ui.text_edit_multiline(string).changed() {
        let mut found_err = None;
        vector.clear();
        for line in string.split("\n") {
            if !line.is_empty() {
                match parse_line(line) {
                    Ok(entry) => vector.push(entry),
                    Err(e) => {
                        if found_err.is_none() {
                            found_err = Some(e.to_string())
                        }
                    }
                }
            }
        }
        match found_err {
            Some(e) => *error = e,
            None => error.clear(),
        };
    }

    ui.monospace(format!(
        "Preview:\n{}",
        vector
            .iter()
            .map(|(k, d)| format!("({},{})", k.symbol_or_name(), d))
            .join("  ")
    ));
}

#[derive(Default)]
pub struct NewKsf {
    ksf: KsfData,
    file_name: String,
    freq_string: String,
    dura_string: String,
    freq_error: String,
    dura_error: String,
    save_error: String,
}

impl NewKsf {
    fn save_file_to_path(&mut self, data: &Data, root_directory: &PathBuf) -> Result<()> {
        if !self.ksf.is_valid() {
            return Err(anyhow::anyhow!(
                "ksf contains duplicate keys or duplicate descriptions"
            ));
        }
        let p = Path::new(root_directory)
            .join(&format!("{}", data.client.id))
            .join(&format!("{}.xtx", self.file_name));
        let mut writer = BufWriter::new(File::create_new(p)?);
        writer.write_all(self.ksf.to_json()?.as_bytes())?;
        writer.flush()?;

        Ok(())
    }

    pub fn view(app: &mut DataPro, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.heading("New Keyboard Setup File for Client ");
                ui.add(egui::Label::new(
                    egui::RichText::new(&app.data.client.id).heading().strong(),
                ));
            });
            ui.add_space(10.0);

            ui.label("Name");
            ui.text_edit_singleline(&mut app.new_ksf_page.file_name);
            ui.add_space(10.0);

            egui::Grid::new("new_ksf_info_grid")
                .min_col_width(150.0)
                .show(ui, |ui| {
                    ui.monospace("Frequency Keys");
                    entry_row(
                        ui,
                        &mut app.new_ksf_page.freq_string,
                        &mut app.new_ksf_page.ksf.frequency,
                        &mut app.new_ksf_page.freq_error,
                    );
                    ui.end_row();

                    ui.monospace("Duration Keys");
                    entry_row(
                        ui,
                        &mut app.new_ksf_page.dura_string,
                        &mut app.new_ksf_page.ksf.duration,
                        &mut app.new_ksf_page.dura_error,
                    );
                    ui.end_row();
                });

            ui.add_enabled_ui(
                app.new_ksf_page.dura_error.is_empty()
                    && app.new_ksf_page.freq_error.is_empty()
                    && !app.new_ksf_page.file_name.is_empty(),
                |ui| {
                    if ui
                        .large_green_button("Save")
                        .on_disabled_hover_text(
                            "cannot save until a name is chosen and there are no errors",
                        )
                        .clicked()
                    {
                        match app
                            .new_ksf_page
                            .save_file_to_path(&app.data, &app.root_directory)
                        {
                            Ok(_) => app.new_ksf_page.save_error.clear(),
                            Err(e) => windows_error_dialog(e),
                        }
                    }
                },
            );

            if ui.large_red_button("Return").clicked() {
                app.display_info.go_to_prep_session();
            }

            ui.strong(app.new_ksf_page.freq_error.to_string());
            ui.strong(app.new_ksf_page.dura_error.to_string());
        });
    }
}
