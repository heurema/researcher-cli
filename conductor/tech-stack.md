# Tech Stack

- **Language:** Rust (Edition 2024) - chosen for performance, safety, and strict contract modeling (CLI Doctrine).
- **CLI Framework:** `clap` (for routing requests and formatting results).
- **Serialization:** `serde` and `serde_json` for contract enforcement.
- **Schema Generation:** `schemars` for JSON schema generation of the API contract.
- **Storage:** File-based only (no external database), using append-only logs for the Research Logger.
- **Testing:** Standard Rust test framework with deep integration/negative tests.