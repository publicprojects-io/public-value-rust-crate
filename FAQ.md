# Frequently asked questions

## Why are GBP-labelled examples tagged USD internally?

`rusty_money::Money` requires a currency tag. The crate deliberately fixes every amount to one
internal working currency (`iso::USD`) so arithmetic cannot fail from mismatched tags. Prose keeps
the source's real GBP labels; the tag is an implementation invariant, not a conversion claim.

## Why is Rust 1.98 the minimum supported version?

The crate uses Rust 2024 and tests 1.98 as its declared MSRV. CI also checks the next minor release,
the pinned stable toolchain, and nightly. Raising the MSRV is a deliberate release decision.

## Can I add my own formula?

Application-specific formulas belong in the application and can compose this crate's public
functions. A generally useful public-value topic may be proposed here, but it needs a published
source, a real worked example, and an entry in `spec/topics.md`.

## Why are `Ratio` and `Percentage` floating point while money is decimal?

Money must add and scale exactly. Ratios need logarithms, roots, and other transcendental
operations, so `f64` is the practical reporting type. Tests use tolerances where appropriate.

## How do I contribute?

Read [CONTRIBUTING.md](CONTRIBUTING.md), `AGENTS.md`, and both files in `spec/`. Run `make ci` and
`make coverage` before opening a pull request.
