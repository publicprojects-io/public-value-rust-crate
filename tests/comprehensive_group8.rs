//! Comprehensive tests for Group 8 — `delivery_connection` — reproducing the worked examples from
//! the source topics in `~/git/public-value-metrics` (see `spec/topics.md`).

use public_value::delivery_connection::{
    ai_capacity_value, buy_total_cost, cost_of_delay_per_week, cycle_time, delay_wellby_loss,
    eligible_developers, fewer_failed_changes, flow_efficiency, incident_avoidance_saving,
    interest_reduction, lead_time_value_pulled_forward, net_capacity_ratio, payback_period_years,
    realization_rate, risk_adjusted_build_cost, technical_debt_principal, total_cost_of_ownership,
    total_delay_loss,
};
use public_value::units::Percentage;
use rust_decimal_macros::dec;
use rusty_money::{Money, iso};

/// `cost-of-delay-in-public-programmes`: the housing-benefit worked example (`CoD` ≈£57,700/week)
/// and the disability-benefit WELLBY-denominated worked example (≈1,080 WELLBYs/year lost).
#[test]
fn cost_of_delay_examples() {
    let cod = cost_of_delay_per_week(Money::from_decimal(dec!(3_000_000), iso::USD));
    assert_eq!(cod.amount().round_dp(0), dec!(57_692));

    let total = total_delay_loss(cod, 52);
    assert_eq!(total.amount().round_dp(0), dec!(3_000_000));

    let wellby_loss = delay_wellby_loss(200_000, 3.0, 0.0018);
    assert!((wellby_loss - 1_080.0).abs() < 1e-6);
}

/// `dora-metrics-for-public-value`: the benefits-claims portal worked example. Cutting lead time
/// pulls ≈£1.46m/year of value forward, and cutting change failure rate avoids ≈£84,200/year.
#[test]
fn dora_metrics_benefits_claims_portal() {
    let weeks_saved = 8.0 - 5.0 / 7.0;
    let lead_time_value = lead_time_value_pulled_forward(25, weeks_saved, Money::from_decimal(dec!(8_000), iso::USD));
    assert!((lead_time_value.amount() - dec!(1_457_143)).abs() < dec!(1_000));

    let fewer_failures = fewer_failed_changes(25, Percentage::from_percent(30.0), Percentage::from_percent(10.0));
    assert!((fewer_failures - 5.0).abs() < 1e-9);

    let saving = incident_avoidance_saving(fewer_failures, Money::from_decimal(dec!(16_840), iso::USD));
    assert_eq!(*saving.amount(), dec!(84_200));
}

/// `flow-metrics-in-government-delivery`: the planning-department worked example, an 8-week cycle
/// time at 1.3% flow efficiency, and the WIP-limit intervention that nearly halves cycle time.
#[test]
fn flow_metrics_planning_department() {
    let weeks = cycle_time(400, 50);
    assert!((weeks - 8.0).abs() < 1e-9);

    let efficiency = flow_efficiency(6.0, 56.0, 8.0);
    assert!((efficiency.as_percent() - 1.3).abs() < 0.05);

    let reduced_weeks = cycle_time(240, 50);
    assert!((reduced_weeks - 4.8).abs() < 1e-9);
    assert!(reduced_weeks < weeks / 1.5);
}

/// `technical-debt-as-public-value-erosion`: the legacy claims-engine worked example, £712,500
/// principal and a 2.5-year payback on targeted remediation.
#[test]
fn technical_debt_legacy_claims_engine() {
    let principal = technical_debt_principal(250_000, Money::from_decimal(dec!(2.85), iso::USD));
    assert_eq!(*principal.amount(), dec!(712_500.00));

    let redirected_contact_cost = Money::from_decimal(dec!(25), iso::USD).mul(5_000_u32).unwrap().mul(4_u32).unwrap();
    let interest = Money::from_decimal(dec!(180_000), iso::USD).add(redirected_contact_cost).unwrap();
    assert_eq!(*interest.amount(), dec!(680_000));

    let reduction = interest_reduction(interest, Percentage::from_percent(70.0));
    assert_eq!(reduction.amount().round_dp(2), dec!(476_000.00));

    let payback = payback_period_years(Money::from_decimal(dec!(1_200_000), iso::USD), reduction);
    assert!((payback - 2.52).abs() < 0.01);
}

