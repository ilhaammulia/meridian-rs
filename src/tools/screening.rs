use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolCandidate {
    pub address: String,
    pub token_symbol: String,
    pub tvl: f64,
    pub volume_24h: f64,
    pub fee_tvl_ratio: f64,
    pub bin_step: u16,
    pub score: f64,
}

pub struct Screener {
    min_tvl: f64,
    min_volume: f64,
    min_fee_tvl_ratio: f64,
}

impl Screener {
    pub fn new() -> Self {
        Self {
            min_tvl: 10_000.0,
            min_volume: 500.0,
            min_fee_tvl_ratio: 0.05,
        }
    }

    /// Simulate screening (replace with real Meteora API later)
    pub async fn get_top_candidates(&self, limit: usize) -> Result<Vec<PoolCandidate>> {
        println!("[Screener] Fetching top candidates (stub)...");

        // Stub data - in real implementation this would call Meteora API
        let candidates = vec![
            PoolCandidate {
                address: "pool_abc123".to_string(),
                token_symbol: "MEME".to_string(),
                tvl: 85_000.0,
                volume_24h: 125_000.0,
                fee_tvl_ratio: 0.12,
                bin_step: 100,
                score: 0.87,
            },
            PoolCandidate {
                address: "pool_def456".to_string(),
                token_symbol: "CAT".to_string(),
                tvl: 42_000.0,
                volume_24h: 78_000.0,
                fee_tvl_ratio: 0.09,
                bin_step: 80,
                score: 0.71,
            },
        ];

        let mut filtered: Vec<_> = candidates
            .into_iter()
            .filter(|p| {
                p.tvl >= self.min_tvl
                    && p.volume_24h >= self.min_volume
                    && p.fee_tvl_ratio >= self.min_fee_tvl_ratio
            })
            .collect();

        filtered.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        filtered.truncate(limit);

        Ok(filtered)
    }
}