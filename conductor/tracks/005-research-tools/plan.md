# Plan: Research Tools

1. Create `src/core/tools/mod.rs` and `src/core/tools/code_runner.rs`.
2. Implement `CodeRunner` using `std::process::Command` with `tokio::time::timeout`. Support `python3` and `rustc` (compiling a temp file then running).
3. Implement `Optimizer` in `src/core/tools/optimizer.rs` using a thread pool or `tokio` for concurrent execution.
4. Integrate tools into `src/main.rs` as a new `Tools` subcommand.
5. Verification:
   - Run a Python script via `tools run --lang python --code "print(2+2)"`.
   - Run a basic optimization loop via `tools optimize --params "range(1,10)" --func "x**2"`.
6. Update the documentation to reflect these new capabilities.
