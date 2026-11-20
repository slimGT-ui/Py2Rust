# Development Workflow

## Session start
1. Read this file, `SESSION_HANDOFF.md`, `ROADMAP.md`, and latest `CHANGELOG.md` entry.
2. Inspect actual repository state before editing.
3. Confirm the goal, scope, acceptance criteria, and affected components.
4. Read relevant implementation/tests before changing code.

## Task workflow
1. Implement one coherent change at a time.
2. Add/update tests with the feature.
3. Run targeted tests first.
4. Run formatting, linting, and broader tests as appropriate.
5. Inspect the complete diff for accidental changes.
6. Update relevant documentation in the same work session.
7. Record exact commands and their actual results.
8. Mark roadmap work implemented/verified only when evidence supports it.

## Definition of done
A task is done only when intended behavior exists, relevant tests are added/updated, checks were actually run, results are documented, limitations/follow-ups are recorded, and the diff was reviewed. If a check could not be run, state that; do not mark it verified.

## Mandatory documentation maintenance
After each meaningful code/architecture change, review:
- `CHANGELOG.md`: chronological details of what changed.
- `ROADMAP.md`: status and next action.
- `ARCHITECTURE.md`: update when component boundaries/contracts change.
- `COMPATIBILITY.md`: update when supported Python behavior changes.
- `DECISIONS.md`: record important technical decisions.
- `SESSION_HANDOFF.md`: exact resumable project state.

Not every document must change every time, but changelog and handoff must be checked and updated every meaningful session.

## Git conventions
Suggested prefixes: `docs:`, `feat:`, `fix:`, `test:`, `refactor:`, `build:`, `ci:`, `chore:`. Keep commits focused. Never claim a commit exists unless GitHub confirms it.

## Planned baseline checks
Once workspace exists, likely checks:
```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```
For generated projects, also run `cargo check` and `cargo test`. Adjust commands to actual workspace/features. Never report commands as run when they were not.

## Handoff requirements
At session end record: phase/milestone; implemented work; verified checks and outputs; changed files; decisions; blockers; exact next task; work that must not be repeated.

## Collaboration
Explain unfamiliar compiler/Rust concepts before relying on them. Prefer small reviewable changes. Inspect files rather than guessing. Do not invent APIs/dependencies/repository state. Ask only when a missing detail blocks safe progress. User runs local commands when local environment access is required.
