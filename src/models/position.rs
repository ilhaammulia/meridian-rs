use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: String,
    pub pool_address: String,
    pub base_mint: String,
    pub quote_mint: String,
    pub bin_step: u16,
    pub lower_bin: i32,
    pub upper_bin: i32,
    pub amount_sol: f64,
    pub status: String,
    pub created_at: String,
}