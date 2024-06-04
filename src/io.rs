use std::{fs, path::Path};

use calamine::{open_workbook, Data, DataType, Range, Reader, Xlsx};
use chrono::NaiveDateTime;
use polars::prelude::*;
use polars_excel_writer::PolarsXlsxWriter;

const ROW_OFFSET: u32 = 3;
const COLUMN_OFFSET: u32 = 3;
const DATE_TIME_OUTPUT_FORMAT: &str = "%Y-%m-%dT%H-%M-%S";
const DATE_TIME_INPUT_FORMAT: &str = "%d.%m.%Y %H:%M";

pub fn write_parquet(mut df: DataFrame, file: &Path) {
    let mut file = fs::File::create(file).expect("Failed to create file");

    ParquetWriter::new(&mut file)
        .finish(&mut df)
        .expect("Failed to write to aggragation file");
}

pub fn read_parquet(path: &Path) -> DataFrame {
    let mut file = std::fs::File::open(path).expect("Failed to open file");
    ParquetReader::new(&mut file)
        .finish()
        .expect("Failed to read parquet file")
}

pub fn write_excel(file: &Path, df: &DataFrame) {
    let mut xlsx_writer = PolarsXlsxWriter::new();
    xlsx_writer.set_float_precision(2);
    xlsx_writer.set_autofit(true);
    xlsx_writer.write_dataframe(df).unwrap();
    xlsx_writer.save(&file).unwrap();
}

pub fn read_excel(file: &Path, off_duty_keyword: &str) -> DataFrame {
    let mut excel: Xlsx<_> = open_workbook(file).unwrap();
    let sheet = excel
        .worksheet_range_at(0)
        .expect("Failed to find workspace")
        .unwrap();

    let date = sheet
        .get_value((0, 1))
        .expect("Failed to get date")
        .to_string();

    let date = NaiveDateTime::parse_from_str(&date, DATE_TIME_INPUT_FORMAT)
        .expect("Failed to parse date and time")
        .format(DATE_TIME_OUTPUT_FORMAT)
        .to_string();

    let sheet = sheet.range(
        (ROW_OFFSET, 0),
        (sheet.height() as u32 - ROW_OFFSET, COLUMN_OFFSET),
    );

    create_dataframe_from_sheet(&sheet, off_duty_keyword, &date)
}

fn create_dataframe_from_sheet(
    sheet: &Range<Data>,
    off_duty_keyword: &str,
    date: &str,
) -> DataFrame {
    let columns = transpose(&sheet);
    let state = StructChunked::new(
        "state",
        &[
            Series::new(
                "state",
                &columns[1]
                    .iter()
                    .map(|state| state != off_duty_keyword)
                    .collect::<Vec<bool>>(),
            ),
            Series::new(
                "note",
                columns[3]
                    .iter()
                    .map(|note| !note.is_empty())
                    .collect::<Vec<bool>>(),
            ),
        ],
    )
    .unwrap();

    df!(
        "name" => &columns[0],
        date => state,
    )
    .expect("Failed to create dataframe")
}

fn transpose(sheet: &Range<Data>) -> Vec<Vec<String>> {
    (0..sheet[0].len())
        .map(|i| {
            sheet
                .rows()
                .map(|row| row[i].get_string().unwrap_or_default().to_string())
                .collect::<Vec<String>>()
        })
        .collect()
}
