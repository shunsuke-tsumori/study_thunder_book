use std::time::Instant;

pub struct TimeKeeper {
    start_time: Instant,
    time_threshold_ms: i64,
}

impl TimeKeeper {
    pub fn new(time_threshold_ms: i64) -> Self {
        Self {
            start_time: Instant::now(),
            time_threshold_ms,
        }
    }

    pub fn is_time_over(&self) -> bool {
        let elapsed_ms = self.start_time.elapsed().as_millis() as i64;
        elapsed_ms >= self.time_threshold_ms
    }
}
