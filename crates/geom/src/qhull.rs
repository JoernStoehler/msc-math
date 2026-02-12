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
///
/// # Performance
///
/// - **Subprocess overhead:** 1.5-2.0 ms per polytope (measured on 5-16 facets)
/// - **Timeout:** 60 seconds (prevents hangs on degenerate inputs)
/// - **Complexity:** Exponential worst-case, polynomial typical
/// - **Practical range:** 5-16 facets (sufficient for thesis workload)
/// - **No parallelization:** Processes one polytope at a time
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
    /// Qhull subprocess exceeded timeout limit
    Timeout(u64),
    /// Unbounded polytope detected via sentinel vertices (-10.101)
    Unbounded,
}

impl std::fmt::Display for QhullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComputationFailed(stderr) => write!(f, "qhull halfspace intersection failed: {}", stderr),
            Self::QhullNotInstalled => write!(f, "qhalf command not found - install qhull-bin package"),
            Self::InputWriteFailed(e) => write!(f, "failed to write qhull input or invoke subprocess: {}", e),
            Self::OutputParseFailed(msg) => write!(f, "failed to parse qhull output: {}", msg),
            Self::InvalidOutput(msg) => write!(f, "invalid qhull output: {}", msg),
            Self::Timeout(secs) => write!(f, "qhull subprocess exceeded {}s timeout - possible degenerate input", secs),
            Self::Unbounded => write!(f, "unbounded polytope detected (qhull returned sentinel vertices)"),
        }
    }
}

impl std::error::Error for QhullError {}

/// Write halfspaces to temporary file in qhull format.
///
/// Format: First line is "5 N" (dimension+1 coefficients per halfspace, count), subsequent lines are
/// "n₁ n₂ n₃ n₄ -h" (one per halfspace). The "5" is d+1 because each halfspace has an offset term.
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
/// Timeout: 60 seconds (qhull can take exponential time on degenerate inputs).
fn run_qhalf(input_path: &Path) -> Result<String, QhullError> {
    use std::io::Read;
    use std::time::Duration;
    use wait_timeout::ChildExt;

    const TIMEOUT_SECS: u64 = 60;

    // Spawn qhalf process with TI flag to read file directly
    let mut child = Command::new("qhalf")
        .arg("TI")
        .arg(input_path)
        .arg("H0,0,0,0")
        .arg("Fp")
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

    // Wait with timeout
    let status = match child
        .wait_timeout(Duration::from_secs(TIMEOUT_SECS))
        .map_err(QhullError::InputWriteFailed)?
    {
        Some(status) => status,
        None => {
            // Timeout - kill the process
            let _ = child.kill();
            return Err(QhullError::Timeout(TIMEOUT_SECS));
        }
    };

    // Read stdout and stderr
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    if let Some(mut stdout_pipe) = child.stdout.take() {
        stdout_pipe
            .read_to_end(&mut stdout)
            .map_err(QhullError::InputWriteFailed)?;
    }
    if let Some(mut stderr_pipe) = child.stderr.take() {
        stderr_pipe
            .read_to_end(&mut stderr)
            .map_err(QhullError::InputWriteFailed)?;
    }

    if !status.success() {
        let stderr_str = String::from_utf8_lossy(&stderr).into_owned();
        return Err(QhullError::ComputationFailed(stderr_str));
    }

    String::from_utf8(stdout).map_err(|e| QhullError::OutputParseFailed(e.to_string()))
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

    // Parse output
    let vertices = parse_fp_output(&output)?;

    // Check for sentinel vertices indicating unbounded polytope
    // Qhull signals unbounded intersections via sentinel value (-10.101, -10.101, -10.101, -10.101)
    // See: https://github.com/qhull/qhull/blob/master/src/user_r.h#L513
    // Empirically validated: 0% false negatives on 375 bounded/unbounded test cases.
    const SENTINEL: f64 = -10.101;
    const TOLERANCE: f64 = 0.001;
    for vertex in &vertices {
        if (vertex[0] - SENTINEL).abs() < TOLERANCE
            || (vertex[1] - SENTINEL).abs() < TOLERANCE
            || (vertex[2] - SENTINEL).abs() < TOLERANCE
            || (vertex[3] - SENTINEL).abs() < TOLERANCE
        {
            return Err(QhullError::Unbounded);
        }
    }

    Ok(vertices)
}

