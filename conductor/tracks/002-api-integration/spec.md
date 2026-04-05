# Specification: API Integration

Implement real API adapters for Anthropic (Claude) and Google (Gemini). 

## Requirements
- **Async Runtime:** Use `tokio` for async execution.
- **HTTP Client:** Use `reqwest` for API calls.
- **Authentication:** Use environment variables (`ANTHROPIC_API_KEY`, `GOOGLE_API_KEY`).
- **Resilience:** Basic error mapping to the `Envelope` error model defined in Track 1.
- **Contract Adherence:** Return results wrapped in the `ResearchResponse` schema.
