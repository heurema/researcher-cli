# Product Definition: ResearcherCLI

## Vision
A universal, unified CLI orchestrator for interacting with Codex, Claude Code, and Gemini CLI.
 It acts as a primary interface for autonomous deep research, multi-agent coordination, and codebase modifications, leveraging the strengths of multiple LLM providers under a single contract-first protocol.

## Core Capabilities
- **Unified Interface:** One CLI command (`omni`) to route requests to the best-suited model (Claude for reasoning/research, Gemini for context/speed, Codex for pure code generation).
- **Stage-Gated Pipeline (from Delve):** SCAN → DECOMPOSE → DIVE → VERIFY → SYNTHESIZE. Includes claim extraction and adversarial verification.
- **Hierarchical Workspace (from MoMoA):** Research projects act as workspaces with overarching goals containing multiple individual research sessions.
- **Local Tooling Sandbox:** Specific "Research Tools" (e.g., Code Runner, Optimizer, Research Logger) that operate securely, tracking experiments using the scientific method.
- **Contract-First Communication (from SpecForge):** Strict separation between domain core and adapters. Machine-safe output standards (JSON envelopes) for agent-to-agent communication, with human-friendly output strictly opt-in.

## Target Audience
- **Developers & AI Researchers:** Primary users invoking `/omni` interactively for coding and research.
- **Agents (Self-Hosting):** The CLI itself is designed to be invoked by other agents (machine-safe output via stdout, diagnostics via stderr).