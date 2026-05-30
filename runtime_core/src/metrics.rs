
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct RuntimeMetrics {
    pub dispatch_count: u64,
    pub bytes_copied: u64,
    pub plugin_load_count: u64,
}

