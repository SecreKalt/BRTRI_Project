use tokio::net::{TcpListener, TcpStream};
use zeromq::{Socket, SocketRecv, SocketSend, PullSocket};
use std::sync::Arc;
use crate::core::monitor::Monitor;
use tracing::{error, info};

const LIDAR_PACKET_HEADER: [u8; 4] = [0x4C, 0x49, 0x44, 0x52]; // "LIDR"
const MAX_PACKET_SIZE: usize = 1024 * 1024; // 1MB
const MAX_RECONNECT_ATTEMPTS: u32 = 5;

pub struct iOSProtocol {
    socket: PullSocket,
    monitor: Arc<Monitor>,
    reconnect_attempts: u32,
}

impl iOSProtocol {
    pub async fn new(port: u16, monitor: Arc<Monitor>) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = PullSocket::new();
        socket.bind(&format!("tcp://*:{}", port)).await?;
        
        Ok(Self {
            socket,
            monitor,
            reconnect_attempts: 0,
        })
    }

    pub async fn receive_frame(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        
        let data = match self.socket.recv_bytes().await {
            Ok(data) => data,
            Err(e) => {
                self.handle_connection_error().await?;
                return Err(e.into());
            }
        };
        
        self.validate_frame(&data)?;
        self.monitor.record_frame(start.elapsed());
        Ok(data[4..].to_vec())
    }

    fn validate_frame(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if data.len() < 4 || &data[0..4] != &LIDAR_PACKET_HEADER {
            self.monitor.record_drop();
            return Err("Invalid packet header".into());
        }
        if data.len() > MAX_PACKET_SIZE {
            return Err("Packet size exceeds maximum".into());
        }
        Ok(())
    }

    async fn handle_connection_error(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.reconnect_attempts += 1;
        if self.reconnect_attempts > MAX_RECONNECT_ATTEMPTS {
            return Err("Max reconnection attempts exceeded".into());
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        Ok(())
    }

    pub async fn handle_buffer_overflow(&self) {
        self.monitor.record_drop();
        error!("Buffer overflow occurred while receiving data from iOS socket");
    }

    pub async fn log_connection_error(&self, error_message: &str) {
        error!("Connection error: {}", error_message);
    }
}
