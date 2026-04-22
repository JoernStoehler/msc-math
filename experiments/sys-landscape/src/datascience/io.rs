//! Shared JSONL and exact-coordinate decoding helpers for datascience tables.

use num_bigint::BigInt;
use num_rational::BigRational;
use serde::de::{DeserializeOwned, Deserializer, Error as DeError};
use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::str::FromStr;

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

fn parse_rational_token<E: DeError>(token: &str) -> Result<BigRational, E> {
    if let Some((numer, denom)) = token.split_once('/') {
        let numer =
            BigInt::from_str(numer).map_err(|e| E::custom(format!("bad numerator {token}: {e}")))?;
        let denom = BigInt::from_str(denom)
            .map_err(|e| E::custom(format!("bad denominator {token}: {e}")))?;
        Ok(BigRational::new(numer, denom))
    } else {
        let integer =
            BigInt::from_str(token).map_err(|e| E::custom(format!("bad integer {token}: {e}")))?;
        Ok(BigRational::from_integer(integer))
    }
}

pub fn deserialize_vec4_rational<'de, D>(deserializer: D) -> Result<Vec<[BigRational; 4]>, D::Error>
where
    D: Deserializer<'de>,
{
    Vec::<[String; 4]>::deserialize(deserializer)?
        .into_iter()
        .map(|row| {
            let [x0, x1, x2, x3] = row;
            Ok([
                parse_rational_token::<D::Error>(&x0)?,
                parse_rational_token::<D::Error>(&x1)?,
                parse_rational_token::<D::Error>(&x2)?,
                parse_rational_token::<D::Error>(&x3)?,
            ])
        })
        .collect()
}
