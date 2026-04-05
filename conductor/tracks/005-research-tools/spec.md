# Specification: Research Tools

Integrate specialized tools as seen in MoMoA-Researcher (Optimizer, Code Runner, Research Logger).

## Requirements
- **Code Runner:** Execute Python/Rust code in a sandboxed/restricted mode (via system subprocesses with timeouts). Return `stdout/stderr`.
- **Optimizer:** Run a function concurrently across a discrete or random search space. Return the best result and its parameters.
- **Unified Interface:** Expose these as subcommands: `omni-cli tools run --lang <lang> --code <code>` and `omni-cli tools optimize`.
- **Orchestration Integration:** Allow models to "call" these tools if necessary (e.g. by recognizing a special `TOOL_USE: <JSON>` syntax in provider outputs).
