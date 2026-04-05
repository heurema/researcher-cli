# Workflow & CLI Doctrine

This project strictly follows the **CLI Doctrine** (as defined in SpecForge):

1. **Contract-First:** Never modify the implementation before updating the contract in `src/contract/`. All input/output is governed by versioned JSON schemas.
2. **Machine-Safe Output:** `stdout` is reserved for valid JSON envelopes. All logs, warnings, and diagnostic info must go to `stderr`.
3. **Architecture:**
   - `src/contract/`: Source of truth for the JSON API.
   - `src/core/`: Domain logic and command definitions (no IO).
   - `src/adapters/`: Concrete implementations (network, filesystem, providers).
   - `src/main.rs`: Thin CLI wrapper.
4. **Verification:** Every slice must have negative tests for unsupported fields and positive tests for supported ones. No silent failures.