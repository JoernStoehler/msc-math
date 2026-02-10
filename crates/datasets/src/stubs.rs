/// Placeholder volume and capacity computations.
///
/// Return constant 1.0 with a ~1ms busy-spin to exercise the benchmarking
/// infrastructure. Will be replaced by real computations from geom/hk2017.
use geom::polytope::Polytope4D;
use std::time::{Duration, Instant};

const SPIN_DURATION: Duration = Duration::from_micros(1000);

fn busy_spin(duration: Duration) {
    let start = Instant::now();
    while start.elapsed() < duration {
        std::hint::spin_loop();
    }
}

/// Placeholder volume computation. Returns (1.0, elapsed_time).
pub fn volume_stub(_polytope: &Polytope4D) -> (f64, Duration) {
    let start = Instant::now();
    busy_spin(SPIN_DURATION);
    (1.0, start.elapsed())
}

/// Placeholder capacity computation. Returns (1.0, elapsed_time).
pub fn capacity_stub(_polytope: &Polytope4D) -> (f64, Duration) {
    let start = Instant::now();
    busy_spin(SPIN_DURATION);
    (1.0, start.elapsed())
}

#[cfg(test)]
#[path = "stubs_test.rs"]
mod stubs_test;
