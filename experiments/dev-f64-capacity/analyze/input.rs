use exp_dev_f64_capacity::ScanRow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub(crate) fn read_rows(path: &Path) -> Vec<ScanRow> {
    let file = File::open(path).unwrap_or_else(|e| panic!("open {}: {e}", path.display()));
    let reader = BufReader::new(file);
    reader
        .lines()
        .enumerate()
        .filter_map(|(line_idx, line)| {
            let line =
                line.unwrap_or_else(|e| panic!("read {}:{}: {e}", path.display(), line_idx + 1));
            let line = line.trim();
            (!line.is_empty()).then(|| {
                serde_json::from_str(line).unwrap_or_else(|e| {
                    panic!("parse {}:{} as scan row: {e}", path.display(), line_idx + 1)
                })
            })
        })
        .collect()
}
