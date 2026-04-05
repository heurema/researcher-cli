# researcher

Local-first, contract-first autonomous research orchestrator.

`researcher` is a high-integrity CLI tool designed for deep technical investigation, claim extraction, and adversarial verification. It acts as a unified interface for multiple LLMs, ensuring research is conducted with scientific rigor and auditable logic.

It combines:
- **orchestration** from `delve` (SCAN → DIVE → VERIFY)
- **rigor** from `MoMoA-Researcher` (Hypotheses, Scientific Method, Research Logger)
- **integrity** from `specforge` (CLI Doctrine, Contract-First, Machine-Safe Envelopes)

## Status

This repo is in **active development (v0.1.0)**.

- core research pipeline is functional
- supports Claude, Gemini, and Codex via system-level OAuth/Subscriptions
- built-in research tools (CodeRunner, Optimizer) implemented
- security-hardened against code and prompt injection

## Product thesis

`researcher` is a local-first runtime for AI-driven research tasks that require more than one model's opinion.

It has three pillars:
- **Multimodal Dive** — parallel data collection from independent LLM providers to avoid model bias.
- **Adversarial Verification** — structured contrarian analysis where one model attempts to refute the claims of another.
- **Scientific Audit** — append-only research logs and state checkpoints for every session.

It is built around a stage-gated pipeline:
- **`dive`** — collect raw intelligence from multiple providers.
- **`extract`** — distill atomic, verifiable claims from the noise.
- **`verify`** — run adversarial checks against extracted claims.
- **`synthesize`** — produce final summary and future hypotheses.

## Core model

Canonical research lifecycle:

```text
Topic
  -> Task (UUID)
    -> Dive (Raw Outputs)
      -> Claims (Confidence + Sources)
        -> Verification (Adversarial Status)
          -> Synthesis (Summary + Hypotheses)
            -> Research Log (Audit Trail)
```

## How to use

### 1. New Research

Start a deep research task using multiple providers:

```bash
researcher research --topic "Room Temperature Superconductors" --providers gemini claude codex
```

### 2. Resume Session

Resume an interrupted session from the last checkpoint:

```bash
# Resume the latest session
researcher research --resume

# Resume specific session
researcher research --task-id <uuid>
```

### 3. Research Tools

Run sandboxed calculations or optimizations:

```bash
# Run code
researcher tools run --lang py --code "print(2**10)"

# Optimize function parameters
researcher researcher tools optimize --params 1 2 3 --code "print({} + 10)"
```

### 4. Machine Interface

Export JSON schema for agent-to-agent integration:

```bash
researcher schema
```

## CLI Doctrine

This project strictly adheres to the following rules:
- **Contract-First**: API is defined in `src/contract/schema.rs` before implementation.
- **Machine-Safe**: `stdout` is reserved for JSON envelopes. Logs go to `stderr`.
- **Zero Secrets**: Uses system CLI orchestration (`gemini`, `claude`, `codex`) to leverage existing OAuth sessions. No API keys in `.env` required.

## Target Architecture

```text
researcher-cli/
├── src/
│   ├── contract/      # JSON Schemas (Source of Truth)
│   ├── core/          # Domain Logic & Research Stages
│   │   └── tools/     # CodeRunner, Optimizer
│   └── adapters/      # CLI/IO (Providers, Logger)
├── artifacts/         # Research Logs & Session States
└── conductor/         # Track-based development plans
```

## License

MIT
