# Topic mapping

The 64 topics ported from
[`public-value-metrics`](https://github.com/public-value-metrics/public-value-metrics)
(`locales/en-gb-oxendict/topics/<slug>/index.md`), grouped by category. Each category is one Rust
module and one `tests/comprehensive_group<N>.rs` file. See [architecture.md](architecture.md) for
the four modelling shapes and lint/test conventions.

## Group 1 — `foundations` (Public value foundations)

| Slug | Shape |
|---|---|
| `public-value` | D — `StrategicTriangle` |
| `value-for-money` | C — economy/efficiency/effectiveness (+equity) |
| `opportunity-cost-in-public-spending` | A |
| `social-discount-rate` | A — Green Book declining discount rate |
| `distributional-weighting` | A |
| `additionality-and-deadweight` | A |
| `displacement-and-attribution` | A |
| `counterfactual-analysis` | D |

## Group 2 — `economic_appraisal` (Governmental economic appraisal)

| Slug | Shape |
|---|---|
| `green-book-appraisal` | D — five-case model |
| `social-cost-benefit-analysis` | B |
| `cost-effectiveness-analysis-in-government` | A |
| `multi-criteria-decision-analysis` | C — weighted criteria scoring |
| `wellbeing-valuation` | A — WELLBY pricing |
| `stated-preference-valuation` | D |
| `revealed-preference-valuation` | D |
| `shadow-pricing` | A |

## Group 3 — `impact_measurement` (Social value and impact measurement)

| Slug | Shape |
|---|---|
| `social-return-on-investment` | B |
| `theory-of-change` | D |
| `logic-model` | D |
| `outcomes-vs-outputs` | D |
| `social-value-act` | D |
| `unit-cost-databases` | D |
| `impact-evaluation-methods` | D |
| `impact-evaluation-vs-process-evaluation` | D |

## Group 4 — `performance_metrics` (Governmental performance and delivery metrics)

| Slug | Shape |
|---|---|
| `public-sector-kpis` | D |
| `public-value-scorecard` | C |
| `outcomes-based-accountability` | D |
| `payment-by-results-and-social-impact-bonds` | A |
| `public-service-productivity` | A |
| `citizen-satisfaction-metrics` | A |
| `service-standards-and-transaction-metrics` | C |
| `trust-and-legitimacy-metrics` | D |

## Group 5 — `philanthropy_metrics` (Social sector and philanthropy metrics)

| Slug | Shape |
|---|---|
| `cost-per-outcome` | A |
| `cost-per-beneficiary` | A |
| `blended-value` | C |
| `effective-altruism-cost-effectiveness` | A |
| `charity-overhead-ratio` | A |
| `donor-return-on-investment` | A |
| `grant-outcomes-reporting` | D |
| `volunteer-time-value` | A |

## Group 6 — `digital_government` (Public value in digital government)

| Slug | Shape |
|---|---|
| `digital-service-standard` | D |
| `cost-per-transaction` | A |
| `channel-shift-savings` | A |
| `digital-inclusion` | D |
| `government-as-a-platform` | D |
| `open-data-value` | A |
| `public-sector-cybersecurity-value` | A |
| `ai-in-government-value` | D |

## Group 7 — `societal_indicators` (Wellbeing, equity and societal indicators)

| Slug | Shape |
|---|---|
| `gdp-alternatives` | D |
| `human-development-index` | B |
| `multidimensional-poverty-index` | B |
| `wellbeing-adjusted-life-years` | A |
| `index-of-multiple-deprivation` | B |
| `social-capital-metrics` | D |
| `natural-capital-accounting` | D |
| `intergenerational-equity-and-sustainability-discounting` | A |

## Group 8 — `delivery_connection` (Software engineering and digital delivery connection)

| Slug | Shape |
|---|---|
| `cost-of-delay-in-public-programmes` | A |
| `dora-metrics-for-public-value` | C |
| `flow-metrics-in-government-delivery` | A — Little's Law |
| `technical-debt-as-public-value-erosion` | A |
| `total-cost-of-ownership-in-government-it` | A |
| `build-vs-buy-in-government` | B |
| `benefits-realization` | D |
| `ai-productivity-in-the-public-sector` | D |

Shapes above are a starting classification from a skim of each source doc's "The maths" section, not
a hard rule — read the actual source file and pick whichever of the four shapes in
[architecture.md](architecture.md) fits best if a topic doesn't cleanly match.

## Extensions beyond the base 64 topics

These items are not ported from a `public-value-metrics` topic file — there is no
`locales/en-gb-oxendict/topics/<slug>/index.md` behind them. Each completes a connection an
existing topic's own text explicitly gestures at but doesn't implement, and each was independently
researched (not recalled from training) with real published sources before implementation. Add
future non-ported additions here, not to the table above, so the 64-topic mapping stays exactly
that.

| Item(s) | Module | Why added | Source |
|---|---|---|---|
| `qaly`, `clears_qaly_threshold` | `economic_appraisal` | `cost-effectiveness-analysis-in-government` says its method is "structurally identical" to NICE's cost-per-QALY method but never implements the QALY unit itself. | NICE technology appraisal guidance; cost-per-QALY threshold £20,000–£30,000 through 2025, £25,000–£35,000 from April 2026. |
| `years_of_life_lost`, `years_lived_with_disability`, `disability_adjusted_life_years` | `economic_appraisal` | `effective-altruism-cost-effectiveness` references "$ per DALY averted" as a sibling unit to its lives-saved reasoning, without implementing DALYs. | WHO Global Burden of Disease methodology: DALY = YLL + YLD. |
| `gini_coefficient` | `societal_indicators` | The standard companion to `distributional-weighting` and the Multidimensional Poverty Index for describing how unequally a distribution actually lands; none of the 64 topics compute it directly. | Standard Lorenz-curve/trapezoidal-rule definition (Gini, 1936). |
| `atkinson_inequality_measure`, `inequality_adjusted_dimension_index`, `inequality_adjusted_hdi`, `ihdi_loss_percentage` | `societal_indicators` | `human-development-index`'s own pitfalls section names "UNDP's separate Inequality-adjusted HDI" as the tool for exactly the limitation it warns about, without implementing it. | UNDP IHDI technical notes, building on Foster, Lopez-Calva and Szekely (2005) and Atkinson (1970). |
| `net_promoter_score` | `performance_metrics` | Standard companion to `citizen-satisfaction-metrics`'s `net_satisfaction`; some funders and service standards ask for the promoter/detractor framing specifically. | Standard NPS definition (Reichheld, 2003): % promoters − % detractors. |

See `tests/comprehensive_extensions.rs` for the worked examples backing these.
