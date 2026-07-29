use serde::Serialize;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn prepare_empty_directory(path: &Path) -> Result<(), String> {
    fs::create_dir_all(path)
        .map_err(|error| format!("create output directory {}: {error}", path.display()))?;
    let mut entries = fs::read_dir(path)
        .map_err(|error| format!("read output directory {}: {error}", path.display()))?;
    if entries.next().is_some() {
        return Err(format!(
            "output directory {} is not empty; choose a fresh directory",
            path.display()
        ));
    }
    Ok(())
}

pub fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), String> {
    let file = File::create(path).map_err(|error| format!("create {}: {error}", path.display()))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, value)
        .map_err(|error| format!("serialize {}: {error}", path.display()))?;
    writer
        .write_all(b"\n")
        .map_err(|error| format!("write {}: {error}", path.display()))?;
    writer
        .flush()
        .map_err(|error| format!("flush {}: {error}", path.display()))
}

pub fn write_jsonl<T: Serialize>(path: &Path, values: &[T]) -> Result<(), String> {
    let file = File::create(path).map_err(|error| format!("create {}: {error}", path.display()))?;
    let mut writer = BufWriter::new(file);
    for value in values {
        serde_json::to_writer(&mut writer, value)
            .map_err(|error| format!("serialize {}: {error}", path.display()))?;
        writer
            .write_all(b"\n")
            .map_err(|error| format!("write {}: {error}", path.display()))?;
    }
    writer
        .flush()
        .map_err(|error| format!("flush {}: {error}", path.display()))
}
