use std::path::PathBuf;

use crate::{
    data::{IoaData, OutputData},
    utils::quick_file_name,
};
use anyhow::Result;
use egui::Key;
use rust_xlsxwriter::*;

fn write_excel_line<'a>(
    worksheet: &'a mut Worksheet,
    row: u32,
    name: &'static str,
    it: impl Iterator<Item = (&'a Key, &'a f32)>,
) -> Result<()> {
    worksheet.write(row, 0, name)?;
    let mut col = 1;
    for (_, n) in it {
        worksheet.write(row, col, &format!("{:.1}", n * 100.0))?;
        col += 1;
    }
    Ok(())
}

pub fn save_excel_workbook(
    ioa_data: &IoaData,
    file_stem: &str,
    prim_data: &Vec<(OutputData, PathBuf)>,
    reli_data: &Vec<(OutputData, PathBuf)>,
) -> Result<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_column_width(0, 22)?;
    worksheet.set_column_range_width(1, 20, 10)?;

    let map = prim_data[0].0.ksf.create_map();
    let mut col = 1;
    for (k, _) in ioa_data.sixty_sec_interval.iter() {
        worksheet.write(0, col, map.get(k).unwrap_or(&String::from("UNKNOWN KEY")))?;
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

    let worksheet2 = workbook.add_worksheet();
    worksheet2.set_column_width(0, 16)?;
    worksheet2.set_column_width(2, 22)?;
    worksheet2.set_column_width(3, 22)?;
    worksheet2.write(0, 0, "Data Taken from")?;
    let mut ctr = 0;
    for (p, r) in prim_data.iter().zip(reli_data.iter()) {
        worksheet2.write(ctr, 2, quick_file_name(&p.1))?;
        worksheet2.write(ctr, 3, quick_file_name(&r.1))?;
        ctr += 1;
    }

    workbook.save(&format!("{}.xlsx", file_stem))?;
    Ok(())
}
