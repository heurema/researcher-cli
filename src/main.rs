mod contract;
mod core;
mod adapters;

use clap::{Parser, Subcommand};
use crate::contract::schema::{Envelope, ResearchRequest, ResearchResponse, ResearchDepth};
use crate::adapters::providers::{ClaudeProvider, GeminiProvider, CodexProvider};
use crate::core::researcher::ResearcherService;

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

            let request = ResearchRequest {
                topic,
                depth: res_depth,
                providers: if providers.is_empty() { 
                    vec!["gemini".to_string(), "claude".to_string()] 
                } else { 
                    providers 
                },
            };

            // Initialize all available providers
            let service_providers: Vec<Box<dyn crate::adapters::providers::Provider>> = vec![
                Box::new(GeminiProvider {}),
                Box::new(ClaudeProvider {}),
                Box::new(CodexProvider {}),
            ];

            let service = ResearcherService::new(service_providers);
            
            match service.conduct_research(request).await {
                Ok(response) => {
                    let envelope = Envelope::success(response);
                    println!("{}", serde_json::to_string_pretty(&envelope)?);
                }
                Err(e) => {
                    let envelope: Envelope<ResearchResponse> = Envelope::error("research_failed", &e.to_string());
                    println!("{}", serde_json::to_string_pretty(&envelope)?);
                }
            }
        }
        Commands::Schema => {
            let schema = schemars::schema_for!(Envelope<ResearchResponse>);
            println!("{}", serde_json::to_string_pretty(&schema)?);
        }
    }
    
    Ok(())
}
