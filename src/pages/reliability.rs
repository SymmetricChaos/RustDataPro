use crate::{
    app::DisplayInfo,
    data::{OutputData, timeline::Timeline},
    utils::DataProUiElements,
};
use anyhow::Result;
use egui::{TextBuffer, Ui};
use egui_file_dialog::FileDialog;
use std::{ffi::OsStr, path::PathBuf};

fn extract_times(v: &Timeline, description: &str) -> Vec<f32> {
    v.iter()
        .filter(|e| e.0 == description)
        .map(|e| e.1)
        .collect()
}

fn interval_reli(
    max_time: f32,
    interval: f32,
    description: &str,
    primary: &Timeline,
    reli: &Timeline,
) -> Result<f32> {
    let mut time = interval;
    let mut ratio: f32 = 1.0;
    let primary = extract_times(primary, description);
    let mut p_iter = primary.into_iter().peekable();
    let reli = extract_times(reli, description);
    let mut r_iter = reli.into_iter().peekable();
    let mut ctr = 1;
    println!("{description}");
    while time <= max_time {
        let mut pctr = 0.0;
        while p_iter.next_if(|x| x <= &time).is_some() {
            pctr += 1.0;
        }
        let mut rctr = 0.0;
        while r_iter.next_if(|x| x <= &time).is_some() {
            rctr += 1.0;
        }
        println!("{ctr}: {pctr} {rctr}");
        // if pctr == 0.0 {
        //     todo!("handle interval where primary count is zero")
        // }
        // if rctr == 0.0 {
        //     todo!("handle interval where reli count is zero")
        // }
        // let interval_ratio = pctr / rctr;
        // ratio *= interval_ratio;
        time += interval;
        ctr += 1;
    }
    Ok(ratio)
}

pub struct ReliabilityPage {
    primary_file_dialog: FileDialog,
    primary_bufs: Vec<PathBuf>,
    primary_data: Vec<OutputData>,
    reli_file_dialog: FileDialog,
    reli_bufs: Vec<PathBuf>,
    reli_data: Vec<OutputData>,
}

impl Default for ReliabilityPage {
    fn default() -> Self {
        Self {
            primary_file_dialog: Default::default(),
            primary_bufs: Vec::new(),
            primary_data: Vec::new(),
            reli_file_dialog: Default::default(),
            reli_bufs: Vec::new(),
            reli_data: Vec::new(),
        }
    }
}

impl ReliabilityPage {
    fn calculate_reli(&self) {
        for (p, r) in self.primary_data.iter().zip(self.reli_data.iter()) {
            for (_, description) in p.ksf.frequency.iter() {
                let _ = interval_reli(
                    p.session_duration,
                    10.0,
                    description,
                    &p.timeline,
                    &r.timeline,
                );
            }
        }
    }

    pub fn view(&mut self, ui: &mut Ui, display_info: &mut DisplayInfo) {
        self.primary_file_dialog.update(ui.ctx());
        self.reli_file_dialog.update(ui.ctx());
        if let Some(bufs) = self.primary_file_dialog.take_picked_multiple() {
            self.primary_bufs = bufs;
            for buf in self.primary_bufs.iter() {
                self.primary_data.push(
                    OutputData::from_file(buf.as_path())
                        .expect("failed to load primary output data"),
                );
            }
        }
        if let Some(bufs) = self.reli_file_dialog.take_picked_multiple() {
            self.reli_bufs = bufs;
            for buf in self.primary_bufs.iter() {
                self.reli_data.push(
                    OutputData::from_file(buf.as_path()).expect("failed to load reli output data"),
                );
            }
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
