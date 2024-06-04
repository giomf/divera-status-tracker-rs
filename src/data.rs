use std::path::Path;

use crate::io;
use either::Either;
use glob::glob;
use polars::prelude::*;

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
