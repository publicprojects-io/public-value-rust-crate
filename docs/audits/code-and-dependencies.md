# Code and dependency audit

Audit date: 2026-11-19. Scope: first-party Rust source, manifest, lockfile, tests, and CI.

## Panic and error handling

Every first-party `expect` is one of three documented cases: the crate-wide same-currency invariant,
decimal conversion of a finite realistic input, or arithmetic division/overflow already described
in the function's `# Panics` section. Changing these to a public error type would contradict
`spec/architecture.md`, which explicitly requires invariant-backed `expect` calls. There are no
first-party production `unwrap` calls; occurrences are in doctests and tests.

## Safety and I/O

There are no first-party `unsafe` blocks, date/time calculations, file operations, network calls,
or environment access in the library. The demo binary reads command-line arguments only. Weekly CI
runs RustSec and cargo-deny checks.

## Arithmetic

Counts stay `u32` where the source domain uses bounded human/service counts. Multiplication that
could overflow is delegated to `rust_decimal`/`rusty_money` and its `Result` is checked. The few
integer products have explicit domain-sized operands or checked relationships. Zero-denominator
panic paths are documented and edge-tested. Money never passes through binary floating point;
only reporting ratios and formulas needing roots/logarithms use `f64`.

## Dependencies

The three runtime dependencies are all required: `rusty-money` provides `Money`, `rust_decimal`
provides formula parameters and exact arithmetic, and `rust_decimal_macros` constructs exact
literals in public examples. Default Cargo requirements are compatible caret constraints, allowing
non-breaking fixes while the lockfile makes CI reproducible. Development-only Criterion and
Proptest support benchmarks and property tests. `cargo-deny` rejects wildcard, unknown-registry,
unknown-git, yanked, and unapproved-license dependencies.

## Allow annotations

The three library Clippy allowances are narrow and justified inline: a five-boolean source
framework and two realistic bounded-count `f64`→`u32` conversions. The benchmark harness allows
missing docs because Criterion generates public harness symbols that are not library API. No
library-wide lint suppression is present.

## Coverage and API use

`bin/test-coverage` enforces 100% function, line, and region coverage across first-party source. Public
items are exercised by source worked examples, integration tests, doctests, property tests, the
demo, and representative benchmarks. A public library cannot prove downstream use of every API;
documentation and source mapping, rather than speculative removal, govern the stable surface.
