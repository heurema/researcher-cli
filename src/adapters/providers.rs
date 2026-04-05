use anyhow::Result;
use async_trait::async_trait;
use tokio::process::Command;
use std::process::Stdio;

#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &str;
    async fn query(&self, prompt: &str) -> Result<String>;
}

pub struct ClaudeProvider;
pub struct GeminiProvider;
pub struct CodexProvider;

#[async_trait]
impl Provider for ClaudeProvider {
    fn name(&self) -> &str { "claude" }
    async fn query(&self, prompt: &str) -> Result<String> {
        let output = Command::new("claude")
            // Sanitize environment to ensure subscription auth is used if present
            .env_remove("ANTHROPIC_API_KEY")
            .env_remove("CLAUDECODE")
            .arg("-p")
            .arg(prompt)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()
            .await?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let err = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Claude CLI error: {}", err)
        }
    }
}

#[async_trait]
impl Provider for GeminiProvider {
    fn name(&self) -> &str { "gemini" }
    async fn query(&self, prompt: &str) -> Result<String> {
        let output = Command::new("gemini")
            .arg("-p")
            .arg(prompt)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()
            .await?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let err = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Gemini CLI error: {}", err)
        }
    }
}

#[async_trait]
impl Provider for CodexProvider {
    fn name(&self) -> &str { "codex" }
    async fn query(&self, prompt: &str) -> Result<String> {
        let output = Command::new("codex")
            .arg("exec")
            .arg(prompt)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()
            .await?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let err = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Codex CLI error: {}", err)
        }
    }
}
