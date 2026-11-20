# Architecture Decision Log

Record consequential decisions with date, status, context, choice, and consequences.

## D-001 — Broad transpiler direction
- Date: 2026-11-20
- Status: Accepted
- Context: A trivial syntax replacer is not the intended product, while perfect translation of arbitrary Python is unrealistic.
- Decision: Build a long-term Python-to-Rust transpiler targeting broad practical Python coverage.
- Consequences: Architecture includes parsing, semantic analysis, type inference, translation, runtime compatibility, and behavioral tests. Compatibility claims remain evidence-based.

## D-002 — Incremental milestones
- Date: 2026-11-20
- Status: Accepted
- Decision: Deliver in small verified milestones, beginning with v0.1.0 foundational MVP.
- Consequences: Later roadmap items are not marked implemented before they exist and pass checks.

## D-003 — Existing parser first
- Date: 2026-11-20
- Status: Accepted in principle; dependency evaluation pending
- Decision: Prefer a maintained parser over writing Python grammar from scratch; evaluate rustpython-parser first.
- Consequences: Verify grammar versions, AST coverage, spans, maintenance, license, and API before pinning dependency.

## D-004 — Implement transpiler in Rust
- Date: 2026-11-20
- Status: Accepted
- Decision: Rust is the implementation language for the transpiler.
- Consequences: Use Cargo and Rust tooling; begin with a small modular workspace.

## D-005 — Documentation is part of delivery
- Date: 2026-11-20
- Status: Accepted
- Context: The project must remain understandable across long breaks and chat sessions.
- Decision: After every meaningful change, update current state, verified work, decisions, blockers, and next steps.
- Consequences: Changelog, roadmap, and session handoff are required maintenance artifacts. Never mark unverified work complete.
