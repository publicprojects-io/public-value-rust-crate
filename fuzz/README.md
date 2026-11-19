# Fuzzing

Install `cargo-fuzz` and run:

```sh
cargo fuzz run money_arithmetic
```

The target exercises `Money::add`, `Money::sub`, and `Money::mul` over arbitrary integer amounts
and factors. Arithmetic overflow is an expected `Err`; panics, crashes, and invariant violations
are failures.
