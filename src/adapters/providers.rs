use anyhow::Result;

pub trait Provider {
    fn name(&self) -> &str;
    fn query(&self, prompt: &str) -> Result<String>;
}

pub struct ClaudeProvider;
pub struct GeminiProvider;
pub struct CodexProvider;

impl Provider for ClaudeProvider {
    fn name(&self) -> &str { "claude" }
    fn query(&self, _prompt: &str) -> Result<String> {
        Ok("Claude response mock".to_string())
    }
}

impl Provider for GeminiProvider {
    fn name(&self) -> &str { "gemini" }
    fn query(&self, _prompt: &str) -> Result<String> {
        Ok("Gemini response mock".to_string())
    }
}

impl Provider for CodexProvider {
    fn name(&self) -> &str { "codex" }
    fn query(&self, _prompt: &str) -> Result<String> {
        Ok("Codex response mock".to_string())
    }
}
