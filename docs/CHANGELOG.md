# Development Changelog

Chronological engineering log. Add an entry after every meaningful work session.

## 2026-11-20 — Phase 1.1: Initial Cargo package and CLI
**Type:** Rust foundation / CLI / CI  
**Status:** IMPLEMENTED; local formatting, lint, build, and tests have not yet been run.

### Added
- Minimal single-package Cargo project using Rust edition 2024 and the stable toolchain.
- Standard-library-only Python input loader with structured errors for missing paths, directories, unsupported extensions, metadata failures, and read/UTF-8 failures.
- CLI help/version output and one-file input validation.
- Explicit message that Python translation is not implemented yet; input Python is never executed.
- Unit and integration tests for loader and CLI behavior.
- GitHub Actions CI matrix for Windows and Linux, running rustfmt, Clippy, and tests.
- Updated README, roadmap, decisions, and session handoff.

### Environment evidence supplied by the user
- Windows PowerShell; Rust 1.100.0 and Cargo 1.100.0.
- Stable target: `x86_64-pc-windows-msvc`.
- Visual Studio Build Tools 2022 17.14.45; MSVC 19.44.35230.
- Git 2.55.0.windows.5.
- Local clone: `C:\programming\Py2Rust`, branch `main`, clean and synchronized before this commit.

### Not yet verified
- No Cargo command has been run against the new source.
- No local formatting, Clippy, build, or test result is available.
- CI has not yet run on GitHub.
- Windows SDK/linker availability has not yet been confirmed by a real Rust build.
- Linux/Arch build has not yet been run.

### Next
Pull the commit on Windows and run the commands in `SESSION_HANDOFF.md`. Fix any compiler, test, or lint issues before marking Phase 1 verified.

## 2026-11-20 — Phase 1 kickoff: Rust foundation
**Type:** Project setup / engineering workflow  
**Status:** IN PROGRESS; environment preflight completed on Windows.

### Confirmed repository state
- Canonical repository: `slimGT-ui/Py2Rust`; default branch: `main`.
- User's Windows clone: `C:\programming\Py2Rust`, branch `main`, clean and synchronized before implementation.
- Repository description identifies the project as a work in progress.

### Started
- Phase 1 — Rust foundation.
- Verified Git, Rust/Cargo, and MSVC before creating project files.

### Scope
- Minimal Cargo project and CLI, stable toolchain policy, rustfmt, Clippy, tests, CI, actionable input-file errors.
- No Python parser or translation implementation in this phase.

## 2026-11-20 — Initial project documentation
**Type:** Documentation / planning  
**Status:** Committed; no implementation at that time.

### Added
- Project overview, scope, compatibility expectations, twelve-phase roadmap, and v0.1.0 MVP definition.
- Proposed pipeline, component responsibilities, and workspace layout.
- Compatibility vocabulary, development workflow, decision log, and session handoff.

### Decisions
- Long-term broad Python-to-Rust transpiler; no promise of perfect conversion for arbitrary Python.
- Implement transpiler in Rust.
- Evaluate an existing parser first; rustpython-parser is the initial candidate, not a pinned dependency.
- Work in small milestones with tests and honest verification.
- Update docs after every meaningful change.
