use async_trait::async_trait;
use std::error::Error;
use reqwest::Client;
use serde_json::{json, Value};

use super::Provider;

#[derive(Debug)]
pub struct ClaudeProvider {
    client: Client,
    api_key: String,
}

impl ClaudeProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: std::env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set"),
        }
    }
}

#[async_trait]
impl Provider for ClaudeProvider {
    fn name(&self) -> &str {
        "Claude"
    }

    async fn generate_text(
        &self,
        system_prompt: &str,
        user_input: &str,
        temperature: f32,
    ) -> Result<String, Box<dyn Error>> {
        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": "claude-3-5-haiku-latest",
                "messages": [
                    {
                        "role": "user",
                        "content": user_input
                    }
                ],
                "temperature": temperature,
                "system": system_prompt,
                "max_tokens": 8192
            }))
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response["content"][0]["text"]
            .as_str()
            .unwrap_or("Failed to generate text")
            .to_string())
    }
} 