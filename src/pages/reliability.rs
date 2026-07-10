use crate::{
    app::DisplayInfo,
    data::{IoaData, OutputData, timeline::Timeline},
    utils::{DataProUiElements, time_stamp},
};
use anyhow::{Context, Result};
use egui::{TextBuffer, Ui};
use egui_file_dialog::FileDialog;
use rust_xlsxwriter::*;
use std::{
    borrow::Cow,
    ffi::OsStr,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

fn write_excel_line<'a>(
    worksheet: &'a mut Worksheet,
    row: u32,
    name: &'static str,
    it: impl Iterator<Item = &'a (String, f32)>,
) -> Result<()> {
    worksheet.write(row, 0, name)?;
    let mut col = 1;
    for (_, n) in it {
        worksheet.write(row, col, &format!("{:.1}", n * 100.0))?;
        col += 1;
    }
    Ok(())
}

fn excel_output(ioa_data: &IoaData) -> Result<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 22)?;
    worksheet.set_column_range_width(1, 20, 10)?;

    let mut col = 1;
    for (d, _) in ioa_data.sixty_sec_interval.iter() {
        worksheet.write(0, col, d)?;
        col += 1;
    }
    write_excel_line(
        worksheet,
        1,
        "60 Second Interval",
        ioa_data.sixty_sec_interval.iter(),
    )?;
    write_excel_line(
        worksheet,
        2,
        "10 Second Interval",
        ioa_data.ten_sec_interval.iter(),
    )?;
    write_excel_line(worksheet, 3, "Total Count", ioa_data.total_count.iter())?;
    write_excel_line(
        worksheet,
        4,
        "Total Duration",
        ioa_data.total_duration.iter(),
    )?;

    workbook.save(&format!("reli_data_{}.xlsx", time_stamp()))?;
    Ok(())
}

fn extract_times(v: &Timeline, description: &str) -> Vec<f32> {
    v.iter()
        .filter(|e| e.0 == description)
        .map(|e| e.1)
        .collect()
}

/// Used for Total Count and Total Duration IOA. Divides the smaller value by the larger value. If both values are zero returns None to be handled specially.
fn total_ratio_ioa(primary: f32, reli: f32) -> Option<f32> {
    if primary == 0.0 && reli == 0.0 {
        return None;
    }
    if primary >= reli {
        Some(reli / primary)
    } else {
        Some(primary / reli)
    }
}

/// Caclulate the fraction of intervals in which both primary and reliability data have the same count. If no intervals exist returns None.
/// If strict is true then any intervals in which neither data set records anything are ignored from the total.
fn interval_ioa(
    max_time: f32,
    interval: f32,
    description: &str,
    primary: &Timeline,
    reli: &Timeline,
    strict: bool,
) -> Option<f32> {
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
        None
    } else {
        Some(correct_intervals / total_intervals)
    }
}

fn quick_file_name(pathbuf: &PathBuf) -> Cow<'_, str> {
    pathbuf
        .file_name()
        .unwrap_or(&OsStr::new("invalid"))
        .to_string_lossy()
}

pub struct ReliabilityPage {
    primary_file_dialog: FileDialog,
    primary_bufs: Vec<PathBuf>,
    pdata: Vec<OutputData>,
    reli_file_dialog: FileDialog,
    reli_bufs: Vec<PathBuf>,
    rdata: Vec<OutputData>,
    error: String,
    strict: bool,
    none_val: f32,
}

impl Default for ReliabilityPage {
    fn default() -> Self {
        Self {
            primary_file_dialog: Default::default(),
            primary_bufs: Vec::new(),
            pdata: Vec::new(),
            reli_file_dialog: Default::default(),
            reli_bufs: Vec::new(),
            rdata: Vec::new(),
            error: String::new(),
            strict: true,
            none_val: f32::NAN,
        }
    }
}

impl ReliabilityPage {
    fn validate_files(&self) -> Result<()> {
        if self.pdata.is_empty() {
            return Err(anyhow::anyhow!("no primary data files"));
        }
        if self.rdata.is_empty() {
            return Err(anyhow::anyhow!("no reliability data files"));
        }

        let ksf = &self.pdata[0].ksf;
        let id = &self.pdata[0].client.client_id;
        for (i, p) in self.pdata.iter().enumerate() {
            if &p.ksf != ksf {
                return Err(anyhow::anyhow!(
                    "primary file {} has an incorrect ksf",
                    quick_file_name(&self.primary_bufs[i])
                ));
            }
            if &p.client.client_id != id {
                return Err(anyhow::anyhow!(
                    "primary file {} has an incorrect client id",
                    quick_file_name(&self.primary_bufs[i])
                ));
            }
        }
        for (i, r) in self.rdata.iter().enumerate() {
            if &r.ksf != ksf {
                return Err(anyhow::anyhow!(
                    "reli file {} has an incorrect ksf",
                    quick_file_name(&self.reli_bufs[i])
                ));
            }
            if &r.client.client_id != id {
                return Err(anyhow::anyhow!(
                    "reli file {} has an incorrect client id",
                    quick_file_name(&self.reli_bufs[i])
                ));
            }
        }
        Ok(())
    }

