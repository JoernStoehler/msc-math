use nalgebra::Matrix2;

/// The standard symplectic matrix J in R^2: [[0, -1], [1, 0]].
pub fn j2() -> Matrix2<f64> {
    Matrix2::new(0.0, -1.0, 1.0, 0.0)
}

#[cfg(test)]
#[path = "symplectic_test.rs"]
mod symplectic_test;
