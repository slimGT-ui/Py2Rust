# Session Handoff

Update this file at the end of every meaningful work session. It is the concise source of truth for resuming work.

## Current state
- Project: Py2Rust
- Repository: https://github.com/slimGT-ui/4
- Date initialized: 2026-11-20
- Milestone: `v0.1.0 — Foundational MVP`
- Phase: Phase 0 — documentation and repository setup
- Phase status: documentation committed; Rust workspace not initialized
- Implementation: NOT STARTED
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
- Repository `slimGT-ui/4` was identified and confirmed empty before documentation was created.
- Initial documentation files were committed individually to the default branch.
- No Rust implementation, build, or tests exist yet; no code checks have been run.

## Key decisions
See `DECISIONS.md`: broad practical transpiler; Rust implementation; incremental MVP; evaluate existing parser (rustpython-parser candidate); documentation updated after every meaningful change.

## Blockers / open questions
- Rust toolchain on the user's Arch laptop has not been checked for this repository.
- Parser library/version and Python grammar target have not been evaluated.
- Workspace crate boundaries are provisional and should be kept minimal at first.

## Exact next task
Start Phase 1:
1. Check `rustc --version`, `cargo --version`, and Git state in the user's local clone.
2. Initialize a minimal Cargo workspace and CLI binary.
3. Add baseline formatting, lint, and tests.
4. Verify build/test commands.
5. Update roadmap, changelog, architecture if needed, and this handoff with exact results.

## Do not redo
- Do not recreate or replace these documents without inspecting them.
- Do not claim Phase 1 or any translation feature is complete.
- Do not add a custom parser before evaluating maintained parser options.
- Do not mark tests passed unless they were actually run.
