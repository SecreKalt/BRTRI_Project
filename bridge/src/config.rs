use serde::Deserialize;
use std::path::PathBuf;
use crate::error::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub compression_level: u8,
    pub noise_threshold: f32,
    pub network: NetworkConfig,
    pub processing: ProcessingConfig,
}

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub ios_port: u16,
    pub blender_port: u16,
    pub buffer_size: usize,
}

#[derive(Debug, Deserialize)]
pub struct ProcessingConfig {
    pub threads: usize,
    pub batch_size: usize,
}

impl Config {
    pub fn load() -> Result<Self> {
        let builder = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::Environment::with_prefix("BRTRI"));

        builder
            .build()
            .map_err(|e| Error::Config(e.to_string()))?
            .try_deserialize()
            .map_err(|e| Error::Config(e.to_string()))
    }
}
