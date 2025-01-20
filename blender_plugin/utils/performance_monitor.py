import time
from collections import deque

class PerformanceMonitor:
    def __init__(self):
        self.frame_times = deque(maxlen=100)
        self.processing_times = deque(maxlen=100)
        self.last_frame_time = time.time()
        
    def start_frame(self):
        self.last_frame_time = time.time()
        
    def end_frame(self):
        frame_time = time.time() - self.last_frame_time
        self.frame_times.append(frame_time)
        
    def get_fps(self):
        if not self.frame_times:
            return 0
        return 1.0 / (sum(self.frame_times) / len(self.frame_times))
        
    def get_stats(self):
        return {
            'fps': self.get_fps(),
            'avg_process_time': sum(self.processing_times) / len(self.processing_times) if self.processing_times else 0
        }

performance_monitor = PerformanceMonitor()
