mod contract;
mod core;
mod adapters;

use clap::{Parser, Subcommand};
use crate::contract::schema::{Envelope, ResearchResponse, Claim, VerificationStatus, ResearchDepth};
use crate::adapters::providers::{ClaudeProvider, GeminiProvider, CodexProvider, Provider};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about = "ResearcherCLI: Universal AI CLI Orchestrator")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Execute a research task
    Research {
        #[arg(short, long)]
        topic: String,
        #[arg(short, long, default_value = "medium")]
        depth: String,
        #[arg(short, long)]
        providers: Vec<String>,
    },
    /// Export JSON schema for the contract
    Schema,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Research { topic, depth, providers } => {
            let res_depth = match depth.as_str() {
                "shallow" => ResearchDepth::Shallow,
                "deep" => ResearchDepth::Deep,
                _ => ResearchDepth::Medium,
            };

            let req_providers = if providers.is_empty() { 
                vec!["gemini".to_string()] 
            } else { 
                providers 
            };
            
            let mut summary = String::new();
            
            for p_name in &req_providers {
                let prompt = format!(
                    "Research the following topic and provide key claims: {}. Depth: {:?}", 
                    topic, res_depth
                );

                let result = match p_name.as_str() {
                    "gemini" => GeminiProvider {}.query(&prompt).await,
                    "claude" => ClaudeProvider {}.query(&prompt).await,
                    "codex" => CodexProvider {}.query(&prompt).await,
                    _ => Err(anyhow::anyhow!("Unknown provider: {}", p_name)),
                };

                match result {
                    Ok(text) => {
                        summary.push_str(&format!("--- {} ---\n{}\n", p_name, text));
                    }
                    Err(e) => {
                        eprintln!("Warning: Provider {} failed: {}", p_name, e);
                    }
                }
            }

            // Simple mock response extraction from summaries
            let response = ResearchResponse {
                task_id: Uuid::new_v4(),
                claims: vec![
                    Claim {
                        text: "Initial extracted claim placeholder".to_string(),
                        confidence: 0.8,
                        sources: vec![],
                        verification_status: VerificationStatus::Unverified,
                    }
                ],
                summary: if summary.is_empty() { 
                    "No provider response. Ensure CLIs (gemini, claude, codex) are authenticated and in PATH.".to_string() 
                } else { 
                    summary 
                },
            };

            let envelope = Envelope::success(response);
            println!("{}", serde_json::to_string_pretty(&envelope)?);
        }
        Commands::Schema => {
            let schema = schemars::schema_for!(Envelope<ResearchResponse>);
            println!("{}", serde_json::to_string_pretty(&schema)?);
        }
    }
    
    Ok(())
}
