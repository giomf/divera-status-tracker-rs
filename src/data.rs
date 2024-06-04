use std::path::Path;

use crate::io;
use either::Either;
use glob::glob;
use polars::prelude::*;

const NAME_COLUMN_NAME: &str = "Name";
const TOTAL_ON_DUTY_COLUMN_NAME: &str = "Total On-Duty";
const WEEKEND_ON_DUTY_COLUMN_NAME: &str = "Weekend On-Duty";
const OFF_DUTY_WITH_NOTE_COLUMN_NAME: &str = "Off-Duty with Note";

lazy_static::lazy_static! {
    static ref NAMES: Expr = col("*").exclude(["date"]);
    static ref STATE: Expr = NAMES.clone().struct_().field_by_name("state");
    static ref NOTE: Expr = NAMES.clone().struct_().field_by_name("note");
    static ref COUNT: Expr = NAMES.clone().count();
}

pub fn aggregate_attachments(attachments_path: &Path, off_duty_keyword: &str) -> DataFrame {
    glob(
        attachments_path
            .join("*.xlsx")
            .as_os_str()
            .to_str()
            .unwrap(),
    )
    .expect("Failed to read glob pattern")
    .map(|path| io::read_excel(&path.unwrap(), off_duty_keyword))
    .reduce(|acc, df| {
        acc.clone()
            .lazy()
            .join(
                df.clone().lazy(),
                [col("name")],
                [col("name")],
                JoinArgs::new(JoinType::Outer { coalesce: true }),
            )
            .collect()
            .unwrap()
    })
    .unwrap()
    .transpose(Some("date"), Some(Either::Left("name".to_string())))
    .unwrap()
    .lazy()
    .select([
        col("date")
            .clone()
            .str()
            .to_datetime(None, None, StrptimeOptions::default(), lit("raise")),
        NAMES.clone(),
    ])
    .collect()
    .unwrap()
}

pub fn calculate(df: DataFrame) -> DataFrame {
    let total_on_duty = calculate_total_on_duty(&df);
    let weekend_on_duty = calculate_weekend_on_duty(&df);
    let off_duty_with_note = calculate_off_duty_with_note(&df);

    let mut result = total_on_duty
        .clone()
        .lazy()
        .join(
            weekend_on_duty.lazy(),
            [col("name")],
            [col("name")],
            JoinArgs::new(JoinType::Left),
        )
        .join(
            off_duty_with_note.lazy(),
            [col("name")],
            [col("name")],
            JoinArgs::new(JoinType::Left),
        )
        .collect()
        .unwrap();

    let result = result.rename("name", NAME_COLUMN_NAME).unwrap().clone();

    result
        .sort([TOTAL_ON_DUTY_COLUMN_NAME], true, true)
        .unwrap()
}

fn calculate_total_on_duty(df: &DataFrame) -> DataFrame {
    let mut total_on_duty = df
        .clone()
        .lazy()
        .select([
            (STATE.clone().sum().cast(DataType::Float32) / COUNT.clone() * lit(100))
                .name()
                .keep(),
        ])
        .collect()
        .unwrap()
        .transpose(Some("name"), None)
        .unwrap();

    total_on_duty
        .rename("column_0", TOTAL_ON_DUTY_COLUMN_NAME)
        .unwrap()
        .clone()
}

fn calculate_weekend_on_duty(df: &DataFrame) -> DataFrame {
    let dates = col("date");

    let weekend = dates.clone().dt().weekday().gt(lit(5)).or(dates
        .clone()
        .dt()
        .weekday()
        .eq(lit(5))
        .and(dates.clone().dt().hour().gt_eq(lit(18))));

    let weekend_on_duty = STATE.clone().and(weekend.clone()).sum();

    let mut weekend_on_duty = df
        .clone()
        .lazy()
        .select([
            (weekend_on_duty.cast(DataType::Float32) / weekend.sum() * lit(100))
                .name()
                .keep(),
        ])
        .collect()
        .unwrap()
        .transpose(Some("name"), None)
        .unwrap();

    weekend_on_duty
        .rename("column_0", WEEKEND_ON_DUTY_COLUMN_NAME)
        .unwrap()
        .clone()
}

fn calculate_off_duty_with_note(df: &DataFrame) -> DataFrame {
    let off_duty_with_note = STATE.clone().not().and(NOTE.clone()).sum();

    let mut off_duty_with_note =
        df.clone()
            .lazy()
            .select([(off_duty_with_note.cast(DataType::Float32)
                / STATE.clone().xor(lit(true)).sum()
                * lit(100))
            .name()
            .keep()])
            .collect()
            .unwrap()
            .transpose(Some("name"), None)
            .unwrap();

    off_duty_with_note
        .rename("column_0", OFF_DUTY_WITH_NOTE_COLUMN_NAME)
        .unwrap()
        .clone()
}
