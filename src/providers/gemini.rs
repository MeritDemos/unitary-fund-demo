use async_trait::async_trait;
use std::error::Error;
use reqwest::Client;
use serde_json::{json, Value};

use super::Provider;

#[derive(Debug)]
pub struct GeminiProvider {
    client: Client,
    api_key: String,
}

impl GeminiProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            api_key: std::env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set"),
        }
    }
}

#[async_trait]
impl Provider for GeminiProvider {
    fn name(&self) -> &str {
        "Gemini"
    }

    async fn generate_text(
        &self,
        system_prompt: &str,
        user_input: &str,
        temperature: f32,
    ) -> Result<String, Box<dyn Error>> {
        let response = self.client
            .post("https://generativelanguage.googleapis.com/v1/models/gemini-2.0-flash:generateContent")
            .query(&[("key", &self.api_key)])
            .json(&json!({
                "contents": [{
                    "role": "user",
                    "parts": [{
                        "text": format!("{}\n\n{}", system_prompt, user_input)
                    }]
                }],
                "generationConfig": {
                    "temperature": temperature
                }
            }))
            .send()
            .await?
            .json::<Value>()
            .await?;

        Ok(response["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("Failed to generate text")
            .to_string())
    }
} 