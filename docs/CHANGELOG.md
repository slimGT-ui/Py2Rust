# Development Changelog

Chronological engineering log. Add an entry after every meaningful work session.

## 2026-11-20 — Initial project documentation
**Type:** Documentation / planning  
**Status:** Committed to `slimGT-ui/4`; no implementation yet.

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

### Not done
- No Rust code written.
- No Cargo workspace initialized.
- No dependencies installed or evaluated.
- No tests, builds, or compiler checks run.
- No compatibility feature implemented.

### Next
1. Inspect repository state and local environment.
2. Initialize Phase 1 Cargo workspace.
3. Add minimal CLI and baseline formatting/lint/test configuration.
4. Evaluate parser dependency before adoption.
