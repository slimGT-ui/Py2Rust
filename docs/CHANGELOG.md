# Development Changelog

Chronological engineering log. Add an entry after every meaningful work session.

## 2026-11-20 — Phase 1 kickoff: Rust foundation
**Type:** Project setup / engineering workflow  
**Status:** IN PROGRESS; no Rust implementation has been written yet.

### Confirmed repository state
- Canonical repository name is now `slimGT-ui/Py2Rust` (the repository was previously referred to as `slimGT-ui/4`).
- Default branch: `main`.
- Existing project documentation was fetched successfully from the canonical repository.
- The repository description identifies the project as a work in progress.

### Started
- Phase 1 — Rust foundation.
- First subtask: verify the user's local clone, working-tree state, and Rust toolchain before creating project files.

### Scope for Phase 1
- Minimal Cargo project/workspace and CLI binary.
- Documented Rust toolchain policy.
- rustfmt, clippy, tests, and CI.
- Basic CLI help/version and robust input-file validation.
- Structured, actionable errors.
- No Python parser or translation implementation in this phase.

### Not done / not yet verified
- No Rust source files or Cargo manifest have been added.
- No local toolchain versions have been reported.
- No build, formatting, lint, or test commands have been run.
- No parser dependency has been selected or added.

### Next
Run the local preflight commands in `SESSION_HANDOFF.md`. Once the clone and toolchain are confirmed, create the minimal Cargo project and verify it on the user's machine.

## 2026-11-20 — Initial project documentation
**Type:** Documentation / planning  
**Status:** Committed to the repository; no implementation at that time.

### Added
- Project overview, scope, and compatibility expectations.
- Twelve-phase roadmap and v0.1.0 MVP definition.
- Proposed pipeline, component responsibilities, and workspace layout.
- Compatibility vocabulary and initial matrix.
- Development workflow and definition of done.
- Architecture decision log and session handoff.

### Decisions
- Long-term broad Python-to-Rust transpiler; no promise of perfect conversion for arbitrary Python.
- Implement transpiler in Rust.
- Evaluate an existing parser first; rustpython-parser is the initial candidate, not yet a pinned dependency.
- Work in small milestones with tests and honest verification.
- Update docs after every meaningful change.

### Not done at initial documentation stage
- No Rust code written.
- No Cargo workspace initialized.
- No dependencies installed or evaluated.
- No tests, builds, or compiler checks run.
- No compatibility feature implemented.
