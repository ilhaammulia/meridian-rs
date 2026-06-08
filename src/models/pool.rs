use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolInfo {
    pub address: String,
    pub token_x: String,
    pub token_y: String,
    pub bin_step: u16,
    pub tvl: f64,
    pub volume_24h: f64,
    pub fee_tvl_ratio: f64,
}