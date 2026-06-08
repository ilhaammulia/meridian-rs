use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub struct Wallet {
    pub pubkey: Pubkey,
    client: RpcClient,
}

impl Wallet {
    pub fn new(rpc_url: &str, pubkey_str: &str) -> Result<Self> {
        let pubkey = Pubkey::from_str(pubkey_str)?;
        let client = RpcClient::new(rpc_url.to_string());

        Ok(Self { pubkey, client })
    }

    pub fn get_sol_balance(&self) -> Result<f64> {
        let balance = self.client.get_balance(&self.pubkey)?;
        Ok(balance as f64 / 1_000_000_000.0) // lamports to SOL
    }

    pub fn get_pubkey(&self) -> &Pubkey {
        &self.pubkey
    }
}