# Performance notes

The library performs in-memory scalar arithmetic and small slice traversals. Most functions are
constant time and allocate nothing. Slice-based scores, schedules, and inequality calculations are
linear in the number of supplied observations. IRR uses bounded bisection and is
`O(cash_flows × max_iterations)`.

Money arithmetic uses decimal operations to preserve financial accuracy; ratios use `f64` where
roots and logarithms are required. No function performs file, network, or database I/O. The
compiler is left to make inlining decisions; manual caches would add synchronization and stale-data
risk without removing meaningful work.

Run `make test-benchmarks` for representative Criterion measurements on the current machine. Benchmark
results are not committed because absolute timings are hardware- and load-dependent.