    fn frequency_ioa(&self, ioa_data: &mut IoaData) -> Result<()> {
        for (p, r) in self.pdata.iter().zip(self.rdata.iter()) {
            let max_time = if p.session_duration >= r.session_duration {
                p.session_duration
            } else {
                r.session_duration
            };
            for (_, description) in p.ksf.frequency.iter() {
                // 10 Second Interval-by-Interval IOA
                let r10 = interval_ioa(
                    max_time,
                    10.0,
                    description,
                    &p.timeline,
                    &r.timeline,
                    self.strict,
                )
                .unwrap_or(self.none_val);
                ioa_data.ten_sec_interval.push((description.clone(), r10));

                // 60 Second Interval-by-Interval IOA
                let r60 = interval_ioa(
                    max_time,
                    60.0,
                    description,
                    &p.timeline,
                    &r.timeline,
                    self.strict,
                )
                .unwrap_or(self.none_val);
                ioa_data.sixty_sec_interval.push((description.clone(), r60));

                // Total duration is meaningless for frequency data but for alignment in potential output a NaN is pushed
                ioa_data
                    .total_duration
                    .push((description.clone(), f32::NAN));

                // Total Count IOA
                let primary_dur = *p
                    .frequency
                    .get(description)
                    .context("missing primary duration")? as f32; // conversion of u32 to f32 is valid so long as count is below about 16 million, so it is not checked
                let reli_dur = *r
                    .frequency
                    .get(description)
                    .context("missing reli duration")? as f32;
                ioa_data.total_count.push((
                    description.clone(),
                    total_ratio_ioa(primary_dur, reli_dur).unwrap_or(self.none_val),
                ));
            }
        }
        Ok(())
    }

    fn duration_ioa(&self, ioa_data: &mut IoaData) -> Result<()> {
        for (p, r) in self.pdata.iter().zip(self.rdata.iter()) {
            let max_time = if p.session_duration >= r.session_duration {
                p.session_duration
            } else {
                r.session_duration
            };
            for (_, description) in p.ksf.duration.iter() {
                // 10 Second Interval-by-Interval IOA
                let r10 = interval_ioa(
                    max_time,
                    10.0,
                    description,
                    &p.timeline,
                    &r.timeline,
                    self.strict,
                )
                .unwrap_or(self.none_val);
                ioa_data.ten_sec_interval.push((description.clone(), r10));

                // 60 Second Interval-by-Interval IOA
                let r60 = interval_ioa(
                    max_time,
                    60.0,
                    description,
                    &p.timeline,
                    &r.timeline,
                    self.strict,
                )
                .unwrap_or(self.none_val);
                ioa_data.sixty_sec_interval.push((description.clone(), r60));

                // Total Count IOA (onset and offset of duration keys)
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
                ioa_data.total_duration.push((
                    description.clone(),
                    total_ratio_ioa(primary_dur, reli_dur).unwrap_or(self.none_val),
                ));

                // Total Duration IOA
                let primary_dur = p
                    .duration
                    .get(description)
                    .context("missing primary duration")?
                    .0 as f32;
                let reli_dur = r
                    .duration
                    .get(description)
                    .context("missing reli duration")?
                    .0 as f32;
                ioa_data.total_count.push((
                    description.clone(),
                    total_ratio_ioa(primary_dur, reli_dur).unwrap_or(self.none_val),
                ));
            }
        }
        Ok(())
    }

    fn calculate_ioa(&self) -> Result<()> {
        let mut ioa_data = IoaData::new();
        self.frequency_ioa(&mut ioa_data)?;
        self.duration_ioa(&mut ioa_data)?;
        excel_output(&ioa_data)?;

        let mut writer = BufWriter::new(File::create(&format!("reli_data_{}.txt", time_stamp()))?);
        writer.write_all(ioa_data.to_json()?.as_bytes())?;
        writer.flush()?;
        Ok(())
    }

    pub fn view(&mut self, ui: &mut Ui, display_info: &mut DisplayInfo) {
        self.primary_file_dialog.update(ui.ctx());
        if let Some(bufs) = self.primary_file_dialog.take_picked_multiple() {
            self.primary_bufs = bufs;
            for buf in self.primary_bufs.iter() {
                match OutputData::from_file(buf.as_path()) {
                    Ok(data) => self.pdata.push(data),
                    Err(e) => self.error = e.to_string(),
                }
            }
        }

        self.reli_file_dialog.update(ui.ctx());
        if let Some(bufs) = self.reli_file_dialog.take_picked_multiple() {
            self.reli_bufs = bufs;
            for buf in self.reli_bufs.iter() {
                match OutputData::from_file(buf.as_path()) {
                    Ok(data) => self.rdata.push(data),
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

            if ui.large_green_button("Calculate IOA").clicked() {
                match self.validate_files() {
                    Ok(_) => match self.calculate_ioa() {
                        Ok(_) => self.error.clear(),
                        Err(e) => self.error = e.to_string(),
                    },
                    Err(e) => self.error = e.to_string(),
                }
            }
            if ui.large_red_button("Return").clicked() {
                display_info.go_to_about();
                self.primary_bufs.clear();
                self.pdata.clear();
                self.reli_bufs.clear();
                self.rdata.clear();
            }

            ui.strong(&self.error)
        });
    }
}
