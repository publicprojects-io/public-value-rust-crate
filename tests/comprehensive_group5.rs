//! Comprehensive tests for Group 5 — `philanthropy_metrics` — reproducing the worked examples from
//! the source topics in `~/git/public-value-metrics` (see `spec/topics.md`).

use public_value::philanthropy_metrics::{
    bespoke_reporting_relationships, charity_overhead_ratio, cost_per_beneficiary, cost_per_outcome,
    donor_return_on_investment, effective_altruism_cost_effectiveness, programme_ratio,
    standardized_reporting_mappings, units_produced, volunteer_time_value, BlendedValue,
};
use public_value::units::Money;
use rust_decimal_macros::dec;

/// `cost-per-outcome`: £450,000 / 630 households achieving food security ≈ £714.
#[test]
fn cost_per_outcome_food_bank() {
    let cost = cost_per_outcome(Money::new(dec!(450_000)), 630);
    assert_eq!(cost.value().round_dp(2), dec!(714.29));
}

/// `cost-per-beneficiary`: £450,000 / 1,800 households served = £250, always ≤ cost per outcome
/// for the same programme since the outcome population is a subset of the beneficiary population.
#[test]
fn cost_per_beneficiary_food_bank() {
    let cost = cost_per_beneficiary(Money::new(dec!(450_000)), 1_800);
    assert_eq!(cost.value(), dec!(250));
    assert!(cost.value() <= cost_per_outcome(Money::new(dec!(450_000)), 630).value());
}

/// `blended-value`: the loan-A-versus-loan-B worked example. A purely financial lens prefers loan B
/// (higher interest), which the struct captures without ever collapsing the three lines into one
/// number.
#[test]
fn blended_value_loan_comparison() {
    let loan_a = BlendedValue::new(
        Money::new(dec!(10_000)),
        "40 people placed into sustained work per year".to_owned(),
        "Neutral".to_owned(),
    );
    let loan_b = BlendedValue::new(Money::new(dec!(30_000)), "None stated".to_owned(), "Neutral".to_owned());

    assert!(loan_a.economic_annual.value() < loan_b.economic_annual.value());
    assert_ne!(loan_a.social_line, loan_b.social_line);
}

/// `effective-altruism-cost-effectiveness`: `GiveWell`'s published Against Malaria Foundation
/// worked example, roughly $4,500 per life saved, i.e. very roughly 20 lives saved per £100,000.
#[test]
fn effective_altruism_cost_effectiveness_amf() {
    let cost_per_life = effective_altruism_cost_effectiveness(Money::new(dec!(4_500)), dec!(1));
    assert_eq!(cost_per_life.value(), dec!(4_500));

    let lives_per_100k = units_produced(Money::new(dec!(100_000)), cost_per_life);
    assert!((lives_per_100k - 22.222_222).abs() < 0.001);
}

/// `charity-overhead-ratio`: Charity A (8% overhead, no evaluation capacity) versus Charity B (22%
/// overhead, funded evaluation and case management) — the ratio alone cannot distinguish them.
#[test]
fn charity_overhead_ratio_two_charities() {
    let charity_a = charity_overhead_ratio(Money::new(dec!(80_000)), Money::new(dec!(1_000_000)));
    let charity_b = charity_overhead_ratio(Money::new(dec!(220_000)), Money::new(dec!(1_000_000)));

    assert!((charity_a.as_percent() - 8.0).abs() < 0.001);
    assert!((charity_b.as_percent() - 22.0).abs() < 0.001);
    assert!((programme_ratio(charity_b).as_percent() - 78.0).abs() < 0.001);
}

/// `donor-return-on-investment`: Charity D (funding-constrained, marginal £5,000 buys 20 additional
/// people served) has a strictly higher donor ROI than Charity C (fully funded, marginal gift not
/// additional), for the same £5,000 gift.
#[test]
fn donor_roi_funding_constrained_vs_fully_funded() {
    let roi_d = donor_return_on_investment(20.0, 0.0, Money::new(dec!(5_000)));
    let roi_c = donor_return_on_investment(0.0, 0.0, Money::new(dec!(5_000)));

    assert!((roi_d - 0.004).abs() < 1e-6);
    assert!(roi_c.abs() < 1e-9);
    assert!(roi_d > roi_c);
}

/// `grant-outcomes-reporting` (IRIS+): standardizing N funders × M grantees onto one shared
/// vocabulary turns N×M bespoke relationships into roughly N+M mappings.
#[test]
fn grant_outcomes_reporting_combinatorial_reduction() {
    let (funders, grantees) = (10, 20);
    assert_eq!(bespoke_reporting_relationships(funders, grantees), 200);
    assert_eq!(standardized_reporting_mappings(funders, grantees), 30);
    assert!(standardized_reporting_mappings(funders, grantees) < bespoke_reporting_relationships(funders, grantees));
}

/// `volunteer-time-value`: 5,000 UK volunteer hours at the ONS £14.43/hour replacement-cost rate =
/// £72,150, which raises a £300,000 cash-spend charity's true resource cost by about 24%.
#[test]
fn volunteer_time_value_uk_national_average() {
    let value = volunteer_time_value(dec!(5_000), Money::new(dec!(14.43)));
    assert_eq!(value.value(), dec!(72150.00));

    let true_cost = Money::new(dec!(300_000)) + value;
    assert_eq!(true_cost.value(), dec!(372150.00));
}
