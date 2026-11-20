# Py2Rust Architecture

## Goal
A maintainable translation pipeline with independent, testable stages. Parsing, semantic analysis, type decisions, and Rust formatting must not collapse into one large function/module.

## Proposed pipeline
```
Python source / project
  -> loader and project discovery
  -> Python parser
  -> AST with source spans
  -> semantic analysis (symbols, scopes, imports, control flow)
  -> type analysis and inference
  -> translation IR
  -> Rust code generation
  -> generated Cargo project + runtime/dependencies
  -> cargo fmt / check / test
  -> conversion report
```

## Components and responsibilities

### CLI
Arguments, configuration, file/project mode, diagnostics, summaries. No translation logic.

### Loader and project discovery
Read files, normalize paths, discover modules, preserve source identity/spans. Static analysis must not execute the input Python program.

### Parser
Initial candidate: `rustpython-parser`, pending evaluation. Parse source, preserve locations, distinguish syntax errors from valid-but-unsupported constructs. Keep a replaceable adapter boundary.

### Semantic analyzer
Build symbol tables and lexical scopes; resolve names/imports where possible; track assignments/control flow; detect unsupported dynamic behavior. Does not emit Rust strings.

### Type system
Represent known, inferred, union, generic, unknown, and dynamic types as needed. Infer conservatively. Respect and validate annotations. Model constraints relevant to Rust. Unknown is not the same as dynamic or unsupported. Do not force a Rust type when evidence is insufficient.

### Translation IR
Represent validated semantics independently of Python AST and Rust syntax. Make mutability/ownership choices explicit where possible. Allow validation passes before code generation.

### Rust generator
Generate deterministic, readable Rust from validated IR. Manage imports and helper calls. Formatting is separate from semantic decisions. Preserve source mapping where practical.

### Compatibility runtime
A separately tested/versioned Rust library for selected Python-like behavior that cannot be translated directly. Runtime usage must be visible in reports.

### Validation/reporting
Classify parser, unsupported-feature, analysis, type, generation, and Rust compiler failures. Writing a file is not success; report whether generated Rust validation actually passed.

## Proposed workspace shape
```
py2rust/
  Cargo.toml
  crates/
    cli/
    core/
    parser/
    analyzer/
    type_system/
    codegen/
    runtime/
  docs/
  tests/
  examples/
```
Do not create empty crates solely to match the diagram. Begin with the smallest useful workspace and split components when implementation justifies it.

## Parser evaluation criteria
Python grammar/version coverage, AST completeness, source spans, diagnostics, maintenance, license, API stability, and ability to represent required constructs. A parser does not provide semantic analysis or translation.

## Type inference policy
Infer from literals and straightforward expressions; propagate through simple assignments and calls; validate annotations; track branch possibilities; handle mutation/aliasing carefully. Ambiguous cases require diagnostics, annotations/configuration, or a tested runtime representation.

## Ownership and borrowing
Translation must account for moves/copies, mutation, aliasing, lifetimes, closures, shared mutable object graphs, and function boundaries. Ownership is a semantic design concern, not a last-minute text-generation fix.

## Diagnostics
Each diagnostic should include stable code, severity, path, line/column or span, concise message, and remediation when possible.

Suggested namespaces:
- `PY_PARSE_*`
- `PY_UNSUPPORTED_*`
- `ANALYSIS_*`
- `TYPE_*`
- `RUSTGEN_*`
- `RUSTCHECK_*`

## Testing layers
Unit tests per pass; snapshot tests for deterministic Rust output; integration tests from source file to generated project; compiler tests using cargo; differential tests comparing Python and Rust; regression tests for every fixed bug.

## Invariants
1. No silent semantic changes.
2. No false success status.
3. Never execute source Python as part of static translation.
4. Deterministic output for same input/config/tool versions.
5. Unsupported constructs get actionable diagnostics.
6. Stages remain independently testable.
7. Compatibility claims are tied to tests and documented scope.
