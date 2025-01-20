use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct BufferMetrics {
    operations: AtomicUsize,
    drops: AtomicUsize,
    latency_ns: AtomicUsize,
}

pub struct LockFreeBuffer<T> {
    data: Box<[Option<T>]>,
    head: AtomicUsize,
    tail: AtomicUsize,
    capacity: usize,
    metrics: BufferMetrics,
}

impl<T> LockFreeBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        data.resize_with(capacity, || None);
        
        Self {
            data: data.into_boxed_slice(),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            capacity,
            metrics: BufferMetrics {
                operations: AtomicUsize::new(0),
                drops: AtomicUsize::new(0),
                latency_ns: AtomicUsize::new(0),
            },
        }
    }

    #[inline]
    pub fn try_push(&self, item: T) -> Result<(), T> {
        let start = Instant::now();
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) % self.capacity;

        if next_tail == self.head.load(Ordering::Acquire) {
            self.metrics.drops.fetch_add(1, Ordering::Relaxed);
            return Err(item);
        }

        unsafe {
            self.data.get_unchecked(tail)
                .as_ptr()
                .write(Some(item));
        }

        self.tail.store(next_tail, Ordering::Release);
        self.update_metrics(start);
        Ok(())
    }

    #[inline]
    pub fn try_pop(&self) -> Option<T> {
        let start = Instant::now();
        let head = self.head.load(Ordering::Relaxed);
        
        if head == self.tail.load(Ordering::Acquire) {
            return None;
        }

        let item = unsafe {
            self.data.get_unchecked(head)
                .as_ptr()
                .read()
                .take()
        };

        self.head.store((head + 1) % self.capacity, Ordering::Release);
        self.update_metrics(start);
        item
    }

    #[inline(always)]
    fn update_metrics(&self, start: Instant) {
        let latency = start.elapsed().as_nanos() as usize;
        self.metrics.latency_ns.store(latency, Ordering::Relaxed);
        self.metrics.operations.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_metrics(&self) -> (usize, usize, Duration) {
        (
            self.metrics.operations.load(Ordering::Relaxed),
            self.metrics.drops.load(Ordering::Relaxed),
            Duration::from_nanos(self.metrics.latency_ns.load(Ordering::Relaxed) as u64)
        )
    }
}

unsafe impl<T: Send> Send for LockFreeBuffer<T> {}
unsafe impl<T: Send> Sync for LockFreeBuffer<T> {}
