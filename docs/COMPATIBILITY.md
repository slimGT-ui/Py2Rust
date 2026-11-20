# Python Compatibility Policy

## Meaning of support
Parser acceptance does not mean correct translation. Track each feature using these states:
- **Not parsed:** parser cannot represent the syntax/version.
- **Parsed:** syntax accepted, no translation guarantee.
- **Analyzed:** semantic checks implemented.
- **Translated:** Rust output generated.
- **Runtime-backed:** output depends on Py2Rust runtime.
- **Verified:** documented cases pass relevant tests.
- **Unsupported:** rejected with a diagnostic.
- **Unknown:** not evaluated.

Advertise support only with an exact scope and known limitations.

## Principles
- Preserve observable behavior where practical.
- Do not silently replace behavior with something merely similar.
- Explain semantic differences.
- Prefer diagnostics over guesses.
- Add compatibility tests for every claimed behavior.
- Distinguish compilation success from behavioral equivalence.

## Difficult Python features
Dynamic reassignment; reflection/introspection; eval/exec; monkey-patching; dynamic imports; metaclasses/descriptors; arbitrary decorators; runtime class/instance modification; duck typing; complex exceptions; generators/coroutines; shared mutable object graphs and aliasing. Some may be partial/runtime-backed; others may remain unsupported.

## Rust semantic differences to account for
Ownership/borrowing/lifetimes; mutability; trait/generic constraints; Result/Option vs exceptions; integer overflow and numeric conversions; string indexing; hashing/equality for map/set keys; iterator behavior; async/concurrency runtime differences.

## Initial compatibility matrix
All entries below are **NOT STARTED**; this is a plan, not a feature claim.

| Python feature | Intended representation | Status |
|---|---|---|
| Literals | Rust primitive/string types | NOT STARTED |
| Arithmetic | Direct where semantics match | NOT STARTED |
| Variables | Inferred/annotated bindings | NOT STARTED |
| if/else | Rust conditionals | NOT STARTED |
| for/while | Supported patterns first | NOT STARTED |
| Functions | Typed Rust functions | NOT STARTED |
| Lists | Vec<T> for compatible homogeneous cases | NOT STARTED |
| Dictionaries | HashMap<K,V> for compatible cases | NOT STARTED |
| Tuples | Rust tuples | NOT STARTED |
| Classes | Structs/traits/runtime as appropriate | NOT STARTED |
| Exceptions | Explicit error model/runtime | NOT STARTED |
| Decorators | Partial/runtime-backed | NOT STARTED |
| Generators | Iterator/state-machine/runtime | NOT STARTED |
| Async | Rust async/runtime mapping | NOT STARTED |
| Reflection | Limited/runtime-backed or unsupported | NOT STARTED |

Update only when implementation and tests justify a status change.
