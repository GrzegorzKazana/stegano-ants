use std::time::Instant;

pub fn measure<R, F: FnOnce() -> R>(callback: F) -> (R, u128) {
    let start = Instant::now();
    let result = callback();
    let duration_ms = start.elapsed().as_millis();

    (result, duration_ms)
}
