use crate::contract::schema::{ResearchRequest, ResearchResponse, Claim, VerificationStatus, ResearchDepth, Hypothesis, HypothesisStatus};
use crate::adapters::providers::Provider;
use crate::adapters::logger::ResearchLogger;
use anyhow::Result;
use uuid::Uuid;

pub struct ResearcherService {
    providers: Vec<Box<dyn Provider>>,
}

impl ResearcherService {
    pub fn new(providers: Vec<Box<dyn Provider>>) -> Self {
        Self { providers }
    }

    pub async fn conduct_research(&self, request: ResearchRequest) -> Result<ResearchResponse> {
        let task_id = Uuid::new_v4();
        let logger = ResearchLogger::new(task_id)?;
        
        logger.log(&format!("Starting research on topic: {}", request.topic))?;
        
        let mut raw_outputs = Vec::new();
        
        // Stage 1: DIVE (Parallel research)
        for provider in &self.providers {
            if request.providers.contains(&provider.name().to_string()) || request.providers.is_empty() {
                let prompt = format!(
                    "Perform deep research on the following topic: {}. \
                    Depth: {:?}. \
                    Provide a detailed technical breakdown with specific facts.", 
                    request.topic, request.depth
                );
                
                logger.log(&format!("Invoking provider: {}", provider.name()))?;
                match provider.query(&prompt).await {
                    Ok(response) => {
                        logger.log(&format!("Provider {} response received ({} chars)", provider.name(), response.len()))?;
                        raw_outputs.push((provider.name().to_string(), response));
                    }
                    Err(e) => {
                        logger.log(&format!("Provider {} failed: {}", provider.name(), e))?;
                    }
                }
            }
        }

        if raw_outputs.is_empty() {
            anyhow::bail!("All providers failed to respond or no providers specified.");
        }

        // Stage 2: CLAIM EXTRACTION (Using the first available provider)
        let extractor = &self.providers[0]; // Simple selection for now
        let combined_raw: String = raw_outputs.iter()
            .map(|(name, text)| format!("--- Source: {} ---\n{}\n", name, text))
            .collect();

        let extraction_prompt = format!(
            "Analyze the following research data and extract a list of specific, verifiable claims. \
            For each claim, provide: \
            1. The text of the claim. \
            2. A confidence score (0.0 to 1.0). \
            Format your response as a JSON list: [{{\"text\": \"...\", \"confidence\": 0.9}}, ...]. \
            Data:\n{}", 
            combined_raw
        );

        logger.log(&format!("Extracting claims using provider: {}", extractor.name()))?;
        let claims_json = extractor.query(&extraction_prompt).await?;
        
        // Stage 3: SYNTHESIS & HYPOTHESES
        let synthesis_prompt = format!(
            "Based on the following research and extracted claims, provide a concise final summary. \
            Also, propose 2-3 hypotheses for further research. \
            Claims: {}\nResearch Data: {}", 
            claims_json, combined_raw
        );
        
        let synthesis = extractor.query(&synthesis_prompt).await?;

        // Parse claims (basic JSON extraction for MVP)
        let claims: Vec<Claim> = serde_json::from_str(&claims_json).unwrap_or_else(|_| {
            vec![Claim {
                text: "Failed to parse claims as JSON. See raw logs.".to_string(),
                confidence: 0.0,
                sources: vec![],
                verification_status: VerificationStatus::Unverified,
            }]
        });

        let hypotheses = vec![
            Hypothesis {
                text: "Further investigation needed into the consistency of provider outputs.".to_string(),
                status: HypothesisStatus::Proposed,
            }
        ];

        Ok(ResearchResponse {
            task_id,
            hypotheses,
            claims,
            summary: synthesis,
            log_path: logger.path(),
        })
    }
}
