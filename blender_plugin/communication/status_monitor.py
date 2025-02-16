import bpy
import time
import threading
from ..operators.error_handler import BRTRI_ErrorHandler

class StatusMonitor:
    def __init__(self):
        self.connection_status = False
        self.data_received = False
        self.last_data_time = 0
        self.monitor_thread = None
        self.running = False

    def start_monitoring(self):
        self.running = True
        self.monitor_thread = threading.Thread(target=self._monitor_loop, daemon=True)
        self.monitor_thread.start()

    def stop_monitoring(self):
        self.running = False
        if self.monitor_thread:
            self.monitor_thread.join()

    def _monitor_loop(self):
        while self.running:
            current_time = time.time()
            if self.connection_status and (current_time - self.last_data_time > 5):
                self.connection_status = False
                BRTRI_ErrorHandler.log_error("Connection lost")
            time.sleep(1)

    def update_connection_status(self, status):
        self.connection_status = status
        if status:
            self.last_data_time = time.time()

    def update_data_received(self):
        self.data_received = True
        self.last_data_time = time.time()

status_monitor = StatusMonitor()
