use crate::{app::DisplayInfo, data::KsfData, utils::DataProUiElements};
use anyhow::{Context, Result};
use egui::Key;
use egui_file_dialog::FileDialog;
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufWriter, Write},
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

pub struct NewKsf {
    ksf: KsfData,
    freq_string: String,
    dura_string: String,
    file_dialog: FileDialog,
    freq_error: String,
    dura_error: String,
    save_error: String,
}

impl Default for NewKsf {
    fn default() -> Self {
        Self {
            ksf: KsfData::blank(),
            freq_string: String::new(),
            dura_string: String::new(),
            file_dialog: FileDialog::new().default_file_name("NewKsf.txt"),
            freq_error: String::new(),
            dura_error: String::new(),
            save_error: String::new(),
        }
    }
}

impl NewKsf {
    fn save_file_to_path(&mut self) -> Result<()> {
        if let Some(path) = self.file_dialog.take_picked() {
            let mut writer = BufWriter::new(File::create_new(path)?);
            if !self.ksf.is_valid() {
                return Err(anyhow::anyhow!(
                    "ksf contains duplicate keys or duplicate descriptions"
                ));
            }
            writer.write_all(self.ksf.to_json()?.as_bytes())?;
            writer.flush()?;
        }
        Ok(())
    }

    pub fn view(&mut self, ui: &mut egui::Ui, display_info: &mut DisplayInfo) {
        self.file_dialog.update(ui.ctx());
        match self.save_file_to_path() {
            Ok(_) => self.save_error.clear(),
            Err(e) => self.save_error = e.to_string(),
        }

        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Create New Key Specification File");
            ui.label("Key specifications should be written as:\n\tKeyName1, Description1\n\tKeyName2, Description2");
            ui.add_space(5.0);

            egui::Grid::new("new_ksf_info_grid")
                .min_col_width(150.0)
                .show(ui, |ui| {
                    ui.monospace("Frequency Keys");
                    entry_row(
                        ui,
                        &mut self.freq_string,
                        &mut self.ksf.frequency,
                        &mut self.freq_error,
                    );
                    ui.end_row();

                    ui.monospace("Duration Keys");
                    entry_row(
                        ui,
                        &mut self.dura_string,
                        &mut self.ksf.duration,
                        &mut self.dura_error,
                    );
                    ui.end_row();
                });

            ui.add_enabled_ui(
                self.dura_error.is_empty() && self.freq_error.is_empty(),
                |ui| {
                    if ui.large_green_button("Save").clicked() {
                        self.file_dialog.save_file();
                    }
                },
            );

            if ui.large_red_button("Return").clicked() {
                display_info.go_to_about();
            }

            ui.strong(self.freq_error.to_string());
            ui.strong(self.dura_error.to_string());
            ui.strong(self.save_error.to_string());
        });
    }
}
