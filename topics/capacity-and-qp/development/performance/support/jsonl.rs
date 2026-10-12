use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct JsonlWriter {
    writer: BufWriter<File>,
}

impl JsonlWriter {
    pub fn create(path: &Path) -> Result<Self, String> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .map_err(|error| format!("create {}: {error}", path.display()))?;
        Ok(Self {
            writer: BufWriter::new(file),
        })
    }

    pub fn write<T: Serialize>(&mut self, value: &T) -> Result<(), String> {
        serde_json::to_writer(&mut self.writer, value)
            .map_err(|error| format!("serialize jsonl row: {error}"))?;
        self.writer
            .write_all(b"\n")
            .map_err(|error| format!("write jsonl newline: {error}"))
    }

    pub fn flush(&mut self) -> Result<(), String> {
        self.writer
            .flush()
            .map_err(|error| format!("flush jsonl writer: {error}"))
    }
}
