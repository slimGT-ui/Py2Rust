# Py2Rust Roadmap

This file is the source of truth for project progress. Status must reflect verified work, not intentions.

## Status vocabulary
- **NOT STARTED** — no implementation work.
- **PLANNED** — scoped, not started.
- **IN PROGRESS** — actively being implemented.
- **BLOCKED** — waiting on an unresolved dependency/decision.
- **IMPLEMENTED** — code exists, verification may be incomplete.
- **VERIFIED** — relevant tests/checks passed.
- **DEFERRED** — intentionally postponed, reason recorded.

## Current state — 2026-11-20
- Current phase: Phase 1 — Rust foundation.
- Current milestone: `v0.1.0 — Foundational MVP`.
- Implementation: initial Cargo package, CLI, input loader, tests, and CI workflow committed.
- Windows local verification: rustfmt check passed, Clippy passed with warnings denied, and all 10 unit/integration tests passed.
- Remaining verification: manual valid/missing/directory CLI smoke tests, Linux local build, and a green GitHub Actions run.
- Repository: `slimGT-ui/Py2Rust`.
- Windows preflight: Rust/Cargo 1.100.0, stable `x86_64-pc-windows-msvc`, MSVC 19.44.35230, Git 2.55.0.
- Next action: pull the implementation and run formatting, lint, and tests on Windows.

## Phase 0 — Documentation and project setup
**Status: VERIFIED (documentation committed and fetched from GitHub).**

Goals: define scope, roadmap, architecture, compatibility vocabulary, workflow, and session handoff. Confirm repository and keep documentation in the repository.

Exit criteria: documentation committed; repository metadata recorded; workspace plan agreed.

## Phase 1 — Rust foundation
**Status: IN PROGRESS (Windows formatting, Clippy, and automated tests verified; cross-platform CI and manual smoke tests pending).**

### Scope
- Confirm local clone and working-tree state and Rust toolchain.
- Initialize a minimal Cargo project and CLI binary.
- Choose and document the Rust toolchain policy.
- Configure rustfmt, clippy, tests, and CI.
- Implement a minimal CLI with actionable errors.
- Accept one input `.py` path and report invalid paths/read failures.
- Keep translation and parsing out of scope.

### Acceptance criteria
- Project builds with the documented toolchain.
- CLI exposes help/version behavior.
- CLI distinguishes missing path, non-file path, and read failure.
- Expected user-input failures do not panic.
- Unit/integration tests cover CLI and file-loading behavior.
- Formatting, lint, and test commands pass locally and in CI.
- Documentation records actual commands and results.

