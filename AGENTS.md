# Agent guide: public-value

This is a Rust library crate of public value calculations (cost-benefit analysis, SROI, KPIs, the
Human Development Index, and more), each ported with a worked example from a source book. If you
are an AI coding agent working in this repository, read this file first.

## What this crate is

75 public items across 8 modules (`src/foundations.rs` through `src/delivery_connection.rs`), plus
a shared `src/units.rs` of `Money`/`Ratio`/`Percentage` newtypes. 64 of those items each correspond
to one topic in [`public-value-metrics`](https://github.com/public-value-metrics/public-value-metrics)'s
`en-gb-oxendict` locale; a further 11 are independently researched extensions (see
`spec/topics.md`'s "Extensions" section) that complete a connection a source topic references but
doesn't implement. `spec/architecture.md` is the single source of truth for how the crate is
designed; `spec/topics.md` maps every ported topic to its module and symbol, and lists the
extensions separately. Read both before adding or changing a public item.

## Build, test, lint

```sh
cargo build
cargo test                                  # unit + integration + 107 doctests
cargo clippy --all-targets -- -D warnings   # must be clean; clippy::pedantic is denied crate-wide
cargo doc --no-deps                         # must build without warnings; missing_docs is denied
```

Run all four before considering any change to `src/` or `tests/` complete. `cargo clippy` alone is
not sufficient — doctests and the plain `--lib` clippy pass can both be clean while `--all-targets`
still fails on something in `tests/`.

## Conventions (full detail in `spec/architecture.md`)

- **`Money` is `rusty_money::Money` used directly — not wrapped.** `pub type Money =
  rusty_money::Money<'static, iso::Currency>;` in `src/units.rs`. Construct one with
  `Money::from_decimal(dec!(450_000), iso::USD)`, never from an `f64` literal. Every arithmetic or
  comparison call (`.add`, `.sub`, `.mul`, `.div`, `.gt`, `.lt`, ...) is a one-line call straight
  through to `rusty_money::Money`'s own method — never reintroduce a wrapper type or a generic
  `T: FormattableCurrency` parameter on this crate's own functions. `Ratio` and `Percentage` stay
  `f64` because several formulas need transcendental functions.
- **Every `Money` is tagged `USD` internally, even in GBP-labelled worked examples.**
  `rusty_money::Money` requires a `Currency`; this crate fixes it to `USD` for every value so two
  `Money` values built anywhere in the crate can always be added or compared without a runtime
  currency-mismatch error. Doc comments still cite the real pound figures the source material
  publishes — only the underlying type's currency tag is fixed, not the prose. Never construct a
  `Money` with a different currency.
- **`rusty_money::Money::add`/`sub`/`mul`/`div` return `Result`, not a plain `Money`.** They check
  the two operands share a currency. Since this crate always uses `USD`, that check can never
  actually fail — `.expect("...")` the result with a short reason (documented as `# Panics` on the
  enclosing function; see `src/units.rs`'s `CURRENCY_INVARIANT` constant for the standard
  add/sub/compare message), don't `?`-propagate a `MoneyError` your function will never return.
  Comparisons (`<`, `>`) aren't operators either — use `.gt(&other)`/`.lt(&other)` (also
  `Result`-returning); `==` does work, since `rusty_money::Money` derives `PartialEq`. Use
  [`crate::units::money_ratio`] to divide two `Money` amounts into a `Ratio`.
- **`Money`'s `Display` shows a currency symbol and rounds correctly.** It delegates to
  `rusty_money::Money`'s own locale-aware formatting (`$714.29`, not a bare `714.29`), which rounds
  half-to-even rather than truncating.
- **Converting `f64` to `Decimal` leaves binary-floating-point residue.** `Decimal::from_f64_retain`
  on a value like `0.6` or `0.005` does not land on an exact decimal; a downstream `assert_eq!` will
  fail by a sub-cent amount. Round with `.round_dp(n)` before an exact comparison, or use a
  tolerance-based `assert!((a - b).abs() < epsilon)`.
- **One module per category, one comprehensive test file per module.** `tests/comprehensive_group1.rs`
  through `comprehensive_group8.rs` correspond in order to the modules listed in `spec/topics.md`.
- **Every public item needs a doctest reproducing a real worked-example number**, not an invented
  one — check the source topic file before writing the assertion. If the source doc's own worked
  example contains an arithmetic error (this has happened at least twice — a distributional-weight
  calculation and an annuity-factor figure both didn't reconcile on independent recomputation), use
  the mathematically correct number and leave a one-line comment noting the discrepancy, rather than
  encoding the source's mistake as a test target.
- **`#[must_use]` on every pure function/method that returns a value.** `# Panics` only where a real
  panic path exists (Decimal division by zero, an `.expect()` on a conversion); `# Errors` only
  where a function returns `Result`.
- A common first-pass `clippy::pedantic` failure is `clippy::doc_markdown` on a proper noun in a doc
  comment (`GiveWell`, `SonarQube`, `GuideStar`) — wrap it in backticks.

## Scope boundaries

- The source repository (`~/git/public-value-metrics/public-value-metrics` on the machine this was
  built on) also has locale-translation infrastructure and a SvelteKit documentation site
  (`*.github.io`). Neither is in scope for this crate — it is a calculation library, not a book or a
  website. Don't add i18n or a web frontend here without being asked.
- Don't add a numeric type or struct "for consistency" that no topic's source doc actually needs —
  several topics deliberately reuse another topic's functions rather than each getting bespoke API
  surface (e.g. `wellbeing-adjusted-life-years` reuses `economic_appraisal::wellbys`).

## Adding a new topic

See `docs/tutorials/porting-a-new-topic.md` for the full walkthrough (read the source doc, pick one
of the four modelling shapes, add the item to the right module, write the doctest, add the test,
validate).
