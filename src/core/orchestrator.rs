use anyhow::Result;

pub trait Orchestrator {
    fn dispatch(&self, command: &str, payload: &str) -> Result<String>;
}
