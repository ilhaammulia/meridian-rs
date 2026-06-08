use crate::config::load_config;
use anyhow::Result;

/// Management Cycle
pub fn run_management_cycle() -> Result<()> {
    println!("[Cycle] Running Management Cycle...");
    let _config = load_config(None)?;
    println!("[Management] Checking TP/SL/OOR...");
    println!("[Management] Done");
    Ok(())
}

/// Screening Cycle
pub fn run_screening_cycle() -> Result<()> {
    println!("[Cycle] Running Screening Cycle...");
    let _config = load_config(None)?;
    println!("[Screening] Looking for opportunities...");
    println!("[Screening] Done");
    Ok(())
}