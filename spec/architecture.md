# Architecture

This is the single source of truth for how this crate is designed and organized. If code and this
file disagree, this file wins until the code is fixed (or the file is updated by a deliberate
decision).

## Source of the domain content

The domain content (definitions, formulas, worked examples) is ported from
[`public-value-metrics`](https://github.com/public-value-metrics/public-value-metrics), specifically
the canonical `en-gb-oxendict` locale:

```
~/git/public-value-metrics/public-value-metrics/locales/en-gb-oxendict/topics/<slug>/index.md
```

That repository documents 64 topics across 8 categories (8 topics each). See [topics.md](topics.md)
for the full mapping from source topic to Rust module and type. Each Rust doc comment should credit
its source topic and condense (not copy verbatim) the definition, formula, and worked example.

The `*.github.io` SvelteKit site and locale-translation infrastructure in that source repo are
explicitly **out of scope** for this crate.

## Crate shape

- `src/lib.rs` — library root, declares one `pub mod` per category module plus `pub mod units`.
- `src/units.rs` — shared newtypes used across every category (below).
- `src/<category>.rs` — one module per category, containing the 8 topics in that category.
- `src/main.rs` — a thin demo binary that exercises a handful of library calculations. Not the
  primary deliverable; the library is.
- `tests/comprehensive_group<N>.rs` — one integration test file per category (`group1` =
  `foundations` through `group8` = `delivery_connection`, same order as `topics.md`), each
  exercising every topic in that category against the source doc's worked example(s).

## Shared newtypes (`src/units.rs`)

Plain `f64`/`u32` throughout would let a ratio and a percentage and a money amount all be swapped by
accident. Instead:

- `Money(rusty_money::Money<'static, iso::Currency>)` — a currency amount, backed by
  [`rusty_money`](https://docs.rs/rusty-money) rather than a bare `f64` or `Decimal`, so repeated
  addition and percentage scaling of money stays exact. `rusty_money::Money` requires every value
  to carry a `Currency`; this crate fixes that to `USD` for every value it constructs (`Money::new`
  always tags `iso::USD`), as a single internal working currency, so any two `Money` values built
  anywhere in the crate can always be added or compared without a runtime currency-mismatch error.
  Several worked examples are stated in GBP in the source material (`public-value-metrics` is
  UK-focused) — the doc comments still cite the real pound figures HM Treasury and others publish,
  but the `Money` values those doctests construct are tagged `USD` like everything else; the tag is
  a `rusty_money` implementation requirement, not a currency-conversion claim. `Money` supports `+`,
  `-`, `*`/`/` by `Decimal` (scaling), and `*`/`/` by `u32` (headcounts) — these are `std::ops`
  operator impls on *our* wrapper that delegate to `rusty_money::Money`'s fallible `add`/`sub`/`mul`/`div`
  methods internally, `.expect()`-ing them (documented as `# Panics`), so call sites keep ordinary
  infix arithmetic rather than `Result`-chaining. `PartialOrd`/`Ord` are implemented the same way,
  delegating to `rusty_money::Money::compare`. Dividing `Money` by `Money` yields a plain `f64`
  `Ratio` (via `Money::ratio_to`, which is the one place a monetary figure is allowed to become a
  float, because a ratio is a reporting figure, not an amount anything gets added back to), not
  `Money`. Build `Money` values with the `rust_decimal_macros::dec!` literal macro
  (`Money::new(dec!(450_000))`), never from an `f64` — `rust_decimal` and `rust_decimal_macros`
  remain direct dependencies of this crate (as well as being `rusty_money`'s own internal decimal
  type) purely for that literal-construction and `Decimal`-typed function-parameter purpose.
- `Ratio(f64)` — a dimensionless ratio (e.g. an SROI ratio, a cost-effectiveness ratio). Has
  `Ratio::new`, `.value()`, and `Display` (`"1.44"`, `"1.44:1"` is left to callers who want that
  framing).
- `Percentage(f64)` — stored as a **fraction** (`0.0..=1.0`, though not clamped — some pitfall
  discussions in the source docs describe percentages that exceed 100%). `Percentage::from_percent`
  builds one from a `0.0..=100.0` value; `.as_fraction()` and `.as_percent()` read it back either
  way. `Display` renders as `"42.0%"`.

All three: `Debug, Clone, Copy, PartialEq, PartialOrd` (derived for `Ratio`/`Percentage`; for
`Money`, `PartialEq`/`Eq`/`Hash` are derived but `PartialOrd`/`Ord` are hand-implemented against
`rusty_money::Money::compare`, and arithmetic operators are hand-implemented against its fallible
methods — see above), `#[must_use]` on every method that returns a value, and full rustdoc with a
doctest. `Ratio`/`Percentage` constructors stay `const fn`; `Money`'s cannot be, since
`rusty_money::Money::from_decimal` isn't `const`.

No newtype for plain counts (people, deployments, years) — those stay `u32`/`f64` as appropriate;
wrapping them added no safety the source docs' arithmetic needed.

## Modelling a topic: four shapes

Not every topic is "plug numbers into a formula." Classify each topic into the closest of these four
shapes; don't force a formula-shaped API onto a framework-shaped topic.

### Shape A — direct formula

One function, few inputs, one numeric output. Prefer a plain function with named parameters over an
`Inputs`/`Result` struct pair when there are 4 or fewer inputs.

```rust
/// Cost per outcome: total programme cost divided by the number of beneficiaries who achieved the
/// defined outcome (not merely received a service).
///
/// Ported from `cost-per-outcome`.
///
/// # Panics
///
/// Panics if `beneficiaries_achieving_outcome` is zero.
///
/// # Examples
///
/// ```
/// use public_value::philanthropy_metrics::cost_per_outcome;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// let cost = cost_per_outcome(Money::new(dec!(450_000)), 630);
/// assert_eq!(cost.value().round_dp(2), dec!(714.29));
/// ```
#[must_use]
pub fn cost_per_outcome(total_programme_cost: Money, beneficiaries_achieving_outcome: u32) -> Money {
    total_programme_cost / beneficiaries_achieving_outcome
}
```

### Shape B — multi-step formula with named intermediates

Formulas with several dependent sub-calculations (HDI's three sub-indices, SROI's deadweight →
attribution → drop-off chain). Use a struct for the inputs when there are more than 4, and either
return a result struct with the named intermediates (when the source doc's worked example calls
them out individually, e.g. HDI's LEI/EI/II) or just the final value (when the source doc only cares
about the end number, e.g. SROI's ratio). Prefer the simplest thing that still lets a test assert on
the worked example's stated intermediate figures where the source doc states them.

### Shape C — multi-metric bundle

Topics that are a small set of related metrics reported together, not one number (DORA's four/five
metrics, the Green Book's five cases). Model as a struct with one field or method per metric; don't
try to collapse it into a single score the source doc doesn't itself compute.

### Shape D — qualitative framework

Topics that are a structure or test, not arithmetic (the strategic triangle, theory of change, logic
model, outcomes vs outputs). Model the structure as a struct (booleans, enums, `Vec<String>` for
open-ended lists like preconditions/assumptions/indicators) and add the one or two methods the source
doc's "maths" section actually describes as decidable (e.g. `StrategicTriangle::passes(&self) ->
bool` because the source doc states "proceed only if all three hold"). Don't invent scoring the
source doc doesn't define.

## Naming

- Module: `snake_case` category name (`philanthropy_metrics`), see [topics.md](topics.md).
- Primary type per topic: `PascalCase` of the topic slug (`cost-per-outcome` has no single struct
  since it's Shape A; `social-return-on-investment` → `SocialReturnOnInvestment` if Shape B needs a
  struct, or just a free function `social_return_on_investment` if it doesn't).
- Free functions: `snake_case` of the topic slug.
- One module doc comment (`//!`) per file summarizing the category (from the source README's
  category description).
- One doc comment per public item, structured: one-line summary, then (if useful) a short "why it
  matters" clause condensed from the source, then `# Examples` with a doctest reproducing a number
  from the source doc's worked example. Add `# Panics` only where a real panic path exists (e.g. an
  assertion on a precondition), and `# Errors` only where a function returns `Result`. `#[must_use]`
  on every pure function/method that returns a value.
- Inline `//` comments only for non-obvious logic (e.g. *why* deadweight is applied before
  attribution in SROI, or why HDI uses a geometric rather than arithmetic mean) — not restating what
  the code does.

## Lints

`Cargo.toml` sets `clippy::pedantic` and `missing_docs` to `deny`. Every module must compile clean
under:

```
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps
```

Prefer fixing the code over `#[allow(...)]`. An `#[allow]` is acceptable only with a `// why` comment
explaining why the lint's concern doesn't apply here (e.g. `clippy::cast_precision_loss` on a
beneficiary count that will never realistically exceed 2^52).

## Tests

Each `tests/comprehensive_group<N>.rs` file has one `#[test]` per topic in that category (so 8 tests
per file, 64 total), each reproducing the worked example's numeric result(s) from the source doc to
within a small float tolerance (`1e-6` relative, or looser — `0.01` absolute — where the source doc's
own worked example rounds intermediate steps, as HDI's does). For Shape D (qualitative) topics, the
test constructs the worked example's scenario and asserts on the framework method's boolean/enum
result (e.g. the public-value worked example: two of three legs fail → `passes()` is `false`).
