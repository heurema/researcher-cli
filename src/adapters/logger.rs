use crate::contract::schema::SessionState;
use std::fs::{self, OpenOptions, create_dir_all};
use std::io::{Write, Read};
use std::path::PathBuf;
use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

pub struct ResearchLogger {
    log_dir: PathBuf,
    pub task_id: Uuid,
}

impl ResearchLogger {
    pub fn new(task_id: Uuid) -> Result<Self> {
        let log_dir = PathBuf::from("artifacts").join(task_id.to_string());
        create_dir_all(&log_dir)?;
        Ok(Self { log_dir, task_id })
    }

    pub fn log(&self, entry: &str) -> Result<()> {
        let log_file = self.log_dir.join("RESEARCH_LOG.md");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;

        let timestamp = Utc::now().to_rfc3339();
        writeln!(file, "## [{}]", timestamp)?;
        writeln!(file, "{}", entry)?;
        writeln!(file, "\n----\n")?;
        Ok(())
    }

    pub fn save_state(&self, state: &SessionState) -> Result<()> {
        let state_file = self.log_dir.join("session_state.json");
        let json = serde_json::to_string_pretty(state)?;
        fs::write(state_file, json)?;
        Ok(())
    }

    pub fn load_state(task_id: Uuid) -> Result<SessionState> {
        let state_file = PathBuf::from("artifacts").join(task_id.to_string()).join("session_state.json");
        let mut file = fs::File::open(state_file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let state = serde_json::from_str(&content)?;
        Ok(state)
    }

    pub fn path(&self) -> String {
        self.log_dir.join("RESEARCH_LOG.md").to_string_lossy().into_owned()
    }
}
