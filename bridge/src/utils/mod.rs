use std::time::{Duration, Instant};
use tracing::{debug, warn};

pub struct PerformanceMetrics {
    start_time: Instant,
    thresholds: Thresholds,
}

#[derive(Debug, Clone)]
pub struct Thresholds {
    warning_ms: u64,
    critical_ms: u64,
}

impl PerformanceMetrics {
    pub fn new(warning_ms: u64, critical_ms: u64) -> Self {
        Self {
            start_time: Instant::now(),
            thresholds: Thresholds { warning_ms, critical_ms },
        }
    }

    #[inline(always)]
    pub fn measure<F, T>(&self, name: &str, f: F) -> T 
    where
        F: FnOnce() -> T
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        
        self.log_performance(name, duration);
        result
    }

    fn log_performance(&self, operation: &str, duration: Duration) {
        let ms = duration.as_millis() as u64;
        match ms {
            ms if ms >= self.thresholds.critical_ms => 
                warn!("{} took {}ms (critical threshold)", operation, ms),
            ms if ms >= self.thresholds.warning_ms => 
                debug!("{} took {}ms (warning threshold)", operation, ms),
            _ => debug!("{} took {}ms", operation, ms),
        }
    }
}
