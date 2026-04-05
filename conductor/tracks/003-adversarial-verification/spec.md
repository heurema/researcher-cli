# Specification: Adversarial Verification

Implement the VERIFY stage from Delve's pipeline.

## Requirements
- **Verification Stage:** After extracting claims, verify them using a contrarian/adversarial prompt.
- **Cross-Model Verification:** Ideally, if multiple providers are specified, use a *different* provider to verify claims extracted by the first provider.
- **Status Update:** Update the `verification_status` of each claim to `Verified`, `Rejected`, or `Contradictory` based on the verification result.
- **Integration with Logger:** Log the verification process and its outcome in `RESEARCH_LOG.md`.
