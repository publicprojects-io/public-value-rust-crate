# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)

## Benchmark Results

The Criterion suite in `benches/public_api.rs` covers a representative hot calculation from every
category module plus the shared unit ratio. Run `make benchmarks`; Criterion writes statistical
estimates and HTML reports beneath `target/criterion/`. Benchmark output is deliberately generated
locally rather than committed because timings depend on CPU, load, and toolchain.

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)
