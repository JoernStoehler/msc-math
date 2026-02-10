/// Qhull C library FFI wrapper for halfspace intersection.
///
/// This module isolates all qhull C API calls so future code doesn't need
/// to deal with raw FFI. The C API is weird (command-line args baked in,
/// multi-step initialization, void functions that may call exit(), etc.).
use nalgebra::Vector4;
use std::ffi::CString;
use std::ptr;

#[derive(Debug)]
pub enum QhullError {
    /// Qhull computation failed (details printed to stderr by qhull)
    ComputationFailed,
}

impl std::fmt::Display for QhullError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ComputationFailed => write!(f, "qhull halfspace intersection failed"),
        }
    }
}

impl std::error::Error for QhullError {}

/// Compute vertices of 4D polytope from halfspace intersection.
///
/// Given halfspaces { x : n·x ≤ h } where n ∈ S³, h > 0, computes vertices
/// of the polytope K = ⋂ { x : nᵢ·x ≤ hᵢ }.
///
/// Uses qhull C library. Assumes origin [0,0,0,0] is in the interior (guaranteed
/// by h > 0 and unit normals for convex polytopes).
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
    // Convert to qhull format: a₁x₁ + a₂x₂ + a₃x₃ + a₄x₄ + b ≤ 0
    // Our format: n·x ≤ h → n·x - h ≤ 0
    // So: [n₁, n₂, n₃, n₄, -h]
    let num_halfspaces = normals.len();
    let dim = 4;
    let coords_per_halfspace = dim + 1; // 5

    let mut halfspaces: Vec<f64> = Vec::with_capacity(num_halfspaces * coords_per_halfspace);

    for (n, &h) in normals.iter().zip(heights.iter()) {
        halfspaces.push(n[0]);
        halfspaces.push(n[1]);
        halfspaces.push(n[2]);
        halfspaces.push(n[3]);
        halfspaces.push(-h);
    }

    unsafe {
        let mut qh: qhull_sys::qhT = std::mem::zeroed();

        // Build argv for qhull initialization (following qhull Rust crate pattern)
        let argv_strs = vec![
            CString::new("qhull").unwrap(),
            CString::new("H").unwrap(),  // Halfspace mode
        ];
        let argv: Vec<*const i8> = argv_strs
            .iter()
            .map(|s| s.as_ptr())
            .chain(std::iter::once(ptr::null()))
            .collect();
        let argc = (argv.len() - 1) as i32;

        // Step 1: Initialize qhull (from qhull Rust crate build() method)
        // Open /dev/null for all output streams to suppress qhull messages
        let devnull = CString::new("/dev/null").unwrap();
        let read_mode = CString::new("r").unwrap();
        let write_mode = CString::new("w").unwrap();
        let stdin_file = libc::fopen(devnull.as_ptr(), read_mode.as_ptr());
        let stdout_file = libc::fopen(devnull.as_ptr(), write_mode.as_ptr());
        let stderr_file = libc::fopen(devnull.as_ptr(), write_mode.as_ptr());

        if stdin_file.is_null() || stdout_file.is_null() || stderr_file.is_null() {
            return Err(QhullError::ComputationFailed);
        }

        qhull_sys::qh_init_A(
            &mut qh as *mut _,
            stdin_file as *mut _,     // stdin -> /dev/null
            stdout_file as *mut _,    // stdout -> /dev/null
            stderr_file as *mut _,    // stderr -> /dev/null
            argc,
            argv.as_ptr() as *mut *mut i8,
        );

        let qh_ptr = &mut qh as *mut _;

        // NOTE: Skipping qh_checkflags and qh_initflags - they cause crashes
        // qh_init_A should have already set up the flags from argv

        // Set feasible point to origin (interior point guaranteed by Definition 3.2)
        let mut feasible_point = vec![0.0; dim];
        qh.feasible_point = feasible_point.as_mut_ptr();
        std::mem::forget(feasible_point); // qhull will free this, so don't let Rust drop it

        // Step 3: Initialize with halfspace data
        qhull_sys::qh_init_B(
            qh_ptr,
            halfspaces.as_mut_ptr(),
            num_halfspaces as i32,
            dim as i32,
            0, // ismalloc = false (we manage memory)
        );

        // Step 4: Run the qhull algorithm
        qhull_sys::qh_qhull(qh_ptr);

        // Step 5: Prepare output
        qhull_sys::qh_prepare_output(qh_ptr);

        // Extract vertices from vertex_list
        let mut vertices = Vec::new();
        let mut vertex_ptr = qh.vertex_list;

        while !vertex_ptr.is_null() {
            let vertex = &*vertex_ptr;
            if !vertex.point.is_null() {
                let coords = std::slice::from_raw_parts(vertex.point, dim);
                vertices.push(Vector4::new(coords[0], coords[1], coords[2], coords[3]));
            }
            vertex_ptr = vertex.next;
        }

        // Cleanup: qh_freeqhull handles all memory cleanup including FILE pointers
        qhull_sys::qh_freeqhull(qh_ptr, !qhull_sys::qh_ALL);

        // NOTE: Do NOT close FILE handles - qh_freeqhull already freed them

        Ok(vertices)
    }
}
