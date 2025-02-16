use zeromq::{Socket, SocketSend};
use std::time::Instant;
use crate::core::monitor::Monitor;
use std::sync::Arc;

const POINT_CLOUD_HEADER: [u8; 4] = [0x50, 0x43, 0x4C, 0x44]; // "PCLD"

pub struct BlenderProtocol {
    socket: Socket,
    frame_counter: u64,
    monitor: Arc<Monitor>,
}

impl BlenderProtocol {
    pub fn new(socket: Socket, monitor: Arc<Monitor>) -> Self {
        Self {
            socket,
            frame_counter: 0,
            monitor,
        }
    }

    pub async fn send_point_cloud(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();

        // Frame format: [Header(4)] [Counter(8)] [Length(4)] [Data(N)]
        let mut frame = Vec::with_capacity(data.len() + 12);
        frame.extend_from_slice(&POINT_CLOUD_HEADER);
        frame.extend_from_slice(&self.frame_counter.to_le_bytes());
        frame.extend_from_slice(&(data.len() as u32).to_le_bytes());
        frame.extend_from_slice(&data);
        
        if let Err(e) = self.socket.send_bytes(&frame).await {
            self.monitor.record_drop();
            return Err(e.into());
        }
        
        self.monitor.record_frame(start.elapsed());
        self.frame_counter += 1;
        Ok(())
    }

    pub fn get_frame_count(&self) -> u64 {
        self.frame_counter
    }
}
