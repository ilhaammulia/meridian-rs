#![allow(dead_code)]

use anyhow::Result;

mod config;
mod tools;
mod utils;
mod state;
mod agent;
mod web;
mod llm;
mod cycle;

use config::load_config;
use state::positions::PositionState;
use tools::dlmm::DlmmClient;
use tools::wallet::Wallet;
use utils::logger::info;
use agent::loop_::AgentLoop;
use cycle::{run_management_cycle, run_screening_cycle};

#[tokio::main]
async fn main() -> Result<()> {
    info("🚀 Meridian RS - DLMM Liquidity Provider Agent");
    info("Version: 0.1.0 (Rust Rewrite)");
    info("----------------------------------------");

    let _config = load_config(None)?;
    info("Phase 1: Config loaded");

    let rpc_url = "https://api.mainnet-beta.solana.com";
    let example_pubkey = "11111111111111111111111111111111";

    if let Ok(_wallet) = Wallet::new(rpc_url, example_pubkey) {
        info("Wallet initialized");
    }

    let _dlmm = DlmmClient::new(rpc_url);
    info("DLMM client ready");

    let _position_state = PositionState::load("state.json")?;
    info("State management ready");

    info("Telegram disabled - Web UI (HyperOS style) prepared");

    let _agent = AgentLoop::new();
    info("Agent loop ready");

    // Run cycles
    run_management_cycle()?;
    run_screening_cycle()?;

    info("Starting web server...");
    web::start_web_server().await?;

    Ok(())
}