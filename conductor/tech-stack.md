# Tech Stack

- [package]
- **Language:** Rust (Edition 2024)
- **CLI Framework:** `clap`
- **Serialization:** `serde`, `serde_json`, `schemars`
- **Orchestration:** Subprocess invocation of `claude`, `gemini`, and `codex` CLIs. This leverages system-level OAuth authentication and user subscriptions, removing the need for raw API keys in the environment.
- **Async Runtime:** `tokio` (for non-blocking CLI orchestration).