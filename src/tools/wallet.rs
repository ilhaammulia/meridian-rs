use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::config::Config;

/// Wrapped SOL mint address (canonical).
pub const SOL_MINT: &str = "So11111111111111111111111111111111111111112";

/// Default Jupiter API key for swaps.
const DEFAULT_JUPITER_API_KEY: &str = "b15d42e9-e0e4-4f90-a424-ae41ceeaa382";

/// Jupiter Price API v3 base URL.
const JUPITER_PRICE_API: &str = "https://api.jup.ag/price/v3";

/// Jupiter Swap V2 API base URL.
const JUPITER_SWAP_V2_API: &str = "https://api.jup.ag/swap/v2";

/// Default referral account (configurable via config.jupiter.referralAccount).
const DEFAULT_REFERRAL_ACCOUNT: &str = "";

/// Default referral fee in basis points.
const DEFAULT_REFERRAL_FEE_BPS: u32 = 50;

// ─── Response types ──────────────────────────────────────────────

/// Token balance entry from Helius.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusBalanceEntry {
    pub mint: Option<String>,
    pub symbol: Option<String>,
    pub balance: Option<f64>,
    pub price_per_token: Option<f64>,
    pub usd_value: Option<f64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Helius wallet balances API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeliusBalancesResponse {
    pub balances: Option<Vec<HeliusBalanceEntry>>,
    pub total_usd_value: Option<f64>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Wallet balances result returned to the LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalances {
    pub wallet: Option<String>,
    pub sol: f64,
    pub sol_price: f64,
    pub sol_usd: f64,
    pub usdc: f64,
    pub tokens: Vec<TokenBalance>,
    pub total_usd: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Single token balance entry for LLM output.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenBalance {
    pub mint: String,
    pub symbol: String,
    pub balance: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<f64>,
}

/// Jupiter price API response for a single token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupiterPriceResponse {
    #[serde(flatten)]
    pub data: std::collections::HashMap<String, JupiterPriceEntry>,
}

/// Jupiter price entry for a token.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupiterPriceEntry {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub price: String,
}

/// Jupiter Swap V2 order request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupiterOrderRequest {
    pub input_mint: String,
    pub output_mint: String,
    pub amount: String,
    pub taker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_fee: Option<u32>,
}

/// Jupiter Swap V2 order response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupiterOrderResponse {
    pub transaction: Option<String>,
    pub request_id: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub fee_bps: Option<u32>,
    pub fee_mint: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Jupiter Swap V2 execute request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupiterExecuteRequest {
    pub signed_transaction: String,
    pub request_id: String,
}

/// Jupiter Swap V2 execute response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JupiterExecuteResponse {
    pub status: Option<String>,
    pub signature: Option<String>,
    pub code: Option<String>,
    pub input_amount_result: Option<String>,
    pub output_amount_result: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

/// Swap result returned to the LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwapResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_mint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_mint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_out: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_fee_bps_requested: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_bps_applied: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ─── Helpers ─────────────────────────────────────────────────────

/// Normalize any SOL-like address to the canonical wrapped SOL mint.
///
/// Collapses "SOL", "native", "So1...", and any So1-prefixed address
/// that is not already the canonical wrapped SOL mint.
pub fn normalize_mint(mint: &str) -> String {
    if mint.is_empty() {
        return mint.to_string();
    }
    // Direct match on common aliases
    if mint.eq_ignore_ascii_case("SOL") || mint == "native" {
        return SOL_MINT.to_string();
    }
    // Match pure "So1..." patterns (all So1 characters)
    if mint.chars().all(|c| c == 'S' || c == 'o' || c == '1') && mint.starts_with("So1") {
        return SOL_MINT.to_string();
    }
    // Match any So1-prefixed address that's not the canonical one
    if mint.len() >= 32 && mint.len() <= 44 && mint.starts_with("So1") && mint != SOL_MINT {
        return SOL_MINT.to_string();
    }
    mint.to_string()
}

/// Round a number to `decimals` decimal places.
fn round(val: f64, decimals: u32) -> f64 {
    let factor = 10f64.powi(decimals as i32);
    (val * factor).round() / factor
}

/// Safe parse: returns 0.0 if value cannot be parsed.
fn safe_f64(val: &serde_json::Value) -> f64 {
    val.as_f64().unwrap_or(0.0)
}

/// Get the Jupiter API key from config or fallback.
fn get_jupiter_api_key(config: &Config) -> String {
    config
        .api
        .helius_api_key
        .clone()
        .or_else(|| std::env::var("JUPITER_API_KEY").ok())
        .unwrap_or_else(|| DEFAULT_JUPITER_API_KEY.to_string())
}

