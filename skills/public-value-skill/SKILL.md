---
name: public-value-skill
description: Compute public value metrics (SROI, value for money, cost-effectiveness, the Human Development Index, cost of delay, and 59 others) using the `public-value` Rust crate's typed functions rather than recalling formulas from general training. Use when the user asks to compute a public-value metric, wire one into a Rust codebase, or check which of this crate's functions matches a governmental or social-sector calculation they need.
---

# Using the `public-value` crate

This crate (`src/*.rs`) is a typed, tested library of 75 public value calculations: 64 each ported
from one topic in the [`public-value-metrics`](https://github.com/public-value-metrics/public-value-metrics)
book, plus 11 independently researched extensions (QALYs, DALYs, the Gini coefficient, the
Inequality-adjusted HDI, Net Promoter Score — see `spec/topics.md`'s "Extensions" section). Prefer
it over deriving a formula from memory: every function's rustdoc reproduces a real worked example,
so you can sanity-check your own numbers against a known-correct one before wiring a new caller.

## Workflow

1. **Find the right function.** `spec/topics.md` maps every source topic to its module and
   primary symbol; `llms.json` gives the same index as structured JSON. If the user names a
   concept ("SROI", "value for money", "cost per outcome"), grep `src/*.rs` for the slug or read
   the relevant module's doc comment block — module docs list every topic they cover.
2. **Read the function's doctest before calling it.** The `# Examples` section on every public
   item is a real worked example, including which inputs are fractions versus percentages, and
   what a realistic magnitude looks like. A `Percentage` is *not* the same as a raw `f64` — use
   `Percentage::from_percent(40.0)` for "40%", not `0.4`, unless the function's doctest shows
   otherwise.
3. **Build `Money` values with `Money::from_decimal(dec!(...), iso::USD)`, never an `f64` literal.**
   `Money` is `rusty_money::Money` used directly (`use rusty_money::{Money, iso};`), not a wrapper —
   every value needs a currency, and this crate always uses `iso::USD` internally, even for
   GBP-labelled worked examples (see [`units`] module docs for why). `.add`/`.sub`/`.mul`/`.div`
   return `Result` (they check currencies match); `.expect("...")` it rather than propagating a
   `MoneyError` that can't actually occur here. Comparisons are `.gt(&other)`/`.lt(&other)`, not
   `<`/`>`; `==` works directly. Use [`units::money_ratio`] to divide two `Money` amounts into a
   `Ratio`.
4. **Compose across modules where the source book does.** Several real calculations span more
   than one topic — an SROI figure needs [`foundations::present_value`] for the year-2 discounting
   step; a natural-capital asset value needs [`economic_appraisal::annuity_factor`]. Check a
   function's rustdoc "see also" links (`[`crate::other_module::other_fn`]`) before assuming you
   need to write new arithmetic.
5. **Don't invent a missing input.** If a calculation needs a rate, proxy value, or threshold the
   user hasn't supplied (a discount rate, a WELLBY value, a deadweight percentage), ask, or use the
   crate's doctest default and say explicitly that you did — the same discipline the source book's
   own worked examples follow (e.g. citing "HM Treasury's 3.5% Green Book rate" rather than a bare
   number).
6. **If no existing function fits**, don't approximate with a nearby one — check whether the topic
   exists in the source book at `~/git/public-value-metrics/public-value-metrics/locales/en-gb-oxendict/topics/`
   (if available locally) and consider porting it (see `docs/tutorials/porting-a-new-topic.md`)
   rather than writing an unported, untested calculation inline.

## Common compositions

- **A business case**: [`foundations::net_public_value`] (what's displaced) →
  [`delivery_connection::cost_of_delay_per_week`] (urgency) →
  [`impact_measurement::sroi_ratio`] or [`economic_appraisal::benefit_cost_ratio`] (the net figure).
- **An appraisal-grade case**: [`economic_appraisal::FiveCaseModel`] →
  [`economic_appraisal::net_present_social_value`] →
  [`foundations::distributional_weight`]/[`weighted_benefit`] to judge who the value reaches.
- **Proving a claim, not asserting it**: net off [`foundations::net_additional_outcomes`]
  (deadweight) and [`foundations::net_additional_impact`] (displacement) against a real
  [`foundations::difference_in_differences`] counterfactual estimate before reporting a headline
  number.

## What not to do

- Don't collapse a [`performance_metrics::PublicValueScorecard`] or
  [`economic_appraisal::FiveCaseModel`]'s several fields into one score — both types exist
  specifically because the source topics warn against exactly that collapse.
- Don't report a gross figure (before [`impact_measurement::value_net_of_rate`] adjustments, or
  before [`foundations::net_additional_outcomes`]) as if it were the net one.
- Don't treat a function's worked-example inputs as the only valid inputs — they're a
  correctness check, not a whitelist.
