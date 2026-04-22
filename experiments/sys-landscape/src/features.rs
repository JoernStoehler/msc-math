//! Shared shell helpers for sys-landscape dataset feature binaries.

use num_bigint::BigInt;
use num_rational::BigRational;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct StandardFeatureArgs {
    pub normalized_dir: PathBuf,
    pub out: PathBuf,
}

pub fn default_feature_output_path(stem: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before UNIX_EPOCH")
        .as_millis();
    std::env::temp_dir().join(format!("sys-feature-{stem}-{stamp}.jsonl"))
}

pub fn parse_standard_feature_args(stem: &str) -> StandardFeatureArgs {
    let args: Vec<String> = std::env::args().collect();
    let mut normalized_dir: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut i = 1usize;
    while i < args.len() {
        match args[i].as_str() {
            "--normalized-dir" => {
                let value = args.get(i + 1).expect("--normalized-dir requires a value");
                normalized_dir = Some(PathBuf::from(value));
                i += 2;
            }
            "--out" => {
                let value = args.get(i + 1).expect("--out requires a value");
                out = Some(PathBuf::from(value));
                i += 2;
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    StandardFeatureArgs {
        normalized_dir: normalized_dir.expect("--normalized-dir is required"),
        out: out.unwrap_or_else(|| default_feature_output_path(stem)),
    }
}

pub fn read_jsonl<T: DeserializeOwned>(path: &Path) -> Vec<T> {
    let file = File::open(path).unwrap_or_else(|e| panic!("open {}: {e}", path.display()));
    let reader = BufReader::new(file);
    reader
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            line.unwrap_or_else(|e| panic!("read {} line {}: {e}", path.display(), idx + 1))
        })
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            serde_json::from_str::<T>(&line)
                .unwrap_or_else(|e| panic!("parse {}: {e}\nline={line}", path.display()))
        })
        .collect()
}

pub fn write_jsonl<T: Serialize>(path: &Path, rows: &[T]) {
    let file = File::create(path).unwrap_or_else(|e| panic!("create {}: {e}", path.display()));
    let mut writer = BufWriter::new(file);
    for row in rows {
        serde_json::to_writer(&mut writer, row).expect("serialize row");
        writeln!(writer).expect("write newline");
    }
    writer.flush().expect("flush output");
}

pub fn parse_rational(token: &str) -> BigRational {
    if let Some((numer, denom)) = token.split_once('/') {
        let numer =
            BigInt::from_str(numer).unwrap_or_else(|e| panic!("bad numerator {token}: {e}"));
        let denom =
            BigInt::from_str(denom).unwrap_or_else(|e| panic!("bad denominator {token}: {e}"));
        BigRational::new(numer, denom)
    } else {
        BigRational::from_integer(
            BigInt::from_str(token).unwrap_or_else(|e| panic!("bad integer {token}: {e}")),
        )
    }
}

pub fn parse_vec4(data: &[[String; 4]]) -> Vec<[BigRational; 4]> {
    data.iter()
        .map(|row| std::array::from_fn(|i| parse_rational(&row[i])))
        .collect()
}
