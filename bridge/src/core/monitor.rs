use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::time;

#[derive(Debug)]
pub struct SystemMetrics {
    pub fps: f32,
    pub latency_ms: f32,
    pub buffer_utilization: f32,
    pub dropped_frames: usize,
}

pub struct Monitor {
    start_time: Instant,
    frame_count: AtomicUsize,
    dropped_frames: AtomicUsize,
    last_latency: AtomicUsize,
}

impl Monitor {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            frame_count: AtomicUsize::new(0),
            dropped_frames: AtomicUsize::new(0),
            last_latency: AtomicUsize::new(0),
        }
    }

    pub async fn start_monitoring(&self) {
        let mut interval = time::interval(Duration::from_secs(1));
        loop {
            interval.tick().await;
            let metrics = self.get_metrics();
            println!("System Metrics: {:?}", metrics);
        }
    }

    pub fn record_frame(&self, latency: Duration) {
        self.frame_count.fetch_add(1, Ordering::Relaxed);
        self.last_latency.store(latency.as_micros() as usize, Ordering::Relaxed);
    }

    pub fn record_drop(&self) {
        self.dropped_frames.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_metrics(&self) -> SystemMetrics {
        let elapsed = self.start_time.elapsed();
        let frames = self.frame_count.load(Ordering::Relaxed);
        let drops = self.dropped_frames.load(Ordering::Relaxed);
        let latency = self.last_latency.load(Ordering::Relaxed);

        SystemMetrics {
            fps: frames as f32 / elapsed.as_secs_f32(),
            latency_ms: latency as f32 / 1000.0,
            buffer_utilization: 0.0, // To be implemented with buffer integration
            dropped_frames: drops,
        }
    }

    pub fn reset(&self) {
        self.frame_count.store(0, Ordering::Relaxed);
        self.dropped_frames.store(0, Ordering::Relaxed);
        self.last_latency.store(0, Ordering::Relaxed);
    }
}
