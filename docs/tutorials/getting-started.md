# Getting started

This tutorial builds a small business case using three calculations from three different modules,
the way a real appraisal usually draws from several of this book's topics at once.

## Add the dependency

```toml
[dependencies]
public-value = "0.1"
rust_decimal = "1"
rust_decimal_macros = "1"
```

`rust_decimal_macros` gives you the `dec!` literal macro, which is how every `Money` value in this
crate should be constructed — never from an `f64` literal, since binary floating point cannot
represent most decimal amounts exactly.

## Scenario

A council is appraising a £250,000 employment programme. You want three numbers: the programme's
Social Return on Investment ratio, its cost per outcome, and — because the programme is funded by a
grant with a statutory deadline — the cost of a delay to going live.

### 1. Social return on investment

```rust
use public_value::foundations::present_value;
use public_value::impact_measurement::{sroi_ratio, value_net_of_rate};
use public_value::units::{Money, Percentage};
use rust_decimal_macros::dec;

// 60 participants move into sustained employment, valued at £8,500/person/year.
let gross = Money::new(dec!(8_500)) * 60_u32;

// Deadweight (40% would have found work anyway) and attribution (30% of what remains is due to
// other agencies) both reduce the claim before it can be called impact.
let year1_impact = value_net_of_rate(
    value_net_of_rate(gross, Percentage::from_percent(40.0)),
    Percentage::from_percent(30.0),
);

// A second year of impact, discounted at the Green Book's 3.5% social discount rate.
let year2_impact = present_value(
    value_net_of_rate(year1_impact, Percentage::from_percent(30.0)), // 30% year-on-year drop-off
    dec!(0.035),
    1,
);

let total_impact = year1_impact + year2_impact;
let ratio = sroi_ratio(total_impact, Money::new(dec!(250_000)));
println!("SROI ratio: {ratio}"); // 1.44
```

### 2. Cost per outcome

```rust
use public_value::philanthropy_metrics::cost_per_outcome;
use public_value::units::Money;
use rust_decimal_macros::dec;

// Of 90 participants, 60 achieved the defined outcome (sustained employment).
let cost = cost_per_outcome(Money::new(dec!(250_000)), 60);
println!("Cost per outcome: {cost}");
```

Notice this is deliberately a different function from a naive "cost per participant" — see the
[`philanthropy_metrics`] module's rustdoc for why the distinction (outcome versus output) matters.

### 3. Cost of a delay

```rust
use public_value::delivery_connection::cost_of_delay_per_week;
use public_value::units::Money;
use rust_decimal_macros::dec;

// The programme is forecast to deliver £520,000/year of value once live.
let cod = cost_of_delay_per_week(Money::new(dec!(520_000)));
println!("Cost of delay: {cod}/week"); // every week of slippage costs roughly this much
```

## Next steps

- [`spec/architecture.md`](../../spec/architecture.md) explains the four ways a topic is modelled
  (a direct formula, a multi-step formula, a multi-metric bundle, or a qualitative framework) and
  the shared `Money`/`Ratio`/`Percentage` newtypes every module builds on.
- [`spec/topics.md`](../../spec/topics.md) maps all 64 topics to their module and primary
  type/function.
- The [README](../../README.md) lists every module's public items in one place.
