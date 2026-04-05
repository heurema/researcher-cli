# Specification: Resume Support

Enable the researcher to resume interrupted sessions from the last successful stage.

## Requirements
- **State Persistence:** Save a `session_state.json` in the task's artifact directory after each stage (DIVE, EXTRACT, VERIFY, SYNTHESIZE).
- **Session ID:** Allow users to pass an existing `--task-id <uuid>` to resume.
- **Stages as Checkpoints:** Define stages in code so the orchestrator can skip completed ones.
- **CLI Flag:** Add `--resume` (resumes the latest) and `--task-id <uuid>` (resumes specific) flags to the `research` command.