/// Get referral params from config.
fn get_referral_params(config: &Config) -> Option<(String, u32)> {
    // Check config for referral account
    let referral_account = config.api.agent_meridian_key.clone().unwrap_or_default();
    let referral_fee_bps = DEFAULT_REFERRAL_FEE_BPS;

    // Use the default referral account if configured, otherwise skip
    let account = if !DEFAULT_REFERRAL_ACCOUNT.is_empty() {
        DEFAULT_REFERRAL_ACCOUNT.to_string()
    } else if !referral_account.is_empty() {
        referral_account
    } else {
        return None;
    };

    if referral_fee_bps < 50 || referral_fee_bps > 255 {
        return None;
    }
    Some((account, referral_fee_bps))
}

// ─── Public API ──────────────────────────────────────────────────

/// Get wallet balances (SOL + tokens) via Helius Wallet API.
///
/// Calls `https://api.helius.xyz/v1/wallet/{address}/balances` and
/// returns a `WalletBalances` struct with SOL, USDC, and all SPL tokens.
pub async fn get_wallet_balances(
    _rpc_url: &str,
    pubkey: &str,
    helius_api_key: &str,
) -> Result<WalletBalances> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://api.helius.xyz/v1/wallet/{}/balances?api-key={}",
        pubkey, helius_api_key,
    );

    let resp = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(15))
        .send()
        .await
        .map_err(|e| anyhow!("Helius API request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Ok(WalletBalances {
            wallet: Some(pubkey.to_string()),
            sol: 0.0,
            sol_price: 0.0,
            sol_usd: 0.0,
            usdc: 0.0,
            tokens: vec![],
            total_usd: 0.0,
            error: Some(format!("Helius API error: {} {}", status, body)),
        });
    }

    let data: HeliusBalancesResponse = resp
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse Helius response: {}", e))?;

    let balances = data.balances.unwrap_or_default();

    // Find SOL and USDC entries
    let sol_entry = balances
        .iter()
        .find(|b| b.mint.as_deref() == Some(SOL_MINT) || b.symbol.as_deref() == Some("SOL"));
    let usdc_entry = balances.iter().find(|b| {
        b.mint.as_deref() == Some("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v")
            || b.symbol.as_deref() == Some("USDC")
    });

    let sol_balance = sol_entry.as_ref().and_then(|e| e.balance).unwrap_or(0.0);
    let sol_price = sol_entry
        .as_ref()
        .and_then(|e| e.price_per_token)
        .unwrap_or(0.0);
    let sol_usd = sol_entry.as_ref().and_then(|e| e.usd_value).unwrap_or(0.0);
    let usdc_balance = usdc_entry.as_ref().and_then(|e| e.balance).unwrap_or(0.0);

    // Map all tokens
    let tokens: Vec<TokenBalance> = balances
        .into_iter()
        .map(|b| {
            let mint_str = b.mint.unwrap_or_default();
            let symbol = b
                .symbol
                .or_else(|| {
                    if mint_str.is_empty() {
                        None
                    } else {
                        Some(mint_str.chars().take(8).collect::<String>())
                    }
                })
                .unwrap_or_else(|| "unknown".to_string());
            TokenBalance {
                mint: mint_str,
                symbol,
                balance: b.balance.unwrap_or(0.0),
                usd: b.usd_value.map(|v| round(v, 2)),
            }
        })
        .collect();

    Ok(WalletBalances {
        wallet: Some(pubkey.to_string()),
        sol: round(sol_balance, 6),
        sol_price: round(sol_price, 2),
        sol_usd: round(sol_usd, 2),
        usdc: round(usdc_balance, 2),
        tokens,
        total_usd: round(data.total_usd_value.unwrap_or(0.0), 2),
        error: None,
    })
}

