# public-value

[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0%20OR%20BSD--3--Clause%20OR%20GPL--2.0--only%20OR%20GPL--3.0--only-blue)](#license)

Public value models, structs, and calculations for software engineers building for governmental and
social sector organizations: value for money, cost-benefit and cost-effectiveness analysis, social
return on investment, the Human Development Index, DORA-metrics-to-public-value translation, and 59
other formulas and frameworks, each ported with a worked example from
[`public-value-metrics`](https://github.com/public-value-metrics/public-value-metrics) — plus a
handful of independently researched extensions (QALYs, DALYs, the Gini coefficient, the
Inequality-adjusted HDI, Net Promoter Score) that complete connections the source topics reference
but don't implement. See [`spec/topics.md`](spec/topics.md#extensions-beyond-the-base-64-topics).

```toml
[dependencies]
public-value = "0.3"
rusty-money = "0.5"
rust_decimal_macros = "1"
```

## Quick start

```rust
use public_value::philanthropy_metrics::cost_per_outcome;
use rust_decimal_macros::dec;
use rusty_money::{iso, Money};

// £450,000 spent; 630 households achieved food security (not just received a food parcel).
let cost = cost_per_outcome(Money::from_decimal(dec!(450_000), iso::USD), 630);
println!("Cost per outcome: {cost}"); // $714.29
```

Every public function and type carries a doctest reproducing a real worked example from the source
material — read the [generated rustdoc](https://docs.rs/public-value) or a module's source for the
worked numbers behind any calculation.

## Why a crate, not just a spreadsheet

Public value calculations get re-derived, by hand, in a spreadsheet, for every business case, grant
application, and procurement bid — which is exactly how a deadweight adjustment gets silently
dropped from an SROI ratio, or a discount rate gets applied inconsistently across two options in
the same appraisal. This crate gives those calculations a typed, tested, version-controlled home:

- **[`units`](src/units.rs)** — shared `Money` (backed by [`rusty_money`](https://docs.rs/rusty-money),
  not a bare `f64` or `Decimal`, so repeated addition and percentage scaling stay exact; every value
  is tagged with a fixed internal working currency, `USD`, so arithmetic never hits a currency
  mismatch — see the module's rustdoc), `Ratio`, and `Percentage` newtypes, so a ratio and a
  percentage and a money amount can't be swapped by accident at a call site.
- **Worked-example doctests** — every public item's rustdoc reproduces an actual number from its
  source topic's worked example, not an invented one, so the documentation is also a regression
  test.
- **`#![deny(missing_docs)]` and `#![deny(clippy::pedantic)]`** — every public item is documented,
  and the lint bar stays high as the crate grows.

## Modules

Each module below is one category from the source material; each public item is one topic. See
[`spec/topics.md`](spec/topics.md) for the full topic-to-symbol mapping and
[`spec/architecture.md`](spec/architecture.md) for the conventions every module follows.

### [`foundations`](src/foundations.rs) — public value foundations

`StrategicTriangle`, `ValueForMoneyOption`, `net_public_value`, `discount_factor`/`present_value`,
`distributional_weight`/`weighted_benefit`, `net_additional_outcomes`, `net_additional_impact`,
`difference_in_differences`.

### [`economic_appraisal`](src/economic_appraisal.rs) — governmental economic appraisal

`FiveCaseModel`, `annuity_factor`, `net_present_social_value`/`benefit_cost_ratio`,
`cost_effectiveness_ratio`/`incremental_cost_effectiveness_ratio`, `weighted_score`,
`wellbys`/`monetize_wellbys`, `aggregate_stated_preference_value`, `hedonic_implicit_price`,
`travel_cost_annual_value`, `shadow_carbon_benefit`, `shadow_wage_rate`, plus the health-economics
units these formulas reference but don't implement: `qaly`/`clears_qaly_threshold` and
`years_of_life_lost`/`years_lived_with_disability`/`disability_adjusted_life_years` (see
[Extensions](spec/topics.md#extensions-beyond-the-base-64-topics)).

### [`impact_measurement`](src/impact_measurement.rs) — social value and impact measurement

`sroi_ratio`, `TheoryOfChange`, `LogicModel`, `net_outcome_uplift`,
`proportional_social_value_score`, `applied_unit_cost_value`, `propensity_score_matching_impact`,
`CombinedDiagnosis`/`diagnose`.

### [`performance_metrics`](src/performance_metrics.rs) — governmental performance and delivery metrics

`kpi_percentage`, `PublicValueScorecard`, `PerformanceAccountability`, `payment_by_results_total`,
`productivity_growth_percentage_points`, `net_satisfaction`, `channel_shift_saving`,
`failure_demand_cost`, `LegitimacyTriangulation`, `net_promoter_score` (extension).

### [`philanthropy_metrics`](src/philanthropy_metrics.rs) — social sector and philanthropy metrics

`cost_per_outcome`, `cost_per_beneficiary`, `BlendedValue`,
`effective_altruism_cost_effectiveness`, `charity_overhead_ratio`, `donor_return_on_investment`,
`bespoke_reporting_relationships`/`standardized_reporting_mappings`, `volunteer_time_value`.

### [`digital_government`](src/digital_government.rs) — public value in digital government

`ServiceAssessment`, `gross_channel_shift_saving`/`fte_released`, `blended_cost_per_transaction`,
`platform_adoption_saving`, `cost_avoided_open_data_value`, `annualized_loss_expectancy`,
`security_control_value`, `net_ai_value`.

### [`societal_indicators`](src/societal_indicators.rs) — wellbeing, equity and societal indicators

`genuine_progress_indicator`, `human_development_index`, `multidimensional_poverty_index`,
`imd_composite_score`/`imd_decile`, `social_capital_pillar_gap`, `ecosystem_asset_value`,
`present_value_across_schedule`, plus `gini_coefficient` and the Inequality-adjusted HDI
(`atkinson_inequality_measure`, `inequality_adjusted_dimension_index`, `inequality_adjusted_hdi`,
`ihdi_loss_percentage`) (extensions).

### [`delivery_connection`](src/delivery_connection.rs) — software engineering and digital delivery connection

`cost_of_delay_per_week`, `lead_time_value_pulled_forward`/`fewer_failed_changes`, `cycle_time`,
`flow_efficiency`, `technical_debt_principal`/`interest_reduction`, `total_cost_of_ownership`,
`risk_adjusted_build_cost`, `realization_rate`, `ai_capacity_value`.

## Testing this crate

```sh
cargo test                          # unit + integration + 107 doctests
cargo clippy --all-targets -- -D warnings
cargo doc --no-deps --open
```

`tests/comprehensive_group1.rs` through `comprehensive_group8.rs` mirror the module list above — one
file per category, one test per topic. `tests/comprehensive_extensions.rs` covers the extensions.

## Source and scope

The domain content (definitions, formulas, worked examples) is ported from
[`public-value-metrics`](https://github.com/public-value-metrics/public-value-metrics)'s canonical
`en-gb-oxendict` locale. This crate deliberately does not port that project's locale-translation
infrastructure or its SvelteKit documentation site — it is a calculation library, not a book or a
website. See [`spec/architecture.md`](spec/architecture.md) for what was and wasn't carried over and
why.

## License

Licensed, at your option, under any of:

- MIT ([LICENSE-MIT.md](LICENSE-MIT.md))
- Apache-2.0 ([LICENSE-APACHE.md](LICENSE-APACHE.md))
- BSD-3-Clause ([LICENSE-BSD-3-CLAUSE.md](LICENSE-BSD-3-CLAUSE.md))
- GPL-2.0-only ([LICENSE-GPL-2.0.md](LICENSE-GPL-2.0.md))
- GPL-3.0-only ([LICENSE-GPL-3.0.md](LICENSE-GPL-3.0.md))
