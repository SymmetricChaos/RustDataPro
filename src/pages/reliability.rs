use crate::{
    app::DisplayInfo,
    data::{DataType, IoaData, OutputData, timeline::Timeline},
    utils::{DataProUiElements, quick_file_name, time_stamp},
};
use anyhow::{Context, Result};
use egui::{Color32, Key, RichText, Ui};
use egui_file_dialog::FileDialog;
use rust_xlsxwriter::*;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

const RELI_FILE_START: &'static str = "reli_data_";

fn write_excel_line<'a>(
    worksheet: &'a mut Worksheet,
    row: u32,
    name: &'static str,
    it: impl Iterator<Item = &'a (Key, f32)>,
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
    for (k, _) in ioa_data.sixty_sec_interval.iter() {
        worksheet.write(0, col, k.name())?;
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

    workbook.save(&format!("{}{}.xlsx", RELI_FILE_START, time_stamp()))?;
    Ok(())
}

fn extract_times(v: &Timeline, key: Key) -> Vec<f32> {
    v.iter().filter(|e| e.0 == key).map(|e| e.1).collect()
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
    key: Key,
    primary: &Timeline,
    reli: &Timeline,
    strict: bool,
) -> Option<f32> {
    let mut time = interval;

    let primary = extract_times(primary, key);
    let mut p_iter = primary.into_iter().peekable();
    let reli = extract_times(reli, key);
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

pub struct ReliabilityPage {
    file_dialog: FileDialog,
    bufs: Vec<PathBuf>,
    prim_data: Vec<OutputData>,
    reli_data: Vec<OutputData>,
    error: String,
    strict: bool,
    none_val: f32,
    ioa_finished: bool,
}

impl Default for ReliabilityPage {
    fn default() -> Self {
        Self {
            file_dialog: Default::default(),
            bufs: Vec::new(),
            prim_data: Vec::new(),
            reli_data: Vec::new(),
            error: String::new(),
            strict: true,
            none_val: f32::NAN,
            ioa_finished: false,
        }
    }
}

impl ReliabilityPage {
    fn validate_files(&mut self) -> Result<()> {
        // Are there any files?
        match (self.prim_data.is_empty(), self.reli_data.is_empty()) {
            (true, true) => return Err(anyhow::anyhow!("no data files selected")),
            (true, false) => return Err(anyhow::anyhow!("no primary data files")),
            (false, true) => return Err(anyhow::anyhow!("no reliability data files")),
            _ => (),
        }

        // Is there a reliability file to go with each primary file?
        if self.prim_data.len() != self.reli_data.len() {
            return Err(anyhow::anyhow!(
                "unequal number of primary and reliability files"
            ));
        }

        // Assume the first primary file is correct and check that all files use the save KSF and have the same client ID
        let ksf = &self.prim_data[0].ksf;
        let id = &self.prim_data[0].client.client_id;
        for (i, o) in self
            .prim_data
            .iter()
            .chain(self.reli_data.iter())
            .enumerate()
            .skip(1)
        {
            if &o.ksf != ksf {
                return Err(anyhow::anyhow!(
                    "file {} has an incorrect ksf",
                    quick_file_name(&self.bufs[i])
                ));
            }
            if &o.client.client_id != id {
                return Err(anyhow::anyhow!(
                    "file {} has an incorrect client id",
                    quick_file_name(&self.bufs[i])
                ));
            }
        }

        Ok(())
    }

    fn interval_ioa(&self, ioa_data: &mut IoaData) {
        for (p, r) in self.prim_data.iter().zip(self.reli_data.iter()) {
            let max_time = if p.session_duration >= r.session_duration {
                p.session_duration
            } else {
                r.session_duration
            };
            for (key, _) in p.ksf.pairs() {
                // 10 Second Interval-by-Interval IOA
                let r10 = interval_ioa(max_time, 10.0, *key, &p.timeline, &r.timeline, self.strict)
                    .unwrap_or(self.none_val);
                ioa_data.ten_sec_interval.push((*key, r10));

                // 60 Second Interval-by-Interval IOA
                let r60 = interval_ioa(max_time, 60.0, *key, &p.timeline, &r.timeline, self.strict)
                    .unwrap_or(self.none_val);
                ioa_data.sixty_sec_interval.push((*key, r60));
            }
        }
    }

    fn frequency_ioa(&self, ioa_data: &mut IoaData) -> Result<()> {
        for (p, r) in self.prim_data.iter().zip(self.reli_data.iter()) {
            for (key, desc) in p.ksf.frequency.iter() {
                // Total Duration IOA is meaningless for frequency data but for alignment in potential output a NaN is pushed
                ioa_data.total_duration.push((*key, f32::NAN));

                // Total Count IOA
                let primary_dur =
                    *p.frequency.get(desc).context("missing primary duration")? as f32; // conversion of u32 to f32 is valid so long as count is below about 16 million, so it is not checked
                let reli_dur = *r.frequency.get(desc).context("missing reli duration")? as f32;
                ioa_data.total_count.push((
                    *key,
                    total_ratio_ioa(primary_dur, reli_dur).unwrap_or(self.none_val),
                ));
            }
        }
        Ok(())
    }

    fn duration_ioa(&self, ioa_data: &mut IoaData) -> Result<()> {
        for (p, r) in self.prim_data.iter().zip(self.reli_data.iter()) {
            for (key, description) in p.ksf.duration.iter() {
                // Total Duration IOA
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
                    *key,
                    total_ratio_ioa(primary_dur, reli_dur).unwrap_or(self.none_val),
                ));

                // Total Count IOA (onset and offset of duration keys)
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
                    *key,
                    total_ratio_ioa(primary_dur, reli_dur).unwrap_or(self.none_val),
                ));
            }
        }
        Ok(())
    }

    fn calculate_ioa(&mut self) -> Result<()> {
        let mut ioa_data = IoaData::new();
        self.interval_ioa(&mut ioa_data);
        self.frequency_ioa(&mut ioa_data)?;
        self.duration_ioa(&mut ioa_data)?;
        excel_output(&ioa_data)?;

        let mut writer = BufWriter::new(File::create(&format!(
            "{}{}.txt",
            RELI_FILE_START,
            time_stamp()
        ))?);
        writer.write_all(ioa_data.to_json()?.as_bytes())?;
        writer.flush()?;

        Ok(())
    }

    pub fn view(&mut self, ui: &mut Ui, display_info: &mut DisplayInfo) {
        self.file_dialog.update(ui.ctx());
        if let Some(bufs) = self.file_dialog.take_picked_multiple() {
            self.bufs = bufs;
            self.ioa_finished = false;
            self.error.clear();
            // Simultaneously parse and filter the input files.
            self.bufs
                .retain(|buf| match OutputData::from_file(buf.as_path()) {
                    Ok(data) => {
                        match data.session.data_type {
                            DataType::Primary => self.prim_data.push(data),
                            DataType::Reliability => self.reli_data.push(data),
                        }
                        true
                    }
                    Err(_) => false,
                });
        }

        egui::CentralPanel::default().show(ui, |ui| {
            if ui.large_button("Select Data").clicked() {
                self.file_dialog.pick_multiple();
            }

            egui::containers::ScrollArea::vertical()
                .id_salt("file_name_area")
                .show(ui, |ui| {
                    for buf in self.bufs.iter() {
                        ui.strong(quick_file_name(buf));
                    }
                });

            ui.add_space(20.0);

            if ui.large_green_button("Calculate IOA").clicked() {
                match self.validate_files() {
                    Ok(_) => match self.calculate_ioa() {
                        Ok(_) => {
                            self.error.clear();
                            self.ioa_finished = true;
                        }
                        Err(e) => self.error = e.to_string(),
                    },
                    Err(e) => self.error = e.to_string(),
                }
            }
            if ui.large_red_button("Return").clicked() {
                self.bufs.clear();
                self.prim_data.clear();
                self.reli_data.clear();
                self.error.clear();
                display_info.go_to_about();
            }

            ui.strong(&self.error);

            if self.ioa_finished {
                ui.monospace(
                    RichText::new("IOA Calculated and Saved!")
                        .heading()
                        .color(Color32::GREEN),
                );
            }
        });
    }
}
