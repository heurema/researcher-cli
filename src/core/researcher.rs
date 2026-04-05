use crate::contract::schema::{ResearchRequest, ResearchResponse, Claim, VerificationStatus, ResearchDepth, Hypothesis, HypothesisStatus, SessionState};
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

    /// Basic sanitization to remove shell metacharacters and suspicious prompt injection patterns.
    fn sanitize_input(input: &str) -> String {
        input.chars()
            .filter(|&c| c.is_alphanumeric() || c.is_whitespace() || ".-_/:".contains(c))
            .collect()
    }

    pub async fn conduct_research(&self, request: ResearchRequest, resume_state: Option<SessionState>) -> Result<ResearchResponse> {
        let task_id = resume_state.as_ref().map(|s| s.task_id).unwrap_or_else(Uuid::new_v4);
        let logger = ResearchLogger::new(task_id)?;
        
        let sanitized_topic = Self::sanitize_input(&request.topic);
        
        let mut state = resume_state.unwrap_or(SessionState {
            task_id,
            topic: sanitized_topic.clone(),
            depth: request.depth,
            providers: request.providers.clone(),
            completed_stage: "start".to_string(),
            raw_outputs: Vec::new(),
            claims: Vec::new(),
            summary: None,
        });

        if state.completed_stage == "start" {
            logger.log(&format!("Starting research on topic: {}", sanitized_topic))?;
        } else {
            logger.log(&format!("Resuming research on topic: {} from stage: {}", sanitized_topic, state.completed_stage))?;
        }

        // Stage 1: DIVE
        if state.completed_stage == "start" {
            for provider in &self.providers {
                if request.providers.contains(&provider.name().to_string()) || request.providers.is_empty() {
                    let prompt = format!(
                        "Perform deep research on the following topic: {}. \
                        Depth: {:?}. \
                        Provide a detailed technical breakdown with specific facts.", 
                        sanitized_topic, request.depth
                    );
                    
                    logger.log(&format!("Invoking provider: {}", provider.name()))?;
                    match provider.query(&prompt).await {
                        Ok(response) => {
                            logger.log(&format!("Provider {} response received", provider.name()))?;
                            state.raw_outputs.push((provider.name().to_string(), response));
                        }
                        Err(e) => {
                            logger.log(&format!("Provider {} failed: {}", provider.name(), e))?;
                        }
                    }
                }
            }
            if state.raw_outputs.is_empty() {
                anyhow::bail!("All providers failed to respond.");
            }
            state.completed_stage = "dive".to_string();
            logger.save_state(&state)?;
        }

        let combined_raw: String = state.raw_outputs.iter()
            .map(|(name, text)| format!("--- Source: {} ---\n{}\n", name, text))
            .collect();

        // Stage 2: EXTRACT
        if state.completed_stage == "dive" {
            let extractor = &self.providers[0];
            let extraction_prompt = format!(
                "Analyze the following research data and extract a list of specific, verifiable claims. \
                Format as JSON: [{{\"text\": \"...\", \"confidence\": 0.9}}, ...]\nData:\n{}", 
                combined_raw
            );

            logger.log(&format!("Extracting claims using: {}", extractor.name()))?;
            let claims_json = extractor.query(&extraction_prompt).await?;
            state.claims = serde_json::from_str(&claims_json).unwrap_or_default();
            state.completed_stage = "extract".to_string();
            logger.save_state(&state)?;
        }

        // Stage 2.5: VERIFY
        if state.completed_stage == "extract" {
            let verifier = self.providers.last().unwrap_or(&self.providers[0]);
            logger.log(&format!("Adversarial Verification using: {}", verifier.name()))?;
            
            let claims_text_only: Vec<String> = state.claims.iter().map(|c| c.text.clone()).collect();
            let verification_prompt = format!(
                "Refute these claims using ONLY the data. Return JSON array: [\"verified\", \"rejected\", \"contradictory\"].\nClaims: {:?}\nData: {}",
                claims_text_only, combined_raw
            );

            if let Ok(resp) = verifier.query(&verification_prompt).await {
                if let Ok(statuses) = serde_json::from_str::<Vec<String>>(&resp) {
                    for (i, status_str) in statuses.iter().enumerate() {
                        if let Some(claim) = state.claims.get_mut(i) {
                            claim.verification_status = match status_str.to_lowercase().as_str() {
                                "verified" => VerificationStatus::Verified,
                                "rejected" => VerificationStatus::Rejected,
                                "contradictory" => VerificationStatus::Contradictory,
                                _ => VerificationStatus::Unverified,
                            };
                        }
                    }
                }
            }
            state.completed_stage = "verify".to_string();
            logger.save_state(&state)?;
        }
        
        // Stage 3: SYNTHESIS
        if state.summary.is_none() {
            let synth_provider = &self.providers[0];
            let synthesis_prompt = format!(
                "Final summary and 2-3 hypotheses. Claims: {:?}\nData: {}", 
                state.claims, combined_raw
            );
            state.summary = Some(synth_provider.query(&synthesis_prompt).await.unwrap_or_else(|_| "Synthesis failed.".to_string()));
            state.completed_stage = "complete".to_string();
            logger.save_state(&state)?;
        }

        Ok(ResearchResponse {
            task_id,
            hypotheses: vec![Hypothesis { text: "Further investigation into claim consistency.".to_string(), status: HypothesisStatus::Proposed }],
            claims: state.claims,
            summary: state.summary.unwrap_or_default(),
            log_path: logger.path(),
        })
    }
}
