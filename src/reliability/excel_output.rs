use crate::{data::IoaData, reliability::RELI_FILE_START, utils::time_stamp};
use anyhow::Result;
use egui::Key;
use rust_xlsxwriter::*;

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

pub fn excel_output(ioa_data: &IoaData) -> Result<()> {
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
