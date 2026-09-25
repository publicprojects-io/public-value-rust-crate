# Porting a new topic

This crate's 64 items are each ported from one topic in
[`public-value-metrics`](https://github.com/public-value-metrics/public-value-metrics). If that
source book adds a topic, or you want to port one this crate skipped, follow this process.

## 1. Read the source topic in full

```
~/git/public-value-metrics/public-value-metrics/locales/en-gb-oxendict/topics/<slug>/index.md
```

Read the whole file, not just "The maths" section — the worked example and pitfalls sections often
contain the specific numbers your doctest needs and the edge cases your API should guard against.

## 2. Pick a shape

[`spec/architecture.md`](../../spec/architecture.md) defines four shapes:

- **Shape A** — a direct formula with 4 or fewer inputs → a plain function.
- **Shape B** — a multi-step formula with named intermediates → composed functions, or a result
  struct if the source doc's worked example names the intermediates individually.
- **Shape C** — a small bundle of related metrics reported together, never collapsed into one
  number → a struct with one field or method per metric.
- **Shape D** — a qualitative framework or structure, not arithmetic → a struct modelling the
  structure, with only the methods the source doc's "maths" section actually describes as
  decidable.

Pick whichever fits after reading the actual topic — don't force a formula-shaped API onto a
framework-shaped topic.

## 3. Add it to the right module

Find the topic's category in [`spec/topics.md`](../../spec/topics.md) and add your function or type
to that category's `src/<module>.rs`. Reuse the shared newtypes in [`src/units.rs`](../../src/units.rs)
(`Money`, `Ratio`, `Percentage`) rather than plain `f64`, and reuse another module's function
directly (via `use crate::<module>::...`) if the exact formula already exists elsewhere in the
crate — several topics share a formula shape (e.g. `wellbeing-valuation` and
`wellbeing-adjusted-life-years` both port the same WELLBY arithmetic; the second just reuses the
first's functions rather than redefining them).

## 4. Write the doc comment

One-line summary, a short "why it matters" clause condensed from the source (not copied verbatim),
a `# Examples` doctest reproducing a real number from the source doc's worked example, `#[must_use]`
on the item, and `# Panics`/`# Errors` sections only where a real panic or `Result` exists.

If the source doc's own worked example contains an arithmetic error (this has happened — check your
numbers independently, e.g. with a quick Python calculation, rather than trusting a stated
approximation), use the mathematically correct figure in your doctest and note the discrepancy in
one short comment rather than silently reproducing the wrong number.

## 5. Add the topic's test

Add one `#[test]` to that category's `tests/comprehensive_group<N>.rs` file, reproducing the source
doc's worked example(s) to a sensible float tolerance (`Decimal` values usually want
`.round_dp(n)` before an exact `assert_eq!`, since converting an `f64` percentage or rate into a
`Decimal` via `from_f64_retain` carries binary floating-point residue).

## 6. Validate

```sh
cargo clippy --all-targets -- -D warnings
cargo test
cargo doc --no-deps
```

All three must be clean before the topic is done. A common first-pass clippy failure is
`clippy::doc_markdown` on a proper noun in a doc comment (e.g. `GiveWell`, `SonarQube`) — wrap it in
backticks.
