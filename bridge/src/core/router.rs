use tokio::sync::{mpsc, watch};
use std::sync::Arc;
use crate::{
    error::Result,
    core::buffer::LockFreeBuffer,
};
use zeromq::{Socket, SocketRecv, SocketSend};
use std::time::Duration;
use crate::error::Error;
use tracing::{error, info};

pub struct Router {
    input_buffer: Arc<LockFreeBuffer<Vec<u8>>>,
    output_buffer: Arc<LockFreeBuffer<Vec<u8>>>,
    shutdown: watch::Receiver<bool>,
}

impl Router {
    pub fn new(capacity: usize, shutdown: watch::Receiver<bool>) -> Self {
        Self {
            input_buffer: Arc::new(LockFreeBuffer::new(capacity)),
            output_buffer: Arc::new(LockFreeBuffer::new(capacity)),
            shutdown,
        }
    }

    pub async fn run(
        &self,
        mut ios_socket: impl SocketRecv + Send + 'static,
        mut blender_socket: impl SocketSend + Send + 'static,
    ) -> Result<()> {
        let (tx, mut rx) = mpsc::channel(1024);
        
        // iOS receiver task
        let input_buffer = self.input_buffer.clone();
        let mut shutdown = self.shutdown.clone();
        tokio::spawn(async move {
            while !*shutdown.borrow() {
                if let Ok(data) = ios_socket.recv_bytes().await {
                    if input_buffer.try_push(data).is_err() {
                        // Handle buffer overflow
                        input_buffer.handle_overflow();
                        error!("Buffer overflow occurred while receiving data from iOS socket");
                    }
                }
            }
        });

        // Blender sender task
        let output_buffer = self.output_buffer.clone();
        let mut shutdown = self.shutdown.clone();
        tokio::spawn(async move {
            while !*shutdown.borrow() {
                if let Some(data) = output_buffer.try_pop() {
                    if let Err(e) = blender_socket.send_bytes(&data).await {
                        // Handle send error
                        error!("Failed to send data to Blender socket: {}", e);
                    }
                }
                tokio::time::sleep(std::time::Duration::from_micros(100)).await;
            }
        });

        // Process data
        while !*self.shutdown.borrow() {
            if let Some(data) = self.input_buffer.try_pop() {
                // Process data here
                if self.output_buffer.try_push(data).is_err() {
                    // Handle buffer overflow
                    self.output_buffer.handle_overflow();
                    error!("Buffer overflow occurred while processing data");
                }
            }
            tokio::task::yield_now().await;
        }

        Ok(())
    }

    pub async fn recover_connection(&self) -> Result<()> {
        // Implement connection recovery logic
        Ok(())
    }
}
