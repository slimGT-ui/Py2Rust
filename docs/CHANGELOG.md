# Development Changelog

Chronological engineering log. Add an entry after every meaningful work session.

## 2026-11-20 — Windows verification: formatting, Clippy, tests, and CLI help
**Type:** Verification  
**Status:** Local Windows checks passed; CI and manual file-input smoke tests remain pending.

### Environment
- Windows Developer PowerShell, repository `C:\programming\Py2Rust`.
- Rust/Cargo 1.100.0, stable `x86_64-pc-windows-msvc`, MSVC 19.44.35230.

### Commands reported by the user
- `git pull --ff-only` — fast-forwarded local main to commit `34275480582d4df6574cd3eeabf6e54c1a037c3e`.
- `cargo fmt --all` — completed successfully.
- `cargo fmt --all -- --check` — completed successfully (no output, exit success implied by subsequent commands).
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — completed successfully.
- `cargo test --workspace` — passed: 5 library unit tests + 5 CLI integration tests; 10 passed, 0 failed.
- `cargo run -- --help` — printed expected usage and current limitation.
- `cargo run -- --version` — printed `py2rust 0.1.0`.

### Still pending
- The user has applied rustfmt locally; resulting source changes need to be inspected and committed/pushed so CI tests the formatted source.
- GitHub Actions for the previously pushed, pre-rustfmt source still reports failure at the formatting step; no green CI result yet.
- Manual valid-file, missing-file, and directory CLI smoke tests not yet reported.
- Linux/Arch verification not yet run.

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

### CI feedback
- GitHub Actions was triggered on the initial implementation commits.
- The Linux job reached `Check formatting` and failed there; Clippy and tests were skipped. The hand-formatted source still differs from rustfmt output.
- This is not a compiler or test failure yet; run `cargo fmt --all` locally, then rerun the full checks.

### Windows local verification — passed
- `cargo fmt --all` and `cargo fmt --all -- --check`: passed.
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`: passed.
- `cargo test --workspace`: 10 passed, 0 failed (5 unit + 5 integration).
- `cargo run -- --help`: expected usage and limitation printed.
- `cargo run -- --version`: `py2rust 0.1.0`.
- Clippy and tests compiled the project successfully on the MSVC toolchain.

### Not yet verified
- GitHub Actions still tests the previously pushed source before the user's local rustfmt changes; it fails at formatting. Push the formatted source and wait for a fresh CI run.
- Manual valid-file, missing-file, and directory smoke tests have not yet been reported.
- A standalone `cargo build` command has not been run (Clippy/tests did compile the project).
- Linux/Arch build and tests have not yet been run.

### Commit history
- Initial foundation: `8a6a6bbd239b51eb90f1f253e66e2744e62ca68e`.
- Follow-up Rust formatting and Display formatting fix: `db5806c9ee70ae9227f3c9073490a92db57d8471`.
- Follow-up loader formatting: `055258d225898405156588bc527592e98c7c3207`.
- Follow-up CLI test formatting: `cc5ddb4bb78671c404bb6745054378bd69730fc8`.

### Next
Pull the latest `main` on Windows and run the commands in `SESSION_HANDOFF.md`. Fix any compiler, test, or lint issues before marking Phase 1 verified.

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
