use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedPosition {
    pub id: String,
    pub pool_address: String,
    pub base_mint: String,
    pub lower_bin: i32,
    pub upper_bin: i32,
    pub amount_sol: f64,
    pub status: String,
    pub created_at: String,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PositionState {
    pub positions: HashMap<String, TrackedPosition>,
}

impl PositionState {
    pub fn load(path: &str) -> Result<Self> {
        if Path::new(path).exists() {
            let content = fs::read_to_string(path)?;
            let state: PositionState = serde_json::from_str(&content)?;
            Ok(state)
        } else {
            Ok(PositionState::default())
        }
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn add_position(&mut self, position: TrackedPosition) {
        self.positions.insert(position.id.clone(), position);
    }

    pub fn remove_position(&mut self, id: &str) {
        self.positions.remove(id);
    }

    pub fn get_active_positions(&self) -> Vec<&TrackedPosition> {
        self.positions
            .values()
            .filter(|p| p.status == "active")
            .collect()
    }
}