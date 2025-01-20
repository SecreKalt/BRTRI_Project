pub mod config;
pub mod error;
pub mod processing;
pub mod utils;

use processing::{CloudFilter, CloudOptimizer, ProcessedCloud};
use error::Result;

pub struct BrtriProcessor {
    filter: CloudFilter,
    optimizer: CloudOptimizer,
}

impl BrtriProcessor {
    pub fn new(filter: CloudFilter, optimizer: CloudOptimizer) -> Self {
        Self { filter, optimizer }
    }

    pub fn process_cloud(&self, cloud: ProcessedCloud) -> Result<ProcessedCloud> {
        let filtered = self.filter.apply(&cloud);
        Ok(self.optimizer.optimize(filtered))
    }
}

pub use config::Config;
pub use error::{Error, Result as BrtriResult};
