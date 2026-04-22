//! Write the final datascience tables.

use crate::features::PolytopeTableRow;
use crate::features_trace::ObservationTableRow;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn write_jsonl<T: Serialize>(path: &Path, rows: &[T]) {
    let file = File::create(path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row).expect("serialize row");
        writeln!(writer).expect("write newline");
    }
    writer.flush().expect("flush output");
}

pub fn write_database(
    out_dir: &Path,
    polytope_rows: &[PolytopeTableRow],
    observation_rows: &[ObservationTableRow],
) {
    std::fs::create_dir_all(out_dir)
        .unwrap_or_else(|e| panic!("create {}: {e}", out_dir.display()));
    write_jsonl(&out_dir.join("polytope-table.jsonl"), polytope_rows);
    write_jsonl(&out_dir.join("observation-table.jsonl"), observation_rows);
}
