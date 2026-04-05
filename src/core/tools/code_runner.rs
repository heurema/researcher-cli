use std::process::Stdio;
use tokio::process::Command;
use tokio::time::{timeout, Duration};
use anyhow::Result;
use std::fs;

pub struct CodeRunner;

impl CodeRunner {
    pub async fn run_python(code: &str) -> Result<String> {
        let output = timeout(Duration::from_secs(30), 
            Command::new("python3")
                .arg("-c")
                .arg(code)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?
                .wait_with_output()
        ).await??;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(stdout.to_string())
        } else {
            anyhow::bail!("Python error: {}", if stderr.is_empty() { stdout } else { stderr })
        }
    }

    pub async fn run_rust(code: &str) -> Result<String> {
        let tmp_file = "temp_research_tool.rs";
        let tmp_bin = "./temp_research_tool";
        
        fs::write(tmp_file, code)?;
        
        // Compile
        let compile = Command::new("rustc")
            .arg(tmp_file)
            .arg("-o")
            .arg(tmp_bin)
            .output()
            .await?;

        if !compile.status.success() {
            let err = String::from_utf8_lossy(&compile.stderr);
            anyhow::bail!("Rust compile error: {}", err)
        }

        // Run
        let run = timeout(Duration::from_secs(30), 
            Command::new(tmp_bin)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?
                .wait_with_output()
        ).await??;

        // Cleanup
        let _ = fs::remove_file(tmp_file);
        let _ = fs::remove_file(tmp_bin);

        let stdout = String::from_utf8_lossy(&run.stdout);
        let stderr = String::from_utf8_lossy(&run.stderr);

        if run.status.success() {
            Ok(stdout.to_string())
        } else {
            anyhow::bail!("Rust runtime error: {}", if stderr.is_empty() { stdout } else { stderr })
        }
    }
}
