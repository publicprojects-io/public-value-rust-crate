# Development

## Requirements

- Rust 1.98 or newer with `rustfmt` and Clippy.
- `cargo-llvm-cov` for coverage.
- Optional: `cargo-deny`, `cargo-audit`, and `cargo-mutants` for their specialist checks.

## Everyday workflow

```sh
make ci
make test-coverage
make ci-minimum-supported-rust-version
```

Run one integration test with `cargo test --test comprehensive_group4`, one named test with
`cargo test citizen_satisfaction`, or a doctest suite with `cargo test --doc`.

When debugging a calculation, start from its source worked example and keep decimal money values
as `Decimal`; do not convert money to `f64`. `RUST_BACKTRACE=1 cargo test test_name -- --nocapture`
shows panic context and test output.

Before changing public items, follow the new-topic checklist in [CONTRIBUTING.md](CONTRIBUTING.md).
