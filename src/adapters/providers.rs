use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::env;

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    async fn query(&self, prompt: &str) -> Result<String>;
}

pub struct ClaudeProvider {
    client: reqwest::Client,
    api_key: String,
}

pub struct GeminiProvider {
    client: reqwest::Client,
    api_key: String,
}

impl ClaudeProvider {
    pub fn new() -> Result<Self> {
        let api_key = env::var("ANTHROPIC_API_KEY")?;
        Ok(Self {
            client: reqwest::Client::new(),
            api_key,
        })
    }
}

impl GeminiProvider {
    pub fn new() -> Result<Self> {
        let api_key = env::var("GOOGLE_API_KEY")?;
        Ok(Self {
            client: reqwest::Client::new(),
            api_key,
        })
    }
}

#[async_trait]
impl Provider for ClaudeProvider {
    fn name(&self) -> &str { "claude" }
    async fn query(&self, prompt: &str) -> Result<String> {
        let resp = self.client.post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&serde_json::json!({
                "model": "claude-3-5-sonnet-latest",
                "max_tokens": 1024,
                "messages": [{"role": "user", "content": prompt}]
            }))
            .send()
            .await?;
        
        let body: serde_json::Value = resp.json().await?;
        // Simplistic extraction for MVP
        let text = body["content"][0]["text"].as_str().unwrap_or("Error parsing Claude response").to_string();
        Ok(text)
    }
}

#[async_trait]
impl Provider for GeminiProvider {
    fn name(&self) -> &str { "gemini" }
    async fn query(&self, prompt: &str) -> Result<String> {
        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", self.api_key);
        let resp = self.client.post(&url)
            .json(&serde_json::json!({
                "contents": [{"parts": [{"text": prompt}]}]
            }))
            .send()
            .await?;
        
        let body: serde_json::Value = resp.json().await?;
        let text = body["candidates"][0]["content"]["parts"][0]["text"].as_str().unwrap_or("Error parsing Gemini response").to_string();
        Ok(text)
    }
}
