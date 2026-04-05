mod contract;
mod core;
mod adapters;

use clap::{Parser, Subcommand};
use crate::contract::schema::{Envelope, ResearchRequest, ResearchResponse, Claim, VerificationStatus, ResearchDepth};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about = "OmniCLI: Universal AI CLI Orchestrator")]
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

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Research { topic, depth, providers } => {
            let res_depth = match depth.as_str() {
                "shallow" => ResearchDepth::Shallow,
                "deep" => ResearchDepth::Deep,
                _ => ResearchDepth::Medium,
            };

            let _request = ResearchRequest {
                topic: topic.clone(),
                depth: res_depth,
                providers: if providers.is_empty() { vec!["claude".to_string()] } else { providers },
            };

            // Mock response
            let response = ResearchResponse {
                task_id: Uuid::new_v4(),
                claims: vec![
                    Claim {
                        text: format!("Claim 1 about {}", topic),
                        confidence: 0.95,
                        sources: vec!["https://example.com".to_string()],
                        verification_status: VerificationStatus::Verified,
                    }
                ],
                summary: format!("Summary for research on {}", topic),
            };

            let envelope = Envelope::success(response);
            println!("{}", serde_json::to_string_pretty(&envelope).unwrap());
        }
        Commands::Schema => {
            let schema = schemars::schema_for!(Envelope<ResearchResponse>);
            println!("{}", serde_json::to_string_pretty(&schema).unwrap());
        }
    }
}
