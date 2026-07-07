use crate::{app::DisplayInfo, utils::DataProUiElements};
use egui::{TextBuffer, Ui, Key};
use egui_file_dialog::FileDialog;
use std::{ffi::OsStr, path::PathBuf};
use anyhow::Result;

fn extract_times(v: Vec<(Key,f32), key: Key) -> Vec<f32> {
    v.iter().filter(|e| e.0 == key).map(|e| e.1).collect()
}

fn interval_reli(max_time: f32, interval: f32, primary: Vec<f32>, reli: Vec<f32>) -> Result<f32> {
    let mut time = interval;
    let mut ratio: f32 = 1.0;
    let mut primary = primary.clone();
    primary.reverse();
    let mut reli = reli.clone();
    reli.reverse();
    while time <= max_time {
        let mut pctr = 0.0;
        while let Some(time) = primary.last() {
            primary.pop();
            pctr += 1.0;
        }
        let mut rctr = 0.0;
        while let Some(time) = reli.last() {
            reli.pop();
            rctr += 1.0;
        }
        let interval_ratio = pctr/rctr;
        time += interval;
    }
    Ok(ratio)
}

pub struct ReliabilityPage {
    primary_file_dialog: FileDialog,
    primary_bufs: Vec<PathBuf>,
    reli_file_dialog: FileDialog,
    reli_bufs: Vec<PathBuf>,
}

impl Default for ReliabilityPage {
    fn default() -> Self {
        Self {
            primary_file_dialog: Default::default(),
            primary_bufs: Vec::new(),
            reli_file_dialog: Default::default(),
            reli_bufs: Vec::new(),
        }
    }
}

impl ReliabilityPage {
    fn calculate_reli(&self) {}

    pub fn view(&mut self, ui: &mut Ui, display_info: &mut DisplayInfo) {
        self.primary_file_dialog.update(ui.ctx());
        self.reli_file_dialog.update(ui.ctx());
        if let Some(bufs) = self.primary_file_dialog.take_picked_multiple() {
            self.primary_bufs = bufs;
        }
        if let Some(bufs) = self.reli_file_dialog.take_picked_multiple() {
            self.reli_bufs = bufs;
        }
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    if ui.large_button("Select Primary").clicked() {
                        self.primary_file_dialog.pick_multiple();
                    }
                    for buf in self.primary_bufs.iter() {
                        ui.strong(
                            buf.file_name()
                                .unwrap_or(&OsStr::new("invalid"))
                                .to_string_lossy()
                                .as_str(),
                        );
                    }
                });
                ui.vertical(|ui| {
                    if ui.large_button("Select Reliability").clicked() {
                        self.reli_file_dialog.pick_multiple();
                    }
                    for buf in self.reli_bufs.iter() {
                        ui.strong(
                            buf.file_name()
                                .unwrap_or(&OsStr::new("invalid"))
                                .to_string_lossy()
                                .as_str(),
                        );
                    }
                });
            });
            ui.add_space(20.0);

            if ui.large_green_button("Calculate Reli").clicked() {
                self.calculate_reli();
            }
            if ui.large_red_button("Return").clicked() {
                display_info.go_to_about();
                self.primary_bufs.clear();
                self.reli_bufs.clear();
            }
        });
    }
}
