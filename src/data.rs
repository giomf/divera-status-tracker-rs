use std::path::Path;

use crate::io;
use either::Either;
use glob::glob;
use polars::prelude::*;

lazy_static::lazy_static! {

    static ref NAMES : Expr =  col("*").exclude(["date"]);
    static ref STATE : Expr = NAMES.clone().struct_().field_by_name("state");
    static ref NOTE : Expr = NAMES.clone().struct_().field_by_name("note");
    static ref COUNT : Expr = NAMES.clone().count();
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
                JoinArgs::new(JoinType::Left),
            )
            .collect()
            .unwrap()
    })
    .unwrap()
    .transpose(Some("date"), Some(Either::Left("name".to_string())))
    .unwrap()
}

pub fn calculate(df: DataFrame) -> DataFrame {
    let on_duty = calculate_on_duty(&df);
    let off_duty_with_note = calculate_off_duty_with_note(&df);

    let result = on_duty
        .clone()
        .lazy()
        .join(
            off_duty_with_note.clone().lazy(),
            [col("name")],
            [col("name")],
            JoinArgs::new(JoinType::Left),
        )
        .collect()
        .unwrap();

    result
        .sort(
            ["on-duty"],
            SortMultipleOptions::default().with_order_descending(true),
        )
        .unwrap()
}

fn calculate_on_duty(df: &DataFrame) -> DataFrame {
    let mut on_duty = df
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

    on_duty.rename("column_0", "on-duty").unwrap().clone()
}

fn calculate_off_duty_with_note(df: &DataFrame) -> DataFrame {
    let mut off_duty_with_note = df
        .clone()
        .lazy()
        .select([(STATE
            .clone()
            .eq(lit(false))
            .and(NOTE.clone().eq(lit(true)))
            .sum()
            .cast(DataType::Float32)
            / STATE.clone().xor(lit(true)).sum()
            * lit(100))
        .name()
        .keep()])
        .collect()
        .unwrap()
        .transpose(Some("name"), None)
        .unwrap();

    off_duty_with_note
        .rename("column_0", "off-duty-with-note")
        .unwrap()
        .clone()
}
