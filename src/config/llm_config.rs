use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmSettings {
    pub base_url: String,
    pub api_key: String,
    pub default_model: String,
}

impl Default for LlmSettings {
    fn default() -> Self {
        Self {
            base_url: "https://token-plan-sgp.xiaomimimo.com/v1".to_string(),
            api_key: "tp-stv2eijlsstb5za06ysxr3f4mrgcq92nbzl2grvmipzm7ztk".to_string(),
            default_model: "gpt-4o-mini".to_string(),
        }
    }
}