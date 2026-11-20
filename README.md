# Py2Rust

Python-to-Rust transpiler focused on gradually expanding Python compatibility and generating native Rust code.

**Status (2026-11-20):** Work in progress. Phase 1 — Rust foundation.

## Vision

Translate individual Python files and eventually multi-file Python projects into Rust while preserving observable behavior as closely as practical. The goal is broad practical Python support, not a text-based syntax replacer.

Perfect conversion of every possible Python program is not promised. Python is dynamic and reflective; Rust is statically typed and has different ownership, memory, and execution models. Some constructs may need annotations, a compatibility runtime, manual intervention, or remain unsupported.

## Current milestone

- Milestone: `v0.1.0 — Foundational MVP`
- Phase: 1 — Rust foundation
- Current implementation: minimal CLI and input-file loader; local verification pending
- Current limitation: this build validates and reads one `.py` file. It does not parse or translate Python yet.

## Build and usage

Requires the stable Rust toolchain with `rustfmt` and `clippy`.

```powershell
cargo build
cargo run -- --help
cargo run -- --version
cargo run -- path\to\program.py
```

The current CLI validates that the input exists, is a regular file, has a `.py` extension, and can be read as UTF-8. It explicitly reports that translation is not implemented yet.

## Quality checks

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

CI runs the same checks on Windows and Linux.

## Documentation

- [Roadmap](docs/ROADMAP.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Compatibility policy](docs/COMPATIBILITY.md)
- [Development workflow](docs/DEVELOPMENT.md)
- [Decision log](docs/DECISIONS.md)
- [Changelog](docs/CHANGELOG.md)
- [Session handoff](docs/SESSION_HANDOFF.md)

## Engineering principles

- No silent semantic changes.
- No false success status.
- Never execute input Python as part of static translation.
- Keep stages independently testable.
- Tie compatibility claims to tests and documented behavior.
- Never claim a check passed unless it was actually run.
