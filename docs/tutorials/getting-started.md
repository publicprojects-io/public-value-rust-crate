# Getting started

This tutorial builds a small business case using three calculations from three different modules,
the way a real appraisal usually draws from several of this book's topics at once.

## Add the dependency

```toml
[dependencies]
public-value = "1"
rusty-money = "0.5"
rust_decimal_macros = "1"
```

This crate's `Money` type is [`rusty_money::Money`](https://docs.rs/rusty-money) used directly, not
wrapped — construct one with `Money::from_decimal(amount, currency)`, where `amount` comes from the
`rust_decimal_macros::dec!` literal macro (never an `f64` literal, since binary floating point
cannot represent most decimal amounts exactly) and `currency` is a `rusty_money::iso` constant such
as `iso::USD`. Every `Money` value this crate constructs is tagged `USD` internally as a single
working currency — see [`public_value::units`](../../src/units.rs)'s module docs for why.

## Scenario

A council is appraising a £250,000 employment programme. You want three numbers: the programme's
Social Return on Investment ratio, its cost per outcome, and — because the programme is funded by a
grant with a statutory deadline — the cost of a delay to going live.

### 1. Social return on investment

```rust
use public_value::foundations::present_value;
use public_value::impact_measurement::{sroi_ratio, value_net_of_rate};
use public_value::units::Percentage;
use rust_decimal_macros::dec;
use rusty_money::{iso, Money};

// 60 participants move into sustained employment, valued at £8,500/person/year.
let gross = Money::from_decimal(dec!(8_500), iso::USD).mul(60_u32).unwrap();

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

let total_impact = year1_impact.add(year2_impact).unwrap();
let ratio = sroi_ratio(total_impact, Money::from_decimal(dec!(250_000), iso::USD));
println!("SROI ratio: {ratio}"); // 1.44
```

Every `rusty_money::Money` arithmetic method (`add`, `sub`, `mul`, `div`) returns a `Result`,
because it checks the two amounts share a currency. Since this crate always uses `iso::USD`
internally, that check can never actually fail here — `.unwrap()` (or `.expect("...")` with a note
of why, as this crate's own source does) is the right call, not `?`-propagating a `MoneyError` your
own function will never actually return.

### 2. Cost per outcome

```rust
use public_value::philanthropy_metrics::cost_per_outcome;
use rust_decimal_macros::dec;
use rusty_money::{iso, Money};

// Of 90 participants, 60 achieved the defined outcome (sustained employment).
let cost = cost_per_outcome(Money::from_decimal(dec!(250_000), iso::USD), 60);
println!("Cost per outcome: {cost}");
```

Notice this is deliberately a different function from a naive "cost per participant" — see the
[`philanthropy_metrics`] module's rustdoc for why the distinction (outcome versus output) matters.

### 3. Cost of a delay

```rust
use public_value::delivery_connection::cost_of_delay_per_week;
use rust_decimal_macros::dec;
use rusty_money::{iso, Money};

// The programme is forecast to deliver £520,000/year of value once live.
let cod = cost_of_delay_per_week(Money::from_decimal(dec!(520_000), iso::USD));
println!("Cost of delay: {cod}/week"); // every week of slippage costs roughly this much
```

## Next steps

- [`spec/architecture.md`](../../spec/architecture.md) explains the four ways a topic is modelled
  (a direct formula, a multi-step formula, a multi-metric bundle, or a qualitative framework) and
  how this crate uses `rusty_money::Money` directly alongside its own `Ratio`/`Percentage`
  newtypes.
- [`spec/topics.md`](../../spec/topics.md) maps all 64 topics to their module and primary
  type/function.
- The [README](../../README.md) lists every module's public items in one place.
