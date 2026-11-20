# Py2Rust

Python-to-Rust transpiler — a long-term open-source project.

**Status (2026-11-20):** planning/documentation initialized; implementation not started.

## Vision
Translate individual Python files and eventually multi-file Python projects into Rust while preserving observable behavior as closely as practical. The goal is broad practical Python support, not a text-based syntax replacer.

Perfect conversion of every possible Python program is not promised. Python is dynamic and reflective; Rust is statically typed and has different ownership, memory, and execution models. Some constructs may need annotations, a compatibility runtime, manual intervention, or remain unsupported.

## Current milestone
- Milestone: `v0.1.0 — Foundational MVP`
- Phase: 0 — documentation and repository setup
- Implementation: not started
- Next: initialize the Rust workspace after documentation is in place.

## Documentation
- [Roadmap](docs/ROADMAP.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Compatibility policy](docs/COMPATIBILITY.md)
- [Development workflow](docs/DEVELOPMENT.md)
- [Decision log](docs/DECISIONS.md)
- [Changelog](docs/CHANGELOG.md)
- [Session handoff](docs/SESSION_HANDOFF.md)

## Engineering principle
A feature is not complete because code was written. It is complete only when behavior, tests, limitations, and verification results are documented. Never claim a check passed unless it was actually run.
