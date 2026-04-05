use crate::contract::schema::{ResearchRequest, ResearchResponse};
use anyhow::Result;

pub trait Researcher {
    fn research(&self, request: ResearchRequest) -> Result<ResearchResponse>;
}
