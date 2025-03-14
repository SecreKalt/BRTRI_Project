import zmq
import threading
import json
import numpy as np
from collections import deque
import time
from ..operators.error_handler import BRTRI_ErrorHandler

class BRTRIBridgeClient:
    def __init__(self, host="localhost", port=5555):
        self.context = zmq.Context()
        self.socket = None
        self.host = host
        self.port = port
        self.running = False
        self.connection_thread = None
        self.data_buffer = deque(maxlen=30)  # Store max 1 second of data at 30fps
        self.last_process_time = 0
        self.PROCESS_INTERVAL = 1.0/30  # 30 FPS target
        self.frame_buffer = deque(maxlen=5)  # Buffer for frame smoothing
        self.last_frame_time = time.time()
        self.target_fps = 30
        self.frame_interval = 1.0 / self.target_fps
        
    def start(self):
        try:
            self.socket = self.context.socket(zmq.SUB)
            # Add high water mark to prevent memory overflow
            self.socket.set_hwm(1000)
            # Add TCP keep alive to handle network interruptions
            self.socket.setsockopt(zmq.TCP_KEEPALIVE, 1)
            self.socket.setsockopt(zmq.TCP_KEEPALIVE_IDLE, 300)
            # Use CONFLATE to only get latest update
            self.socket.setsockopt(zmq.CONFLATE, 1)
            self.socket.connect(f"tcp://{self.host}:{self.port}")
            self.socket.setsockopt_string(zmq.SUBSCRIBE, "")
            self.running = True
            self.connection_thread = threading.Thread(
                target=self._receive_loop, 
                daemon=True  # Make thread daemon for clean shutdown
            )
            self.connection_thread.start()
            return True
        except Exception as e:
            BRTRI_ErrorHandler.log_error(f"Connection failed: {str(e)}")
            return False
            
    def stop(self):
        self.running = False
        if self.socket:
            self.socket.close()
        if self.connection_thread:
            self.connection_thread.join()
        self.context.term()
        
    def _receive_loop(self):
        poller = zmq.Poller()
        poller.register(self.socket, zmq.POLLIN)
        
        while self.running:
            try:
                # Use poller with timeout
                events = dict(poller.poll(timeout=100))
                if self.socket in events:
                    current_time = time.time()
                    if current_time - self.last_frame_time >= self.frame_interval:
                        data = self.socket.recv_multipart(flags=zmq.NOBLOCK)
                        if data:
                            json_data = json.loads(data[0])
                            # Add timestamp for synchronization
                            json_data['timestamp'] = current_time
                            self.frame_buffer.append(json_data)
                            self._process_frame_buffer()
                            self.last_frame_time = current_time
                            
            except Exception as e:
                BRTRI_ErrorHandler.log_error(f"Reception error: {str(e)}")
                
    def _process_frame_buffer(self):
        if len(self.frame_buffer) >= 2:
            # Average last few frames for smoother visualization
            points_list = [frame['points'] for frame in self.frame_buffer]
            averaged_points = np.mean(points_list, axis=0)
            self.process_data({'points': averaged_points.tolist()})
                
    def process_data(self, data):
        from ..operators.visualizer import BRTRI_OT_UpdateMesh
        BRTRI_OT_UpdateMesh.update_point_cloud(data)
