use async_trait::async_trait;
use std::error::Error;
use std::fmt::Debug;
use dialoguer::{theme::ColorfulTheme, Select};

pub mod openai;
pub mod claude;
pub mod gemini;
pub mod deepseek;

pub use openai::OpenAIProvider;
pub use claude::ClaudeProvider;
pub use gemini::GeminiProvider;
pub use deepseek::DeepSeekProvider;

/// Base trait for AI model providers with general capabilities
#[async_trait]
pub trait Provider: Send + Sync + Debug {
    fn name(&self) -> &str;
    
    /// Generate text based on a system prompt and user input
    async fn generate_text(
        &self,
        system_prompt: &str,
        user_input: &str,
        temperature: f32,
    ) -> Result<String, Box<dyn Error>>;
}

/// Error type for provider selection
#[derive(Debug)]
pub enum ProviderError {
    NoProvidersAvailable,
    InvalidSelection,
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoProvidersAvailable => write!(f, "No AI providers available"),
            Self::InvalidSelection => write!(f, "Invalid provider selection"),
        }
    }
}

impl Error for ProviderError {}

/// Get all available providers based on environment variables
pub fn get_available_providers() -> Vec<Box<dyn Provider>> {
    use std::env;

    let mut providers = Vec::new();
    
    if env::var("OPENAI_API_KEY").is_ok() {
        providers.push(Box::new(OpenAIProvider::new()) as Box<dyn Provider>);
    }
    
    if env::var("ANTHROPIC_API_KEY").is_ok() {
        providers.push(Box::new(ClaudeProvider::new()) as Box<dyn Provider>);
    }
    
    if env::var("DEEPSEEK_API_KEY").is_ok() {
        providers.push(Box::new(DeepSeekProvider::new()) as Box<dyn Provider>);
    }
    
    if env::var("GEMINI_API_KEY").is_ok() {
        providers.push(Box::new(GeminiProvider::new()) as Box<dyn Provider>);
    }

    if providers.is_empty() {
        eprintln!("No AI providers found. Please set at least one API key:");
        eprintln!("  OPENAI_API_KEY for OpenAI");
        eprintln!("  ANTHROPIC_API_KEY for Claude");
        eprintln!("  DEEPSEEK_API_KEY for DeepSeek");
        eprintln!("  GEMINI_API_KEY for Google");
        std::process::exit(1);
    }
    
    providers
}

/// Select a provider from the available ones
pub fn select_provider(providers: &[Box<dyn Provider>]) -> Result<usize, Box<dyn Error>> {
    if providers.is_empty() {
        return Err(Box::new(ProviderError::NoProvidersAvailable));
    }

    let provider_names: Vec<String> = providers
        .iter()
        .map(|p| p.name().to_string())
        .collect();

    Ok(Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an AI model provider")
        .items(&provider_names)
        .default(0)
        .interact()?)
} 