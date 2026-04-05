# Plan: Resume Support

1. Define a `SessionState` struct in `src/contract/schema.rs` containing:
   - `task_id: Uuid`
   - `topic: String`
   - `depth: ResearchDepth`
   - `completed_stage: String`
   - `raw_outputs: Vec<(String, String)>`
   - `claims: Vec<Claim>`
   - `summary: Option<String>`
2. Update `ResearchLogger` in `src/adapters/logger.rs` to include a `save_state(&self, state: &SessionState)` method.
3. Update `ResearcherService::conduct_research` to accept an optional `SessionState`. If provided, skip stages marked as `completed_stage`.
4. Add `--resume` and `--task-id` logic to `src/main.rs`.
   - Store session history in a `.researcher_sessions.json` for easy latest-session lookup.
5. Verification: Interrupt a run, resume with `--resume`, and check if it continues from the correct stage.
