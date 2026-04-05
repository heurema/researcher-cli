mod contract;
mod core;
mod adapters;

use clap::{Parser, Subcommand};
use crate::contract::schema::{Envelope, ResearchRequest, ResearchResponse, ResearchDepth};
use crate::adapters::providers::{ClaudeProvider, GeminiProvider, CodexProvider};
use crate::adapters::logger::ResearchLogger;
use crate::core::researcher::ResearcherService;
use crate::core::tools::code_runner::CodeRunner;
use crate::core::tools::optimizer::Optimizer;
use uuid::Uuid;
use std::fs;
use serde::{Serialize, Deserialize};

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
        topic: Option<String>,
        #[arg(short, long, default_value = "medium")]
        depth: String,
        #[arg(short, long)]
        providers: Vec<String>,
        #[arg(short, long)]
        resume: bool,
        #[arg(long)]
        task_id: Option<String>,
    },
    /// Run specialized research tools
    Tools {
        #[command(subcommand)]
        tool: ToolCommands,
    },
    /// Export JSON schema for the contract
    Schema,
}

#[derive(Subcommand, Debug)]
enum ToolCommands {
    /// Execute a code snippet
    Run {
        #[arg(short, long)]
        lang: String,
        #[arg(short, long)]
        code: String,
    },
    /// Optimize a function over a parameter space
    Optimize {
        #[arg(short, long)]
        params: Vec<String>,
        #[arg(short, long)]
        code: String,
    },
}

#[derive(Serialize, Deserialize, Default)]
struct SessionHistory {
    latest_task_id: Option<Uuid>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Research { topic, depth, providers, resume, task_id } => {
            let history_path = ".researcher_sessions.json";
            let mut history: SessionHistory = fs::read_to_string(history_path)
                .map(|s| serde_json::from_str(&s).unwrap_or_default())
                .unwrap_or_default();

            let target_task_id = if let Some(id_str) = task_id {
                Some(Uuid::parse_str(&id_str)?)
            } else if resume {
                history.latest_task_id
            } else {
                None
            };

            let resume_state = if let Some(id) = target_task_id {
                ResearchLogger::load_state(id).ok()
            } else {
                None
            };

            let (final_topic, final_depth, final_providers) = if let Some(ref s) = resume_state {
                (s.topic.clone(), s.depth, s.providers.clone())
            } else {
                let t = topic.ok_or_else(|| anyhow::anyhow!("Topic is required for new research"))?;
                let d = match depth.as_str() {
                    "shallow" => ResearchDepth::Shallow,
                    "deep" => ResearchDepth::Deep,
                    _ => ResearchDepth::Medium,
                };
                let p = if providers.is_empty() { 
                    vec!["gemini".to_string(), "claude".to_string()] 
                } else { 
                    providers 
                };
                (t, d, p)
            };

            let request = ResearchRequest {
                topic: final_topic,
                depth: final_depth,
                providers: final_providers,
            };

            let service_providers: Vec<Box<dyn crate::adapters::providers::Provider>> = vec![
                Box::new(GeminiProvider {}),
                Box::new(ClaudeProvider {}),
                Box::new(CodexProvider {}),
            ];

            let service = ResearcherService::new(service_providers);
            
            match service.conduct_research(request, resume_state).await {
                Ok(response) => {
                    history.latest_task_id = Some(response.task_id);
                    let _ = fs::write(history_path, serde_json::to_string(&history)?);
                    
                    let envelope = Envelope::success(response);
                    println!("{}", serde_json::to_string_pretty(&envelope)?);
                }
                Err(e) => {
                    let envelope: Envelope<ResearchResponse> = Envelope::error("research_failed", &e.to_string());
                    println!("{}", serde_json::to_string_pretty(&envelope)?);
                }
            }
        },
        Commands::Tools { tool } => {
            match tool {
                ToolCommands::Run { lang, code } => {
                    let result = match lang.to_lowercase().as_str() {
                        "python" | "python3" | "py" => CodeRunner::run_python(&code).await,
                        "rust" | "rs" => CodeRunner::run_rust(&code).await,
                        _ => anyhow::bail!("Unsupported language: {}", lang),
                    };
                    match result {
                        Ok(o) => println!("{}", o),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                },
                ToolCommands::Optimize { params, code } => {
                    match Optimizer::optimize(&code, params).await {
                        Ok((best_p, best_o)) => println!("Best parameter: {}\nOutput: {}", best_p, best_o),
                        Err(e) => eprintln!("Optimization Error: {}", e),
                    }
                }
            }
        },
        Commands::Schema => {
            let schema = schemars::schema_for!(Envelope<ResearchResponse>);
            println!("{}", serde_json::to_string_pretty(&schema)?);
        }
    }
    
    Ok(())
}
