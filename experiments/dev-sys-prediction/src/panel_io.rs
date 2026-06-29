use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub(crate) fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    let file =
        File::open(path).unwrap_or_else(|err| panic!("failed to open {}: {err}", path.display()));
    serde_json::from_reader(file)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

pub(crate) fn read_required_json<T: for<'de> Deserialize<'de>>(path: &Path) -> T {
    require_nonempty(path);
    read_json(path)
}

pub(crate) fn write_json<T: Serialize>(path: &Path, value: &T) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("failed to create output parent directory");
    }
    let file = File::create(path)
        .unwrap_or_else(|err| panic!("failed to create {}: {err}", path.display()));
    serde_json::to_writer_pretty(file, value)
        .unwrap_or_else(|err| panic!("failed to write {}: {err}", path.display()));
}

pub(crate) fn load_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Vec<T> {
    let file =
        File::open(path).unwrap_or_else(|err| panic!("failed to open {}: {err}", path.display()));
    let reader = BufReader::new(file);
    reader
        .lines()
        .enumerate()
        .filter_map(|(idx, line)| {
            let line = line.unwrap_or_else(|err| {
                panic!("failed to read {}:{}: {err}", path.display(), idx + 1)
            });
            (!line.trim().is_empty()).then(|| {
                serde_json::from_str(&line).unwrap_or_else(|err| {
                    panic!("failed to parse {}:{}: {err}", path.display(), idx + 1)
                })
            })
        })
        .collect()
}

pub(crate) fn write_jsonl<T: Serialize>(path: &Path, rows: &[T]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("failed to create output parent directory");
    }
    let file = File::create(path)
        .unwrap_or_else(|err| panic!("failed to create {}: {err}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row)
            .unwrap_or_else(|err| panic!("failed to write {}: {err}", path.display()));
        writeln!(writer).expect("failed to finish jsonl row");
    }
}

pub(crate) fn require_nonempty(path: &Path) {
    let metadata = fs::metadata(path)
        .unwrap_or_else(|err| panic!("required file {} is missing: {err}", path.display()));
    assert!(
        metadata.len() > 0,
        "required file {} is empty",
        path.display()
    );
}
