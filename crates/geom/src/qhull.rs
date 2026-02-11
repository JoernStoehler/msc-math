/// Qhull subprocess wrapper for halfspace intersection.
///
/// **STATUS:** Implemented using subprocess approach (qhalf binary).
///
/// This module calls the `qhalf` binary as a subprocess to compute halfspace
/// intersections in 4D. This approach avoids FFI complexity and directly uses
/// the proven qhull CLI with a stable interface.
///
/// # Implementation
///
/// The `halfspace_intersection_4d` function:
/// 1. Accepts halfspaces in format {x ∈ ℝ⁴ : n·x ≤ h} with unit normals
/// 2. Writes input to temp file in qhull format (dimension=5, one line per halfspace)
/// 3. Invokes `qhalf H0,0,0,0 Fp` subprocess
/// 4. Parses Fp output to extract primal intersection vertices
/// 5. Returns ALL vertices of the polytope defined by the intersection
///
/// # Requirements
///
/// - The `qhull-bin` package must be installed (provides `qhalf` binary)
/// - Vertices satisfy n·v ≤ h + ε for all halfspaces (ε ≈ 1e-6)
/// - Correctly handles 4D hypercube (16 vertices), cross-polytope (8 vertices), simplex (5 vertices)
use nalgebra::Vector4;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

#[derive(Debug)]
pub enum QhullError {
    /// Qhull computation failed with diagnostic output
    ComputationFailed(String),
    /// qhalf command not found - install qhull-bin package
    QhullNotInstalled,
    /// Failed to write input file or invoke subprocess
    InputWriteFailed(std::io::Error),
    /// Failed to parse qhull output
    OutputParseFailed(String),
    /// Output format doesn't match expectations
    InvalidOutput(String),
}

impl std::fmt::Display for QhullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComputationFailed(stderr) => write!(f, "qhull halfspace intersection failed: {}", stderr),
            Self::QhullNotInstalled => write!(f, "qhalf command not found - install qhull-bin package"),
            Self::InputWriteFailed(e) => write!(f, "failed to write qhull input or invoke subprocess: {}", e),
            Self::OutputParseFailed(msg) => write!(f, "failed to parse qhull output: {}", msg),
            Self::InvalidOutput(msg) => write!(f, "invalid qhull output: {}", msg),
        }
    }
}

impl std::error::Error for QhullError {}

/// Write halfspaces to temporary file in qhull format.
///
/// Format: First line is "5 N" (dimension+1, count), subsequent lines are
/// "n₁ n₂ n₃ n₄ -h" (one per halfspace).
fn write_qhull_input(
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> Result<NamedTempFile, QhullError> {
    let mut file = NamedTempFile::new().map_err(QhullError::InputWriteFailed)?;

    // Write dimension+1 and count
    writeln!(file, "5 {}", normals.len()).map_err(QhullError::InputWriteFailed)?;

    // Write each halfspace: n₁ n₂ n₃ n₄ -h
    for (n, &h) in normals.iter().zip(heights) {
        writeln!(file, "{} {} {} {} {}", n[0], n[1], n[2], n[3], -h)
            .map_err(QhullError::InputWriteFailed)?;
    }

    file.flush().map_err(QhullError::InputWriteFailed)?;
    Ok(file)
}

/// Run qhalf subprocess and capture output.
///
/// Invokes: `qhalf TI <path> H0,0,0,0 Fp` to read input file directly.
fn run_qhalf(input_path: &Path) -> Result<String, QhullError> {
    // Spawn qhalf process with TI flag to read file directly
    let output = Command::new("qhalf")
        .arg("TI")
        .arg(input_path)
        .arg("H0,0,0,0")
        .arg("Fp")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                QhullError::QhullNotInstalled
            } else {
                QhullError::InputWriteFailed(e)
            }
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(QhullError::ComputationFailed(stderr));
    }

    String::from_utf8(output.stdout)
        .map_err(|e| QhullError::OutputParseFailed(e.to_string()))
}

/// Parse qhalf Fp output format into vertices.
///
/// Format:
/// ```text
/// 4
/// N
/// v₁.x₁ v₁.x₂ v₁.x₃ v₁.x₄
/// v₂.x₁ v₂.x₂ v₂.x₃ v₂.x₄
/// ...
/// ```
fn parse_fp_output(output: &str) -> Result<Vec<Vector4<f64>>, QhullError> {
    let mut lines = output.lines();

    // Parse dimension (should be 4)
    let dim_line = lines
        .next()
        .ok_or_else(|| QhullError::InvalidOutput("empty output".into()))?;
    let dim: usize = dim_line
        .trim()
        .parse()
        .map_err(|e| QhullError::OutputParseFailed(format!("dimension: {}", e)))?;
    if dim != 4 {
        return Err(QhullError::InvalidOutput(format!(
            "expected dimension 4, got {}",
            dim
        )));
    }

    // Parse count
    let count_line = lines
        .next()
        .ok_or_else(|| QhullError::InvalidOutput("missing vertex count".into()))?;
    let count: usize = count_line
        .trim()
        .parse()
        .map_err(|e| QhullError::OutputParseFailed(format!("count: {}", e)))?;

    // Parse vertices
    let mut vertices = Vec::with_capacity(count);
    for (i, line) in lines.enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue; // Skip empty lines
        }

        let coords: Vec<f64> = trimmed
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| QhullError::OutputParseFailed(format!("line {}: {}", i + 3, e)))?;

        if coords.len() != 4 {
            return Err(QhullError::InvalidOutput(format!(
                "line {} has {} coordinates, expected 4",
                i + 3,
                coords.len()
            )));
        }

        vertices.push(Vector4::new(coords[0], coords[1], coords[2], coords[3]));
    }

    if vertices.len() != count {
        return Err(QhullError::InvalidOutput(format!(
            "expected {} vertices, parsed {}",
            count,
            vertices.len()
        )));
    }

    Ok(vertices)
}

