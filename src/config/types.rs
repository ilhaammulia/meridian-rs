use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub screening: ScreeningConfig,
    pub management: ManagementConfig,
    pub risk: RiskConfig,
    pub schedule: ScheduleConfig,
    pub llm: LlmConfig,
    pub dual_strategy: DualStrategyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreeningConfig {
    pub min_fee_active_tvl_ratio: f64,
    pub min_tvl: f64,
    pub max_tvl: f64,
    pub min_volume: f64,
    pub min_organic: f64,
    pub min_holders: u32,
    pub min_mcap: f64,
    pub max_mcap: f64,
    pub min_bin_step: u16,
    pub max_bin_step: u16,
    pub timeframe: String,
    pub category: String,
    pub min_token_fees_sol: f64,
    pub max_bundlers_pct: f64,
    pub max_top10_pct: f64,
    pub blocked_launchpads: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagementConfig {
    pub deploy_amount_sol: f64,
    pub gas_reserve: f64,
    pub position_size_pct: f64,
    pub min_sol_to_open: f64,
    pub out_of_range_wait_minutes: u32,
    pub take_profit_pct: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskConfig {
    pub max_deploy_amount: f64,
    pub max_positions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleConfig {
    pub management_interval_min: u32,
    pub screening_interval_min: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmConfig {
    pub management_model: String,
    pub screening_model: String,
    pub general_model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DualStrategyConfig {
    pub enabled: bool,
    pub primary_pct: f64,
    pub safeguard_oor_wait_min: u32,
    pub aggressive_oor_wait_min: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            screening: ScreeningConfig {
                min_fee_active_tvl_ratio: 0.05,
                min_tvl: 10_000.0,
                max_tvl: 150_000.0,
                min_volume: 500.0,
                min_organic: 60.0,
                min_holders: 500,
                min_mcap: 150_000.0,
                max_mcap: 10_000_000.0,
                min_bin_step: 80,
                max_bin_step: 125,
                timeframe: "5m".to_string(),
                category: "trending".to_string(),
                min_token_fees_sol: 30.0,
                max_bundlers_pct: 30.0,
                max_top10_pct: 60.0,
                blocked_launchpads: vec![],
            },
            management: ManagementConfig {
                deploy_amount_sol: 0.5,
                gas_reserve: 0.2,
                position_size_pct: 0.35,
                min_sol_to_open: 0.55,
                out_of_range_wait_minutes: 30,
                take_profit_pct: None,
            },
            risk: RiskConfig {
                max_deploy_amount: 50.0,
                max_positions: 3,
            },
            schedule: ScheduleConfig {
                management_interval_min: 10,
                screening_interval_min: 30,
            },
            llm: LlmConfig {
                management_model: "gpt-4o-mini".to_string(),
                screening_model: "gpt-4o-mini".to_string(),
                general_model: "gpt-4o-mini".to_string(),
            },
            dual_strategy: DualStrategyConfig {
                enabled: false,
                primary_pct: 0.6,
                safeguard_oor_wait_min: 60,
                aggressive_oor_wait_min: 15,
            },
        }
    }
}