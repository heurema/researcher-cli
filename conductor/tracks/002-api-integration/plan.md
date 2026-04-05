# Plan: API Integration

1. Update `Cargo.toml` with `tokio`, `reqwest`, `dotenvy`.
2. Update `src/adapters/providers.rs`:
   - Replace mocks with actual API calls.
   - Use `env::var` for keys.
   - Map JSON responses to internal types.
3. Update `src/main.rs`:
   - Initialize `dotenvy`.
   - Use `#[tokio::main]`.
   - Call real providers from the `Research` command.
4. Add a `.env.example` file.
5. Verification: Test with a small topic and mock API keys (to verify error handling) or real keys if available.
