use crate::{
    app::DisplayInfo,
    data::{OutputData, ReliData, timeline::Timeline},
    utils::{DataProUiElements, time_stamp},
};
use anyhow::{Context, Result};
use egui::{TextBuffer, Ui};
use egui_file_dialog::FileDialog;
use std::{
    ffi::OsStr,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

fn extract_times(v: &Timeline, description: &str) -> Vec<f32> {
    v.iter()
        .filter(|e| e.0 == description)
        .map(|e| e.1)
        .collect()
}

fn duration_reli(primary: f32, reli: f32) -> f32 {
    if primary >= reli {
        if primary == 0.0 {
            return 0.0;
        } else {
            reli / primary
        }
    } else {
        if reli == 0.0 {
            return 0.0;
        } else {
            primary / reli
        }
    }
}

fn interval_reli(
    max_time: f32,
    interval: f32,
    description: &str,
    primary: &Timeline,
    reli: &Timeline,
    strict: bool,
) -> f32 {
    let mut time = interval;

    let primary = extract_times(primary, description);
    let mut p_iter = primary.into_iter().peekable();
    let reli = extract_times(reli, description);
    let mut r_iter = reli.into_iter().peekable();

    let mut correct_intervals = 0.0;
    let mut total_intervals = 0.0;
    while time <= max_time {
        let mut pctr = 0.0;
        while p_iter.next_if(|x| x <= &time).is_some() {
            pctr += 1.0;
        }
        let mut rctr = 0.0;
        while r_iter.next_if(|x| x <= &time).is_some() {
            rctr += 1.0;
        }
        if strict && pctr == 0.0 && rctr == 0.0 {
            // In strict mode ignore intervals when primary and reli both scored nothing
        } else {
            if pctr == rctr {
                correct_intervals += 1.0;
            }
            total_intervals += 1.0;
        }

        time += interval;
    }

    if total_intervals == 0.0 {
        0.0
    } else {
        correct_intervals / total_intervals
    }
}

pub struct ReliabilityPage {
    primary_file_dialog: FileDialog,
    primary_bufs: Vec<PathBuf>,
    primary_data: Vec<OutputData>,
    reli_file_dialog: FileDialog,
    reli_bufs: Vec<PathBuf>,
    reli_data: Vec<OutputData>,
    error: String,
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
            error: String::new(),
        }
    }
}

impl ReliabilityPage {
    fn calculate_reli(&self) -> Result<()> {
        let mut reliability_output = ReliData::new();

        for (p, r) in self.primary_data.iter().zip(self.reli_data.iter()) {
            let max_time = if p.session_duration >= r.session_duration {
                p.session_duration
            } else {
                r.session_duration
            };
            for (_, description) in p.ksf.frequency.iter() {
                // Calculate 10 second reli
                let r10 =
                    interval_reli(max_time, 10.0, description, &p.timeline, &r.timeline, false);
                reliability_output
                    .ten_sec_interval
                    .push((description.clone(), r10));

                // Calculate 60 second reli
                let r60 =
                    interval_reli(max_time, 60.0, description, &p.timeline, &r.timeline, false);
                reliability_output
                    .sixty_sec_interval
                    .push((description.clone(), r60));
            }
            // compare duration ratios
            for (_, description) in p.ksf.duration.iter() {
                let primary_dur = p
                    .duration
                    .get(description)
                    .context("missing primary duration")?
                    .1;
                let reli_dur = r
                    .duration
                    .get(description)
                    .context("missing reli duration")?
                    .1;
                reliability_output
                    .duration_ratio
                    .push((description.clone(), duration_reli(primary_dur, reli_dur)));
            }
        }

        let mut writer = BufWriter::new(File::create(&format!("reli_data_{}.txt", time_stamp(),))?);
        writer.write_all(reliability_output.to_json()?.as_bytes())?;
        writer.flush()?;
        Ok(())
    }

    pub fn view(&mut self, ui: &mut Ui, display_info: &mut DisplayInfo) {
        self.primary_file_dialog.update(ui.ctx());
        if let Some(bufs) = self.primary_file_dialog.take_picked_multiple() {
            self.primary_bufs = bufs;
            for buf in self.primary_bufs.iter() {
                match OutputData::from_file(buf.as_path()) {
                    Ok(data) => self.primary_data.push(data),
                    Err(e) => self.error = e.to_string(),
                }
            }
        }

        self.reli_file_dialog.update(ui.ctx());
        if let Some(bufs) = self.reli_file_dialog.take_picked_multiple() {
            self.reli_bufs = bufs;
            for buf in self.reli_bufs.iter() {
                match OutputData::from_file(buf.as_path()) {
                    Ok(data) => self.reli_data.push(data),
                    Err(e) => self.error = e.to_string(),
                }
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
                match self.calculate_reli() {
                    Ok(_) => self.error.clear(),
                    Err(e) => self.error = e.to_string(),
                }
            }
            if ui.large_red_button("Return").clicked() {
                display_info.go_to_about();
                self.primary_bufs.clear();
                self.primary_data.clear();
                self.reli_bufs.clear();
                self.reli_data.clear();
            }

            ui.strong(&self.error)
        });
    }
}
