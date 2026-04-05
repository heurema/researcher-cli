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
        
        // Parse claims (basic JSON extraction for MVP)
        let mut claims: Vec<Claim> = serde_json::from_str(&claims_json).unwrap_or_else(|_| {
            vec![Claim {
                text: "Failed to parse claims as JSON. See raw logs.".to_string(),
                confidence: 0.0,
                sources: vec![],
                verification_status: VerificationStatus::Unverified,
            }]
        });

        // Stage 2.5: ADVERSARIAL VERIFICATION
        // Use a different provider for verification if multiple exist
        let verifier = self.providers.last().unwrap_or(extractor);
        logger.log(&format!("Starting Adversarial Verification using provider: {}", verifier.name()))?;
        
        let claims_text_only: Vec<String> = claims.iter().map(|c| c.text.clone()).collect();
        
        let verification_prompt = format!(
            "Act as a contrarian fact-checker. I have extracted the following claims from the research data. \
            Your job is to attempt to refute them using ONLY the provided research data. \
            If the data explicitly supports the claim, mark it 'verified'. \
            If the data contradicts the claim, mark it 'contradictory'. \
            If there is no evidence either way, mark it 'rejected'. \
            Return a JSON array of strings: [\"verified\", \"rejected\", \"contradictory\"], matching the order of the claims. \
            Claims:\n{:?} \
            Data:\n{}",
            claims_text_only,
            combined_raw
        );

        let verification_response = verifier.query(&verification_prompt).await.unwrap_or_default();
        logger.log(&format!("Verification response: {}", verification_response))?;

        // Attempt to parse verification response
        if let Ok(statuses) = serde_json::from_str::<Vec<String>>(&verification_response) {
            for (i, status_str) in statuses.iter().enumerate() {
                if let Some(claim) = claims.get_mut(i) {
                    claim.verification_status = match status_str.to_lowercase().as_str() {
                        "verified" => VerificationStatus::Verified,
                        "rejected" => VerificationStatus::Rejected,
                        "contradictory" => VerificationStatus::Contradictory,
                        _ => VerificationStatus::Unverified,
                    };
                }
            }
        } else {
            logger.log("Failed to parse verification response as JSON array.")?;
        }
        
        // Stage 3: SYNTHESIS & HYPOTHESES
        let synthesis_prompt = format!(
            "Based on the following research and verified claims, provide a concise final summary. \
            Also, propose 2-3 hypotheses for further research. \
            Claims: {:?}\nResearch Data: {}", 
            claims, combined_raw
        );
        let synthesis = extractor.query(&synthesis_prompt).await.unwrap_or_else(|_| "Failed to generate synthesis.".to_string());

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
