//! Local method-development helpers for the smoke prediction binary.

mod cache;
mod compute;
mod row;
mod summary;

use row::{PredictionError, PredictionRow};
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;
pub(crate) use summary::SmokeReport;

pub(crate) fn default_output_path() -> &'static str {
    "/tmp/local-sys-methods/smoke-local-prediction.jsonl"
}

pub(crate) fn run_prediction_smoke(output_path: &Path) -> Result<SmokeReport, PredictionError> {
    let rows = compute::prediction_rows()?;
    write_jsonl(output_path, &rows)?;
    Ok(SmokeReport::from_rows(&rows))
}

fn write_jsonl(path: &Path, rows: &[PredictionRow]) -> Result<(), PredictionError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row)?;
        writeln!(writer)?;
    }
    writer.flush()?;
    Ok(())
}
