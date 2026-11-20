# Session Handoff

Update this file at the end of every meaningful work session. It is the concise source of truth for resuming work.

## Current state
- Project: Py2Rust
- Canonical repository: https://github.com/slimGT-ui/Py2Rust
- Previous repository name: `slimGT-ui/4`
- Date initialized: 2026-11-20
- Milestone: `v0.1.0 — Foundational MVP`
- Current phase: Phase 1 — Rust foundation
- Phase status: IN PROGRESS; local environment preflight is the active subtask
- Implementation: no Rust source files created yet
- Last verified code state: no implementation inspected or tested
- Default branch: `main`

## Goal
Build a Rust-based Python-to-Rust transpiler that progressively supports broad Python syntax/semantics and eventually multi-file projects. Preserve behavior where practical; clearly report unsupported or runtime-backed constructs. Do not promise perfect translation of arbitrary Python.

## Documentation in repository
- `README.md`
- `docs/ROADMAP.md`
- `docs/ARCHITECTURE.md`
- `docs/COMPATIBILITY.md`
- `docs/DEVELOPMENT.md`
- `docs/DECISIONS.md`
- `docs/CHANGELOG.md`
- `docs/SESSION_HANDOFF.md`

## Verified work
- Canonical repository `slimGT-ui/Py2Rust` was retrieved successfully.
- Existing `README.md`, `ROADMAP.md`, `DEVELOPMENT.md`, `ARCHITECTURE.md`, `CHANGELOG.md`, `DECISIONS.md`, and this handoff were fetched from the canonical repository.
- The repository description includes a work-in-progress status.
- Phase 1 has been initiated in project documentation.
- No Rust implementation, build, or tests exist yet; no code checks have been run.

## Key decisions
See `DECISIONS.md`: broad practical transpiler; Rust implementation; incremental MVP; evaluate existing parser (rustpython-parser candidate); documentation updated after every meaningful change.
- Phase 1 begins with one minimal Cargo project/CLI; do not create empty crates just to match a future architecture diagram.
- Do not add the parser dependency during Phase 1.

## Blockers / open questions
- Local clone path and Git working-tree state are not yet known.
- Rust compiler and Cargo versions on the user's Arch laptop have not been checked.
- Workspace/package layout will be finalized after local environment preflight.

## Exact next task
From the root of the local Py2Rust clone, run:
```bash
pwd
git status --short
rustc --version
cargo --version
```
Send the complete output. Then:
1. Confirm the working tree and branch before editing.
2. Initialize the minimal Cargo project and CLI binary.
3. Add baseline formatting, lint, tests, and CI.
4. Run the checks locally and record exact output.
5. Update roadmap, changelog, architecture/decisions if needed, and this handoff.

## Do not redo
- Do not recreate or replace the existing documentation.
- Do not claim Phase 1 or any translation feature is complete.
- Do not add a custom parser before evaluating maintained parser options.
- Do not mark tests passed unless they were actually run.