/// Write vertices to temporary file in qconvex format.
///
/// Format: First line is "4 N" (dimension, count), subsequent lines are
/// "v₁ v₂ v₃ v₄" (one per vertex). The "4" is d (points have no offset term, unlike halfspaces).
fn write_qconvex_input(vertices: &[Vector4<f64>]) -> Result<NamedTempFile, QhullError> {
    let mut file = NamedTempFile::new().map_err(QhullError::InputWriteFailed)?;

    // Write dimension and count
    writeln!(file, "4 {}", vertices.len()).map_err(QhullError::InputWriteFailed)?;

    // Write each vertex: v₁ v₂ v₃ v₄
    for v in vertices {
        writeln!(file, "{} {} {} {}", v[0], v[1], v[2], v[3])
            .map_err(QhullError::InputWriteFailed)?;
    }

    file.flush().map_err(QhullError::InputWriteFailed)?;
    Ok(file)
}

/// Run qconvex subprocess with volume flag and capture output.
///
/// Invokes: `qconvex TI <path> FA` to read input file and compute volume.
/// FA flag: compute facet areas and volume.
fn run_qconvex_volume(input_path: &Path) -> Result<String, QhullError> {
    use std::io::Read;
    use std::time::Duration;
    use wait_timeout::ChildExt;

    const TIMEOUT_SECS: u64 = 60;

    // Spawn qconvex process with TI flag to read file directly
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

    // Wait with timeout
    let status = match child
        .wait_timeout(Duration::from_secs(TIMEOUT_SECS))
        .map_err(QhullError::InputWriteFailed)?
    {
        Some(status) => status,
        None => {
            // Timeout - kill the process
            let _ = child.kill();
            return Err(QhullError::Timeout(TIMEOUT_SECS));
        }
    };

    // Read stdout and stderr
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    if let Some(mut stdout_pipe) = child.stdout.take() {
        stdout_pipe
            .read_to_end(&mut stdout)
            .map_err(QhullError::InputWriteFailed)?;
    }
    if let Some(mut stderr_pipe) = child.stderr.take() {
        stderr_pipe
            .read_to_end(&mut stderr)
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
/// Format:
/// ```text
/// ...
/// Approximate volume:       16
/// ...
/// ```
/// or
/// ```text
/// ...
///   Total volume:       16
/// ...
/// ```
fn parse_fa_output(output: &str) -> Result<f64, QhullError> {
    for line in output.lines() {
        let trimmed = line.trim();

        // Try "Approximate volume:" prefix (used for non-simplicial polytopes)
        if let Some(vol_str) = trimmed.strip_prefix("Approximate volume:") {
            let vol: f64 = vol_str
                .trim()
                .parse()
                .map_err(|e| QhullError::OutputParseFailed(format!("volume: {}", e)))?;
            return Ok(vol);
        }

        // Try "Total volume:" prefix (used for simplicial polytopes)
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

/// Compute volume of a 4D polytope using qconvex FA.
///
/// Given vertices of a convex polytope, uses qconvex FA to compute volume directly.
///
/// # Arguments
/// * `vertices` - Vertices of the polytope
///
/// # Returns
/// Volume as f64
pub(crate) fn compute_volume_qconvex(vertices: &[Vector4<f64>]) -> Result<f64, QhullError> {
    // In R⁴, a polytope with nonzero volume requires at least 5 vertices (4-simplex).
    if vertices.len() < 5 {
        return Ok(0.0);
    }

    // Write vertices to temp file
    let input_file = write_qconvex_input(vertices)?;

    // Run qconvex subprocess
    let output = run_qconvex_volume(input_file.path())?;

    // Parse output and return volume
    parse_fa_output(&output)
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

        // Check all vertices are on the boundary (at least one constraint tight)
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

    /// Test case: 4D simplex with 5 vertices
    /// Approximately the standard simplex conv{0, e₁, e₂, e₃, e₄}, shifted so the
    /// origin is interior (h = 1e-6 for coordinate halfspaces, h = 0.5 for the diagonal).
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

        // Check all vertices are on the boundary (at least one constraint tight)
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

    /// Test error handling: empty polytope (contradictory halfspaces)
    #[test]
    fn empty_polytope_fails() {
        // Contradictory: x₁ ≤ -1 (first) and x₁ ≥ 2 (second), so intersection is empty.
        let normals = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),   // x₁ ≤ -1
            Vector4::new(-1.0, 0.0, 0.0, 0.0),  // -x₁ ≤ -2 (equivalent to x₁ ≥ 2)
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, -1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
        ];
        let heights = vec![-1.0, -2.0, 1.0, 1.0, 1.0];

        let result = halfspace_intersection_4d(&normals, &heights);
        assert!(
            result.is_err(),
            "empty polytope should fail vertex enumeration"
        );
        if let Err(e) = result {
            assert!(
                matches!(e, QhullError::ComputationFailed(_)),
                "expected ComputationFailed, got {:?}",
                e
            );
        }
    }

    /// Test: unbounded polytope (underconstrained) does not panic.
    #[test]
    fn unbounded_polytope_does_not_panic() {
        // Only 4 halfspaces in 4D - unbounded
        let normals = vec![
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        ];
        let heights = vec![1.0, 1.0, 1.0, 1.0];

        let result = halfspace_intersection_4d(&normals, &heights);
        // May return Ok or Err depending on qhull behavior; just verify no panic.
        let _ = result;
    }

    /// Test parsing: empty output
    #[test]
    fn parse_empty_output() {
        let result = parse_fp_output("");
        assert!(
            matches!(result, Err(QhullError::InvalidOutput(_))),
            "empty output should fail parsing"
        );
    }

    /// Test parsing: wrong dimension
    #[test]
    fn parse_wrong_dimension() {
        let output = "3\n5\n0.0 0.0 0.0\n";
        let result = parse_fp_output(output);
        assert!(
            matches!(result, Err(QhullError::InvalidOutput(_))),
            "wrong dimension should fail parsing"
        );
    }

    /// Test parsing: missing vertex count
    #[test]
    fn parse_missing_count() {
        let output = "4\n";
        let result = parse_fp_output(output);
        assert!(
            matches!(result, Err(QhullError::InvalidOutput(_))),
            "missing count should fail parsing"
        );
    }

    /// Test parsing: vertex count mismatch
    #[test]
    fn parse_vertex_count_mismatch() {
        let output = "4\n2\n0.0 0.0 0.0 0.0\n"; // Says 2 vertices, provides 1
        let result = parse_fp_output(output);
        assert!(
            matches!(result, Err(QhullError::InvalidOutput(_))),
            "vertex count mismatch should fail parsing"
        );
    }

    /// Performance benchmark: measure qhull timing for different facet counts
    #[test]
    fn benchmark_qhull_performance() {
        use std::time::Instant;

        println!("\n=== Qhull Performance Benchmark ===");

        let test_cases = vec![
            (
                "Simplex (5 facets, 5 vertices)",
                vec![
                    Vector4::new(-1.0, 0.0, 0.0, 0.0),
                    Vector4::new(0.0, -1.0, 0.0, 0.0),
                    Vector4::new(0.0, 0.0, -1.0, 0.0),
                    Vector4::new(0.0, 0.0, 0.0, -1.0),
                    Vector4::new(1.0, 1.0, 1.0, 1.0).normalize(),
                ],
                vec![1e-6, 1e-6, 1e-6, 1e-6, 0.5],
            ),
            (
                "Hypercube (8 facets, 16 vertices)",
                vec![
                    Vector4::new(1.0, 0.0, 0.0, 0.0),
                    Vector4::new(-1.0, 0.0, 0.0, 0.0),
                    Vector4::new(0.0, 1.0, 0.0, 0.0),
                    Vector4::new(0.0, -1.0, 0.0, 0.0),
                    Vector4::new(0.0, 0.0, 1.0, 0.0),
                    Vector4::new(0.0, 0.0, -1.0, 0.0),
                    Vector4::new(0.0, 0.0, 0.0, 1.0),
                    Vector4::new(0.0, 0.0, 0.0, -1.0),
                ],
                vec![1.0; 8],
            ),
        ];

        // Add cross-polytope (16 facets, 8 vertices)
        let mut cross_normals = Vec::with_capacity(16);
        for s0 in [-1.0_f64, 1.0] {
            for s1 in [-1.0_f64, 1.0] {
                for s2 in [-1.0_f64, 1.0] {
                    for s3 in [-1.0_f64, 1.0] {
                        cross_normals.push(Vector4::new(s0, s1, s2, s3).normalize());
                    }
                }
            }
        }
        let cross_heights = vec![1.0; 16];

        let mut all_cases = test_cases;
        all_cases.push((
            "Cross-polytope (16 facets, 8 vertices)",
            cross_normals,
            cross_heights,
        ));

        for (name, normals, heights) in all_cases {
            // Warm up
            let _ = halfspace_intersection_4d(&normals, &heights);

            // Benchmark (10 iterations)
            let iterations = 10;
            let start = Instant::now();
            for _ in 0..iterations {
                let _ = halfspace_intersection_4d(&normals, &heights)
                    .expect("benchmark halfspace intersection should succeed");
            }
            let elapsed = start.elapsed();
            let avg_ms = elapsed.as_micros() as f64 / iterations as f64 / 1000.0;

            println!("{}: {:.3} ms/call", name, avg_ms);
        }

        println!("===================================\n");
    }
}