### Subtasks
- **1.0 — Local environment preflight: COMPLETE on Windows.** Confirmed Git 2.55.0, Rust/Cargo 1.100.0, stable MSVC target, and MSVC 19.44.35230. Clone: `C:\programming\Py2Rust`, branch `main`, clean before implementation.
- **1.1 — Minimal Cargo package and CLI: IMPLEMENTED; Windows automated checks VERIFIED.** One package, standard-library-only loader, CLI help/version, explicit not-yet-translated message, tests, stable toolchain file, and Windows/Linux CI.
- **1.2 — Windows verification: PARTIALLY COMPLETE.** User ran `cargo fmt --all`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace`. Formatting and Clippy completed successfully; all 10 tests passed (5 unit + 5 integration). `cargo run -- --help` and `--version` both returned expected output.
- **Remaining — 1.3: Commit/push rustfmt changes from the user's working tree, obtain green Windows/Linux CI, run manual input smoke tests, and verify on Arch Linux.**

## Phase 2 — Python parser and AST
**Status: PLANNED**

Evaluate a maintained parser; initial candidate is `rustpython-parser`. Check Python grammar versions, AST coverage, source spans, diagnostics, maintenance, license, and API stability.

Cover in increments: modules/statements, assignments, literals/names, operators/comparisons, calls, functions, conditionals, loops, returns, lists, tuples, dictionaries.

Exit: parser tests, AST inspection/debug output, source-located syntax and unsupported-feature diagnostics.

## Phase 3 — First Rust code generator
**Status: PLANNED**

Create a separated generation layer (and IR when justified). Translate basic literals, arithmetic, comparisons, known local bindings, simple conditionals, functions, returns, and limited `print`. Output must be deterministic and readable.

Exit: snapshot tests pass and generated examples pass `cargo check`; unsupported cases fail with actionable diagnostics.

## Phase 4 — Semantic analysis and type inference
**Status: PLANNED**

Build scopes/symbol tables; track assignments and signatures; infer conservatively; handle branches and numeric conversions; detect conflicts and unknown types. Define mutability, ownership, borrowing, and aliasing policies. Never guess a type without sufficient evidence.

Exit: tests for scopes, reassignment, signatures, control flow, and ambiguity; useful diagnostics.

## Phase 5 — Collections and built-ins
**Status: PLANNED**

Candidate mappings: list → `Vec<T>`, dict → `HashMap<K,V>`, tuple → Rust tuple, set → `HashSet<T>`. Add indexing, iteration, membership, and selected methods. Account for heterogeneous values, equality, hashing, mutation, and Python error behavior.

Exit: nested collection tests and explicit handling of dynamic/unsupported cases.

## Phase 6 — Object-oriented Python
**Status: PLANNED**

Classes, instances, `__init__`, attributes, methods, `self`, selected inheritance patterns, special methods, static/class methods, and properties. Rust structs/traits/enums do not directly reproduce the entire Python object model.

Exit: supported patterns documented and tested; dynamic class behavior diagnosed.

## Phase 7 — Standard library
**Status: PLANNED**

Incremental adapters for selected APIs in `pathlib`, `os`, `sys`, `json`, `datetime`, `re`, `collections`, file I/O, and exceptions. Document Rust dependencies and behavioral differences for every supported API.

## Phase 8 — Modules, packages, whole projects
**Status: PLANNED**

Resolve local imports; build dependency graph; convert multiple files; generate Cargo layout and manifest; map Python dependencies to Rust crates/runtime adapters; report unsupported external dependencies and import cycles.

Exit: a multi-module example produces a buildable Rust project.

## Phase 9 — Advanced Python
**Status: PLANNED**

Lambdas/closures, decorators, iterators, generators/yield, comprehensions, context managers, try/except/finally, async/await, reflection, dynamic imports, descriptors, metaclasses, monkey-patching. Split into smaller milestones based on evidence from earlier phases.

## Phase 10 — Compatibility runtime
**Status: PLANNED**

Add Rust runtime helpers only where needed for selected Python semantics: dynamic values, truthiness/comparisons, dynamic containers/attributes, exception behavior, and difficult language features. Runtime usage must be visible in reports; it is not a blanket full-compatibility guarantee.

## Phase 11 — Behavioral equivalence and reliability
**Status: PLANNED**

Build differential tests that run Python and generated Rust on shared inputs; compare output, exit codes, and selected errors. Add regression tests, fuzz boundaries where useful, and test supported platforms/toolchains.

Exit: repeatable differential harness and evidence-based compatibility claims.

## Phase 12 — Production tooling
**Status: PLANNED**

Stable CLI/config, project reports, dry-run/explain modes, source maps, incremental conversion/cache, adapter/plugin architecture, release automation, docs, contribution guide.

## MVP — v0.1.0
Required:
- CLI accepts one Python file.
- Parser handles documented grammar subset.
- Basic literals, expressions, known-type variables.
- Arithmetic/comparisons, if/else, limited loops.
- Simple functions and returns.
- Limited print mapping.
- Rust source generation.
- Generated output passes `cargo check`.
- Automated parser/translation tests.
- Clear diagnostics for unsupported or ambiguous code.

Explicit non-goals: arbitrary third-party packages; full dynamic Python semantics; all classes/decorators/metaclasses; guaranteed ownership inference for arbitrary programs; perfect conversion of every Python program.

## Roadmap maintenance
After each meaningful session: update statuses from actual checks; add discovered tasks/blockers; update changelog and handoff; state the exact next action. Do not mark unverified work complete.
