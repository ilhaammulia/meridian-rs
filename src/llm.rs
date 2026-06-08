use anyhow::Result;
use serde::{Deserialize, Serialize};
use reqwest::Client;

const DEFAULT_BASE_URL: &str = "https://token-plan-sgp.xiaomimimo.com/v1";
const DEFAULT_API_KEY: &str = "tp-stv2eijlsstb5za06ysxr3f4mrgcq92nbzl2grvmipzm7ztk";

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
}

pub struct LlmClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl LlmClient {
    /// Create LLM client with default Xiaomi Mimo endpoint
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: DEFAULT_API_KEY.to_string(),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    pub fn with_credentials(api_key: &str, base_url: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn chat(&self, model: &str, prompt: &str) -> Result<String> {
        let request = ChatRequest {
            model: model.to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            temperature: 0.7,
            max_tokens: 1000,
        };

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!("LLM API error {}: {}", status, text));
        }

        let chat_response: ChatResponse = response.json().await?;
        
        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Ok("No response from LLM".to_string())
        }
    }
}