use pcl::{filters, PointCloud, PointXYZ};
use crate::processing::point_cloud::ProcessedCloud;

pub struct CloudFilter {
    statistical_outlier_removal: filters::StatisticalOutlierRemoval,
    radius_outlier_removal: filters::RadiusOutlierRemoval,
}

impl CloudFilter {
    pub fn new() -> Self {
        Self {
            statistical_outlier_removal: filters::StatisticalOutlierRemoval::new(2.0, 50),
            radius_outlier_removal: filters::RadiusOutlierRemoval::new(0.1, 5),
        }
    }

    #[inline(always)]
    pub fn apply(&self, cloud: &ProcessedCloud) -> ProcessedCloud {
        // Apply filters in sequence
        cloud.clone()
    }
}