/// `total-cost-of-ownership-in-government-it`: the two-system worked example. System B looks
/// cheaper on capex alone but is marginally more expensive once TCO is discounted and summed.
#[test]
fn total_cost_of_ownership_two_systems() {
    let tco_a = total_cost_of_ownership(Money::from_decimal(dec!(3_500_000), iso::USD), Money::from_decimal(dec!(250_000), iso::USD), dec!(0.035), 5);
    let tco_b = total_cost_of_ownership(Money::from_decimal(dec!(1_800_000), iso::USD), Money::from_decimal(dec!(650_000), iso::USD), dec!(0.035), 5);

    assert!((tco_a.amount() - dec!(4_628_763)).abs() < dec!(1_000));
    assert!((tco_b.amount() - dec!(4_734_784)).abs() < dec!(1_000));
    assert!(tco_b.gt(&tco_a).unwrap()); // reverses the naive capex-only comparison
}

/// `build-vs-buy-in-government`: the adult social care case-management system worked example. Buy
/// wins by roughly £1.5m over five years once risk adjustment and delay cost are counted.
#[test]
fn build_vs_buy_case_management_system() {
    let risk_adjusted_build = risk_adjusted_build_cost(Money::from_decimal(dec!(900_000), iso::USD), dec!(1.4));
    assert_eq!(*risk_adjusted_build.amount(), dec!(1_260_000.0));

    let build_five_year_tco = risk_adjusted_build.add(Money::from_decimal(dec!(150_000), iso::USD).mul(5_u32).unwrap()).unwrap();
    let delay_cost = Money::from_decimal(dec!(40_000), iso::USD).mul(10_u32).unwrap();
    let effective_build_cost = build_five_year_tco.add(delay_cost).unwrap();

    let buy_cost = buy_total_cost(Money::from_decimal(dec!(180_000), iso::USD), 5);

    assert_eq!(*build_five_year_tco.amount(), dec!(2_010_000.0));
    assert_eq!(*effective_build_cost.amount(), dec!(2_410_000.0));
    assert_eq!(*buy_cost.amount(), dec!(900_000));

    let buy_advantage = effective_build_cost.sub(buy_cost).unwrap();
    assert!((buy_advantage.amount() - dec!(1_510_000)).abs() < dec!(10_000));
}

/// `benefits-realization`: the digital planning-application portal worked example. A 70%
/// realization rate on cash savings is knowledge, not automatically a failure.
#[test]
fn benefits_realization_planning_portal() {
    let cash_rate = realization_rate(210_000.0, 300_000.0);
    let hours_rate = realization_rate(3_200.0, 4_500.0);
    let satisfaction_rate = realization_rate(11.0, 8.0);

    assert!((cash_rate.as_percent() - 70.0).abs() < 0.01);
    assert!((hours_rate.as_percent() - 71.1).abs() < 0.1);
    assert!((satisfaction_rate.as_percent() - 137.5).abs() < 0.1);
}

/// `ai-productivity-in-the-public-sector`: the department-wide pilot worked example. Valuing the
/// measured (not self-reported) saving against the classification-eligible population gives a net
/// capacity ratio of roughly 5.5:1.
#[test]
fn ai_productivity_department_pilot() {
    let eligible = eligible_developers(300, Percentage::from_percent(70.0));
    assert_eq!(eligible, 210);

    let value = ai_capacity_value(eligible, 0.2, 220, Money::from_decimal(dec!(55), iso::USD), Percentage::from_percent(60.0));
    assert_eq!(value.amount().round_dp(0), dec!(304_920));

    let licensing_cost = Money::from_decimal(dec!(22), iso::USD).mul(12_u32).unwrap().mul(eligible).unwrap();
    assert_eq!(*licensing_cost.amount(), dec!(55_440));

    let ratio = net_capacity_ratio(value, licensing_cost);
    assert!((ratio.value() - 5.5).abs() < 0.1);
}
