use crate::llm::LlmClient;
use crate::tools::executor::ToolExecutor;
use crate::tools::screening::Screener;
use anyhow::Result;

/// ReAct-style Agent Loop with LLM + Screening
pub struct AgentLoop {
    pub max_iterations: u32,
    executor: ToolExecutor,
    llm: LlmClient,
    screener: Screener,
}

impl AgentLoop {
    pub fn new() -> Self {
        Self {
            max_iterations: 10,
            executor: ToolExecutor::new(),
            llm: LlmClient::new(),
            screener: Screener::new(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        println!("[Agent] Starting ReAct loop with LLM + Screener...");

        // Step 1: Screening
        let candidates = self.screener.get_top_candidates(3).await?;
        println!("[Agent] Top candidates found: {}", candidates.len());

        for candidate in &candidates {
            println!(
                "  - {} | TVL: ${:.0} | Score: {:.2}",
                candidate.token_symbol, candidate.tvl, candidate.score
            );
        }

        // Step 2: Ask LLM what to do with screening result
        let prompt = format!(
            "Here are top pool candidates: {:?}. Which one should I deploy to? Reply with token symbol only.",
            candidates.iter().map(|c| &c.token_symbol).collect::<Vec<_>>()
        );

        let llm_response = self.llm.chat("gpt-4o-mini", &prompt).await?;
        println!("[Agent] LLM recommendation: {}", llm_response);

        // Step 3: Execute tool
        let tool_result = self.executor.execute("get_balance", "")?;
        println!("[Agent] Current balance: {}", tool_result);

        println!("[Agent] ReAct cycle completed.");
        Ok(())
    }
}