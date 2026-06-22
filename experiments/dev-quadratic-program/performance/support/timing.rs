use std::time::{Duration, Instant};

pub fn timed<T>(operation: impl FnOnce() -> T) -> (T, f64) {
    let start = Instant::now();
    let value = operation();
    (value, ms(start.elapsed()))
}

pub fn timed_result<T, E>(operation: impl FnOnce() -> Result<T, E>) -> (Result<T, E>, f64) {
    let start = Instant::now();
    let value = operation();
    (value, ms(start.elapsed()))
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}
