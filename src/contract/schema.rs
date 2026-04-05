use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Complete,
    PartialFailure,
    Truncated,
    Error,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Envelope<T> {
    pub version: String,
    pub status: Status,
    pub timestamp: DateTime<Utc>,
    pub data: Option<T>,
    pub errors: Vec<ErrorMessage>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ErrorMessage {
    pub code: String,
    pub message: String,
    pub level: ErrorLevel,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResearchRequest {
    pub topic: String,
    pub depth: ResearchDepth,
    pub providers: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum ResearchDepth {
    Shallow,
    #[default]
    Medium,
    Deep,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ResearchResponse {
    pub task_id: uuid::Uuid,
    pub claims: Vec<Claim>,
    pub summary: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Claim {
    pub text: String,
    pub confidence: f32,
    pub sources: Vec<String>,
    pub verification_status: VerificationStatus,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum VerificationStatus {
    Unverified,
    Verified,
    Rejected,
    Contradictory,
}

impl<T> Envelope<T> {
    pub fn success(data: T) -> Self {
        Self {
            version: "1.0".to_string(),
            status: Status::Complete,
            timestamp: Utc::now(),
            data: Some(data),
            errors: Vec::new(),
        }
    }

    pub fn error(code: &str, message: &str) -> Self {
        Self {
            version: "1.0".to_string(),
            status: Status::Error,
            timestamp: Utc::now(),
            data: None,
            errors: vec![ErrorMessage {
                code: code.to_string(),
                message: message.to_string(),
                level: ErrorLevel::Error,
            }],
        }
    }
}
