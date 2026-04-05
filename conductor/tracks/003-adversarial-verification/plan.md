# Plan: Adversarial Verification

1. In `src/core/researcher.rs`, after the Claim Extraction stage (Stage 2), add a new Stage 2.5: ADVERSARIAL VERIFICATION.
2. Select a "verifier" provider. If more than 1 provider is available, pick `providers.last().unwrap()`, which should be different from the primary extractor (`providers.first().unwrap()`).
3. For each extracted claim, construct a verification prompt: "Act as a contrarian fact-checker. Given the research data, try to refute the following claim..."
4. Based on the verifier's response (e.g., asking it to output JSON with status "Verified", "Rejected", "Contradictory"), parse and update the claim's status.
5. (MVP shortcut) Verify all claims in bulk to save time/API calls: "Here are N claims. Refute them based on the text. Return a JSON array with updated statuses."
6. Proceed to Stage 3 (Synthesis).
7. Test build and execution.
