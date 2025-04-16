use async_trait::async_trait;
use std::error::Error;
use reqwest::Client;
use serde_json::{json, Value};

use super::Provider;

#[derive(Debug)]
pub struct OpenAIProvider {
    client: Client,
    api_key: String,
}

impl OpenAIProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
        }
    }
}

#[async_trait]
impl Provider for OpenAIProvider {
    fn name(&self) -> &str {
        "OpenAI"
    }

    async fn generate_text(
        &self,
        system_prompt: &str,
        user_input: &str,
        temperature: f32,
    ) -> Result<String, Box<dyn Error>> {
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": "gpt-4-turbo-preview",
                "messages": [
                    {
                        "role": "system",
                        "content": system_prompt
                    },
                    {
                        "role": "user",
                        "content": user_input
                    }
                ],
                "temperature": temperature
            }))
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("Failed to generate text")
            .to_string())
    }
} 