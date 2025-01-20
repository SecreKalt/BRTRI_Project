use tokio;
use tracing::{info, warn, error, Level};
use tracing_subscriber::FmtSubscriber;
use zeromq::{Socket, SocketRecv, SocketSend};
use std::sync::Arc;
use parking_lot::RwLock;

use crate::{
    config::Config,
    processing::{CloudFilter, CloudOptimizer, ProcessedCloud},
    error::Result,
};

const SHUTDOWN_TIMEOUT: u64 = 5;

struct ProcessingPipeline {
    filter: CloudFilter,
    optimizer: CloudOptimizer,
    running: Arc<RwLock<bool>>,
}

impl ProcessingPipeline {
    fn new(config: &Config) -> Self {
        Self {
            filter: CloudFilter::new(),
            optimizer: CloudOptimizer::new(
                config.compression_level,
                config.noise_threshold,
            ),
            running: Arc::new(RwLock::new(true)),
        }
    }

    async fn run(&self, ios_receiver: impl SocketRecv, blender_sender: impl SocketSend) -> Result<()> {
        while *self.running.read() {
            // Process incoming data
            if let Ok(data) = ios_receiver.recv().await {
                let cloud = ProcessedCloud::new(data.into(), tokio::time::Instant::now().into());
                let filtered = self.filter.apply(&cloud);
                let optimized = self.optimizer.optimize(filtered);
                
                if let Err(e) = blender_sender.send(optimized.into()).await {
                    error!("Failed to send data to Blender: {}", e);
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .pretty()
        .init();

    info!("Starting BRTRI Bridge...");

    // Load configuration
    let config = Config::load()?;
    info!("Configuration loaded successfully");

    // Initialize components
    let pipeline = Arc::new(ProcessingPipeline::new(&config));
    let running = pipeline.running.clone();

    // Setup ZeroMQ sockets
    let ios_receiver = zeromq::SubSocket::new();
    let blender_sender = zeromq::PubSocket::new();

    // Start processing pipeline
    let pipeline_handle = {
        let pipeline = pipeline.clone();
        tokio::spawn(async move {
            if let Err(e) = pipeline.run(ios_receiver, blender_sender).await {
                error!("Processing pipeline error: {}", e);
            }
        })
    };

    // Handle shutdown signals
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Shutdown signal received");
        }
        _ = pipeline_handle => {
            warn!("Pipeline terminated unexpectedly");
        }
    }

    // Graceful shutdown
    *running.write() = false;
    info!("Initiating graceful shutdown...");

    tokio::time::timeout(
        std::time::Duration::from_secs(SHUTDOWN_TIMEOUT),
        pipeline_handle
    ).await??;

    info!("BRTRI Bridge shutdown complete");
    Ok(())
}