/// Compute vertices of 4D polytope from halfspace intersection.
///
/// Given halfspaces { x : n·x ≤ h } where n ∈ S³, h > 0, computes vertices
/// of the polytope K = ⋂ { x : nᵢ·x ≤ hᵢ }.
///
/// Uses `qhalf` subprocess (from qhull-bin package). Assumes origin [0,0,0,0]
/// is in the interior (guaranteed by h > 0 and unit normals for convex polytopes).
///
/// # Arguments
/// * `normals` - Unit normal vectors (n̂ᵢ ∈ S³)
/// * `heights` - Positive heights (ĥᵢ > 0)
///
/// # Returns
/// Vec of vertices as Vector4<f64>
pub(crate) fn halfspace_intersection_4d(
    normals: &[Vector4<f64>],
    heights: &[f64],
) -> Result<Vec<Vector4<f64>>, QhullError> {
    // Validate input
    debug_assert_eq!(
        normals.len(),
        heights.len(),
        "normals and heights must have the same length"
    );

    // Write halfspaces to temp file
    let input_file = write_qhull_input(normals, heights)?;

    // Run qhalf subprocess
    let output = run_qhalf(input_file.path())?;

    // Parse output and return vertices
    parse_fp_output(&output)
}

#[cfg(test)]
mod test {
    use super::*;
    use nalgebra::Vector4;

    /// Test case: 4D hypercube [-1,1]^4
    /// Expected: 16 vertices at all combinations of (±1, ±1, ±1, ±1)
    #[test]
    fn hypercube_vertices() {
        let normals = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),   // x₁ ≤ 1
            Vector4::new(-1.0, 0.0, 0.0, 0.0),  // -x₁ ≤ 1
            Vector4::new(0.0, 1.0, 0.0, 0.0),   // x₂ ≤ 1
            Vector4::new(0.0, -1.0, 0.0, 0.0),  // -x₂ ≤ 1
            Vector4::new(0.0, 0.0, 1.0, 0.0),   // x₃ ≤ 1
            Vector4::new(0.0, 0.0, -1.0, 0.0),  // -x₃ ≤ 1
            Vector4::new(0.0, 0.0, 0.0, 1.0),   // x₄ ≤ 1
            Vector4::new(0.0, 0.0, 0.0, -1.0),  // -x₄ ≤ 1
        ];
        let heights = vec![1.0; 8];

        let vertices = halfspace_intersection_4d(&normals, &heights)
            .expect("hypercube halfspace intersection should succeed");

        assert_eq!(vertices.len(), 16, "hypercube [-1,1]^4 has 16 vertices");

        // All vertices should satisfy all halfspace constraints: n·v ≤ h
        for v in &vertices {
            for (n, &h) in normals.iter().zip(&heights) {
                assert!(
                    n.dot(v) <= h + 1e-6,
                    "vertex {:?} violates constraint {:?} · x ≤ {}",
                    v,
                    n,
                    h
                );
            }
        }

        // All vertices should be on the boundary (at least one constraint tight)
        for v in &vertices {
            let on_boundary = normals
                .iter()
                .zip(&heights)
                .any(|(n, &h)| (n.dot(v) - h).abs() < 1e-6);
            assert!(
                on_boundary,
                "vertex {:?} is not on any facet boundary",
                v
            );
        }
    }

    /// Test case: 4D cross-polytope (±2·eᵢ for i=1,2,3,4)
    /// Defined by: (±1,±1,±1,±1)/2 · x ≤ 1 (16 facets, 8 vertices)
    #[test]
    fn crosspolytope_vertices() {
        let mut normals = Vec::with_capacity(16);
        for s0 in [-1.0_f64, 1.0] {
            for s1 in [-1.0_f64, 1.0] {
                for s2 in [-1.0_f64, 1.0] {
                    for s3 in [-1.0_f64, 1.0] {
                        normals.push(Vector4::new(s0, s1, s2, s3).normalize());
                    }
                }
            }
        }
        let heights = vec![1.0; 16];

        let vertices = halfspace_intersection_4d(&normals, &heights)
            .expect("crosspolytope halfspace intersection should succeed");

        assert_eq!(vertices.len(), 8, "4D cross-polytope has 8 vertices");

        // Check vertices satisfy constraints
        for v in &vertices {
            for (n, &h) in normals.iter().zip(&heights) {
                assert!(
                    n.dot(v) <= h + 1e-6,
                    "vertex {:?} violates constraint {:?} · x ≤ {}",
                    v,
                    n,
                    h
                );
            }
        }
    }

    /// Test case: 4D simplex with 5 vertices
    /// Approximately the standard simplex conv{0, e₁, e₂, e₃, e₄}, shifted slightly
    /// so all heights are positive (h > 0 required for origin to be interior)
    #[test]
    fn simplex_vertices() {
        let normals = vec![
            Vector4::new(-1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, -1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, -1.0),
            Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
        ];
        let heights = vec![1e-6, 1e-6, 1e-6, 1e-6, 0.5];

        let vertices = halfspace_intersection_4d(&normals, &heights)
            .expect("simplex halfspace intersection should succeed");

        assert_eq!(vertices.len(), 5, "4D simplex has 5 vertices");

        // Verify all vertices satisfy constraints
        for v in &vertices {
            for (n, &h) in normals.iter().zip(&heights) {
                assert!(
                    n.dot(v) <= h + 1e-6,
                    "vertex {:?} violates constraint {:?} · x ≤ {}",
                    v,
                    n,
                    h
                );
            }
        }
    }
}
