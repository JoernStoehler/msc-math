//! Qhull subprocess wrapper for 4D volume computation via `qconvex`.
//!
//! Vertex enumeration is handled by the exact rational pipeline
//! (`vertex_enumeration.rs`). This module only provides volume computation
//! via the `qconvex FA` command, which triangulates the convex hull and
//! computes volume from the triangulation.
//!
//! Mathematical correspondence: [def:volume]

use nalgebra::Vector4;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

/// Errors from qhull subprocess interaction.
#[derive(Debug)]
pub enum QhullError {
    /// Qhull computation failed with diagnostic output.
    ComputationFailed(String),
    /// `qconvex` command not found — install `qhull-bin` package.
    QhullNotInstalled,
    /// Failed to write input file or invoke subprocess.
    InputWriteFailed(std::io::Error),
    /// Failed to parse qhull output.
    OutputParseFailed(String),
    /// Output format does not match expectations.
    InvalidOutput(String),
    /// Qhull subprocess exceeded timeout limit.
    Timeout(u64),
}

impl std::fmt::Display for QhullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComputationFailed(stderr) => {
                write!(f, "qhull computation failed: {}", stderr)
            }
            Self::QhullNotInstalled => {
                write!(f, "qconvex command not found - install qhull-bin package")
            }
            Self::InputWriteFailed(e) => {
                write!(f, "failed to write qhull input or invoke subprocess: {}", e)
            }
            Self::OutputParseFailed(msg) => {
                write!(f, "failed to parse qhull output: {}", msg)
            }
            Self::InvalidOutput(msg) => {
                write!(f, "invalid qhull output: {}", msg)
            }
            Self::Timeout(secs) => {
                write!(
                    f,
                    "qhull subprocess exceeded {}s timeout - possible degenerate input",
                    secs
                )
            }
        }
    }
}

impl std::error::Error for QhullError {}

/// Write vertices to a temporary file in qconvex input format.
///
/// Format: first line is "4 N" (dimension, count), then one vertex per line
/// as "v_0 v_1 v_2 v_3".
fn write_qconvex_input(vertices: &[Vector4<f64>]) -> Result<NamedTempFile, QhullError> {
    let mut file = NamedTempFile::new().map_err(QhullError::InputWriteFailed)?;

    writeln!(file, "4 {}", vertices.len()).map_err(QhullError::InputWriteFailed)?;

    for v in vertices {
        writeln!(file, "{} {} {} {}", v[0], v[1], v[2], v[3])
            .map_err(QhullError::InputWriteFailed)?;
    }

    file.flush().map_err(QhullError::InputWriteFailed)?;
    Ok(file)
}

/// Run `qconvex TI <path> FA` and capture stdout.
///
/// FA flag: compute facet areas and total volume.
/// Times out after 60 seconds to prevent hanging on degenerate input.
fn run_qconvex_volume(input_path: &Path) -> Result<String, QhullError> {
    use std::io::Read;
    use std::time::Duration;
    use wait_timeout::ChildExt;

    const TIMEOUT_SECS: u64 = 60;

    let mut child = Command::new("qconvex")
        .arg("TI")
        .arg(input_path)
        .arg("FA")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                QhullError::QhullNotInstalled
            } else {
                QhullError::InputWriteFailed(e)
            }
        })?;

    let status = match child
        .wait_timeout(Duration::from_secs(TIMEOUT_SECS))
        .map_err(QhullError::InputWriteFailed)?
    {
        Some(status) => status,
        None => {
            let _ = child.kill();
            return Err(QhullError::Timeout(TIMEOUT_SECS));
        }
    };

    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    if let Some(mut pipe) = child.stdout.take() {
        pipe.read_to_end(&mut stdout)
            .map_err(QhullError::InputWriteFailed)?;
    }
    if let Some(mut pipe) = child.stderr.take() {
        pipe.read_to_end(&mut stderr)
            .map_err(QhullError::InputWriteFailed)?;
    }

    if !status.success() {
        let stderr_str = String::from_utf8_lossy(&stderr).into_owned();
        return Err(QhullError::ComputationFailed(stderr_str));
    }

    String::from_utf8(stdout).map_err(|e| QhullError::OutputParseFailed(e.to_string()))
}

/// Parse qconvex FA output to extract volume.
///
/// Searches for either "Approximate volume:" (non-simplicial polytopes) or
/// "Total volume:" (simplicial polytopes) and parses the trailing number.
fn parse_fa_output(output: &str) -> Result<f64, QhullError> {
    for line in output.lines() {
        let trimmed = line.trim();

        if let Some(vol_str) = trimmed.strip_prefix("Approximate volume:") {
            let vol: f64 = vol_str
                .trim()
                .parse()
                .map_err(|e| QhullError::OutputParseFailed(format!("volume: {}", e)))?;
            return Ok(vol);
        }

        if let Some(vol_str) = trimmed.strip_prefix("Total volume:") {
            let vol: f64 = vol_str
                .trim()
                .parse()
                .map_err(|e| QhullError::OutputParseFailed(format!("volume: {}", e)))?;
            return Ok(vol);
        }
    }

    Err(QhullError::InvalidOutput(
        "no volume found in qconvex FA output".into(),
    ))
}

/// Compute the volume of a 4D convex polytope using `qconvex FA`.
///
/// Writes vertex coordinates to a temp file, invokes `qconvex`, and parses
/// the volume from the output. Returns 0.0 for fewer than 5 vertices (a 4D
/// polytope with nonzero volume requires at least 5 vertices).
///
/// # Errors
///
/// Returns `QhullError` if qhull is not installed, the subprocess fails,
/// times out, or produces unparseable output.
///
/// Mathematical correspondence: [def:volume]
pub(crate) fn compute_volume_qconvex(vertices: &[Vector4<f64>]) -> Result<f64, QhullError> {
    // In R^4, a polytope with nonzero volume requires at least 5 vertices.
    if vertices.len() < 5 {
        return Ok(0.0);
    }

    let input_file = write_qconvex_input(vertices)?;
    let output = run_qconvex_volume(input_file.path())?;
    parse_fa_output(&output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_fa_approximate_volume() {
        let output = "some header\nApproximate volume:       16.5\nother stuff\n";
        let vol = parse_fa_output(output).unwrap();
        assert!((vol - 16.5).abs() < 1e-12);
    }

    #[test]
    fn parse_fa_total_volume() {
        let output = "some header\n  Total volume:       42.0\nother stuff\n";
        let vol = parse_fa_output(output).unwrap();
        assert!((vol - 42.0).abs() < 1e-12);
    }

    #[test]
    fn parse_fa_no_volume_line() {
        let output = "some header\nno volume here\n";
        let result = parse_fa_output(output);
        assert!(
            matches!(result, Err(QhullError::InvalidOutput(_))),
            "missing volume line should fail: got {result:?}"
        );
    }

    #[test]
    fn parse_fa_bad_volume_value() {
        let output = "Approximate volume:       notanumber\n";
        let result = parse_fa_output(output);
        assert!(
            matches!(result, Err(QhullError::OutputParseFailed(_))),
            "non-numeric volume should fail: got {result:?}"
        );
    }
}