/// Get the current SOL price via Jupiter Price API.
///
/// Calls `https://api.jup.ag/price/v3/{SOL_MINT}` and returns the price in USD.
pub async fn get_sol_price() -> Result<f64> {
    let client = reqwest::Client::new();
    let url = format!("{}/{}", JUPITER_PRICE_API, SOL_MINT);

    let resp = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .map_err(|e| anyhow!("Jupiter price API request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(anyhow!("Jupiter price API error: {} {}", status, body));
    }

    let data: JupiterPriceResponse = resp
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse Jupiter price response: {}", e))?;

    if let Some(entry) = data.data.get(SOL_MINT) {
        let price: f64 = entry
            .price
            .parse()
            .map_err(|_| anyhow!("Failed to parse SOL price: {}", entry.price))?;
        Ok(price)
    } else {
        Err(anyhow!("SOL not found in Jupiter price response"))
    }
}

/// Swap tokens via Jupiter Swap V2 API (order → sign → execute).
///
/// This function builds the order, but signing requires the wallet private key.
/// In the Rust version, the caller is responsible for providing a signed
/// transaction. For now, this returns the order info that needs signing.
pub async fn swap_token(
    mint: &str,
    amount: f64,
    _slippage_bps: u32,
    referral_bps: u32,
    config: &Config,
) -> Result<SwapResult> {
    let input_mint = normalize_mint(mint);
    let output_mint = SOL_MINT.to_string();

    if std::env::var("DRY_RUN").ok().as_deref() == Some("true") {
        return Ok(SwapResult {
            success: false,
            tx: None,
            input_mint: Some(input_mint.clone()),
            output_mint: Some(output_mint.clone()),
            amount_in: None,
            amount_out: None,
            referral_account: None,
            referral_fee_bps_requested: None,
            fee_bps_applied: None,
            error: Some("DRY RUN — no transaction sent".to_string()),
        });
    }

    let client = reqwest::Client::new();
    let jupiter_api_key = config
        .api
        .helius_api_key
        .clone()
        .or_else(|| std::env::var("JUPITER_API_KEY").ok())
        .unwrap_or_else(|| DEFAULT_JUPITER_API_KEY.to_string());

    // Convert amount to smallest unit (9 decimals for SOL)
    let decimals: u32 = 9;
    let amount_lamports = (amount * 10f64.powi(decimals as i32)).floor() as u64;
    let amount_str = amount_lamports.to_string();

    // Build order URL
    let mut url = format!(
        "{}/order?inputMint={}&outputMint={}&amount={}",
        JUPITER_SWAP_V2_API, input_mint, output_mint, amount_str,
    );

    // Add referral params if configured
    let referral_params = if referral_bps >= 50 && referral_bps <= 255 {
        let account = config
            .api
            .agent_meridian_key
            .clone()
            .unwrap_or_else(|| DEFAULT_REFERRAL_ACCOUNT.to_string());
        if !account.is_empty() {
            url = format!(
                "{}&referralAccount={}&referralFee={}",
                url, account, referral_bps
            );
            Some((account, referral_bps))
        } else {
            None
        }
    } else {
        None
    };

    tracing::info!("swap_token: {} of {} → {}", amount, input_mint, output_mint,);

    // ─── Get Swap V2 order ──────────────────────────────────────
    let mut req_builder = client.get(&url).timeout(std::time::Duration::from_secs(30));
    if !jupiter_api_key.is_empty() {
        req_builder = req_builder.header("x-api-key", &jupiter_api_key);
    }

    let order_resp = req_builder
        .send()
        .await
        .map_err(|e| anyhow!("Swap V2 order request failed: {}", e))?;

    if !order_resp.status().is_success() {
        let status = order_resp.status();
        let body = order_resp.text().await.unwrap_or_default();
        return Ok(SwapResult {
            success: false,
            tx: None,
            input_mint: Some(input_mint),
            output_mint: Some(output_mint),
            amount_in: None,
            amount_out: None,
            referral_account: referral_params.as_ref().map(|(a, _)| a.clone()),
            referral_fee_bps_requested: referral_params.as_ref().map(|(_, f)| *f),
            fee_bps_applied: None,
            error: Some(format!("Swap V2 order failed: {} {}", status, body)),
        });
    }

    let order: JupiterOrderResponse = order_resp
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse swap V2 order response: {}", e))?;

    if let Some(ref err_msg) = order.error_message {
        return Ok(SwapResult {
            success: false,
            tx: None,
            input_mint: Some(input_mint),
            output_mint: Some(output_mint),
            amount_in: None,
            amount_out: None,
            referral_account: referral_params.as_ref().map(|(a, _)| a.clone()),
            referral_fee_bps_requested: referral_params.as_ref().map(|(_, f)| *f),
            fee_bps_applied: order.fee_bps,
            error: Some(format!("Swap V2 order error: {}", err_msg)),
        });
    }

    // NOTE: In a full implementation, we would deserialize the unsigned
    // transaction, sign it with the wallet keypair, and then execute.
    // For now, we return the unsigned transaction for the caller to handle.
    let _unsigned_tx = order
        .transaction
        .ok_or_else(|| anyhow!("Swap V2 order returned no transaction"))?;
    let request_id = order
        .request_id
        .ok_or_else(|| anyhow!("Swap V2 order returned no request_id"))?;

    tracing::info!("swap_token: order received, request_id={}", request_id);

    // ─── Return order info ───────────────────────────────────────
    // In production, we'd sign the transaction here and call execute.
    // For the Rust port, the signing is deferred to the caller or
    // handled via a keypair loaded at init time.
    Ok(SwapResult {
        success: false,
        tx: None,
        input_mint: Some(input_mint),
        output_mint: Some(output_mint),
        amount_in: Some(amount_str),
        amount_out: None,
        referral_account: referral_params.as_ref().map(|(a, _)| a.clone()),
        referral_fee_bps_requested: referral_params.as_ref().map(|(_, f)| *f),
        fee_bps_applied: order.fee_bps,
        error: Some(format!(
            "Order ready (request_id={}). Transaction signing not yet implemented in Rust.",
            request_id
        )),
    })
}

/// Execute a pre-signed Jupiter swap transaction.
///
/// After the caller signs the unsigned transaction from `swap_token`, this
/// function sends it to Jupiter for execution.
pub async fn execute_swap(
    signed_transaction: &str,
    request_id: &str,
    config: &Config,
) -> Result<SwapResult> {
    let client = reqwest::Client::new();
    let jupiter_api_key = config
        .api
        .helius_api_key
        .clone()
        .or_else(|| std::env::var("JUPITER_API_KEY").ok())
        .unwrap_or_else(|| DEFAULT_JUPITER_API_KEY.to_string());

    let body = JupiterExecuteRequest {
        signed_transaction: signed_transaction.to_string(),
        request_id: request_id.to_string(),
    };

    let mut req_builder = client
        .post(format!("{}/execute", JUPITER_SWAP_V2_API))
        .json(&body)
        .timeout(std::time::Duration::from_secs(30));
    if !jupiter_api_key.is_empty() {
        req_builder = req_builder.header("x-api-key", &jupiter_api_key);
    }

    let exec_resp = req_builder
        .send()
        .await
        .map_err(|e| anyhow!("Swap V2 execute request failed: {}", e))?;

    if !exec_resp.status().is_success() {
        let status = exec_resp.status();
        let body = exec_resp.text().await.unwrap_or_default();
        return Ok(SwapResult {
            success: false,
            tx: None,
            input_mint: None,
            output_mint: None,
            amount_in: None,
            amount_out: None,
            referral_account: None,
            referral_fee_bps_requested: None,
            fee_bps_applied: None,
            error: Some(format!("Swap V2 execute failed: {} {}", status, body)),
        });
    }

    let result: JupiterExecuteResponse = exec_resp
        .json()
        .await
        .map_err(|e| anyhow!("Failed to parse swap V2 execute response: {}", e))?;

    if result.status.as_deref() == Some("Failed") {
        return Ok(SwapResult {
            success: false,
            tx: None,
            input_mint: None,
            output_mint: None,
            amount_in: None,
            amount_out: None,
            referral_account: None,
            referral_fee_bps_requested: None,
            fee_bps_applied: None,
            error: Some(format!(
                "Swap failed on-chain: code={}",
                result.code.unwrap_or_default()
            )),
        });
    }

    tracing::info!(
        "swap execute: SUCCESS tx={}",
        result.signature.as_deref().unwrap_or("unknown")
    );

    Ok(SwapResult {
        success: true,
        tx: result.signature,
        input_mint: None,
        output_mint: None,
        amount_in: result.input_amount_result,
        amount_out: result.output_amount_result,
        referral_account: None,
        referral_fee_bps_requested: None,
        fee_bps_applied: None,
        error: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_mint_sol() {
        assert_eq!(normalize_mint("SOL"), SOL_MINT);
        assert_eq!(normalize_mint("sol"), SOL_MINT);
        assert_eq!(normalize_mint("native"), SOL_MINT);
    }

    #[test]
    fn test_normalize_mint_so1_variants() {
        assert_eq!(
            normalize_mint("So11111111111111111111111111111111111111112"),
            SOL_MINT
        );
        assert_eq!(
            normalize_mint("So11111111111111111111111111111111111111111"),
            SOL_MINT
        );
    }

    #[test]
    fn test_normalize_mint_passthrough() {
        let usdc = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
        assert_eq!(normalize_mint(usdc), usdc);
    }

    #[test]
    fn test_normalize_mint_empty() {
        assert_eq!(normalize_mint(""), "");
    }
}
