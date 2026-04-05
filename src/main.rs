mod contract;
mod core;
mod adapters;

use clap::{Parser, Subcommand};
use crate::contract::schema::{Envelope, ResearchRequest, ResearchResponse, Claim, VerificationStatus, ResearchDepth};
use crate::adapters::providers::{ClaudeProvider, GeminiProvider, Provider};
use uuid::Uuid;
use dotenvy::dotenv;
use std::env;

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
    dotenv().ok();
    let args = Args::parse();

    match args.command {
        Commands::Research { topic, depth, providers } => {
            let res_depth = match depth.as_str() {
                "shallow" => ResearchDepth::Shallow,
                "deep" => ResearchDepth::Deep,
                _ => ResearchDepth::Medium,
            };

            let req_providers = if providers.is_empty() { vec!["gemini".to_string()] } else { providers };
            
            let mut summary = String::new();
            
            for p_name in &req_providers {
                if p_name == "gemini" {
                    if let Ok(p) = GeminiProvider::new() {
                        let prompt = format!("Research the following topic and provide key claims: {}. Depth: {:?}", topic, res_depth);
                        if let Ok(text) = p.query(&prompt).await {
                            summary.push_str(&format!("--- Gemini ---\n{}\n", text));
                        }
                    }
                } else if p_name == "claude" {
                    if let Ok(p) = ClaudeProvider::new() {
                        let prompt = format!("Research the following topic and provide key claims: {}. Depth: {:?}", topic, res_depth);
                        if let Ok(text) = p.query(&prompt).await {
                            summary.push_str(&format!("--- Claude ---\n{}\n", text));
                        }
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
                summary: if summary.is_empty() { "No provider response".to_string() } else { summary },
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
