//! Comprehensive tests for Group 1 — `foundations` — reproducing the worked examples from the
//! source topics in `~/git/public-value-metrics` (see `spec/topics.md`).

use public_value::foundations::{
    attribution_shares_valid, difference_in_differences, discount_factor, distributional_weight,
    net_additional_impact, net_additional_outcomes, net_public_value, present_value, weighted_benefit,
    StrategicTriangle, ValueForMoneyOption,
};
use public_value::units::{Money, Percentage};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;

/// `public-value`: the housing-triage worked example fails on legitimacy and operational capacity,
/// so the strategic triangle does not pass even though it offers real public value.
#[test]
fn strategic_triangle_housing_triage() {
    let housing_triage = StrategicTriangle {
        legitimacy_and_support: false,
        public_value: true,
        operational_capacity: false,
    };
    assert!(!housing_triage.passes());

    let fully_authorized = StrategicTriangle {
        legitimacy_and_support: true,
        public_value: true,
        operational_capacity: true,
    };
    assert!(fully_authorized.passes());
}

/// `value-for-money`: the contact-centre worked example. Option A's cheaper licence wins on
/// economy alone but loses once efficiency accumulates past about a year.
#[test]
fn value_for_money_contact_centre() {
    let option_a = ValueForMoneyOption {
        upfront_cost: Money::new(dec!(600_000)),
        minutes_per_case: dec!(22),
        staff_cost_per_hour: Money::new(dec!(28)),
    };
    let option_b = ValueForMoneyOption {
        upfront_cost: Money::new(dec!(900_000)),
        minutes_per_case: dec!(9),
        staff_cost_per_hour: Money::new(dec!(28)),
    };

    assert_eq!(option_a.annual_efficiency_cost(40_000).value().round_dp(0), dec!(410_667));
    assert_eq!(option_b.annual_efficiency_cost(40_000).value().round_dp(0), dec!(168_000));

    assert!(option_a.total_cost(40_000, 1).value() < option_b.total_cost(40_000, 1).value());
    assert!(option_a.total_cost(40_000, 2).value() > option_b.total_cost(40_000, 2).value());
}

/// `opportunity-cost-in-public-spending`: the digital-transformation fund worked example. The net
/// case for option A over the realistic alternative B is £0.8m, not A's full £7.2m headline.
#[test]
fn opportunity_cost_digital_transformation_fund() {
    let net = net_public_value(Money::new(dec!(7_200_000)), Money::new(dec!(6_400_000)));
    assert_eq!(net.value(), dec!(800_000));
}

/// `social-discount-rate`: the flood-defence worked example at a flat 3.5% rate, and the declining
/// schedule (3.5% for 30 years, 3.0% for the remaining 10) which raises the present value.
#[test]
fn social_discount_rate_flood_defence() {
    let flat_pv = present_value(Money::new(dec!(10_000_000)), dec!(0.035), 40);
    assert!((flat_pv.value().to_f64().unwrap() - 2_525_725.0).abs() < 1_000.0);

    let declining_factor = discount_factor(dec!(0.035), 30) * discount_factor(dec!(0.03), 10);
    let declining_pv = Money::new(dec!(10_000_000)) / declining_factor;
    assert!((declining_pv.value().to_f64().unwrap() - 2_651_046.0).abs() < 5_000.0);
    assert!(declining_pv.value() > flat_pv.value());
}

/// `distributional-weighting`: the two-programme worked example. Unweighted, programmes A and B
/// are tied at £2m; weighted for distributional impact, B's benefit is more than three times A's.
#[test]
fn distributional_weighting_two_programmes() {
    let weight_a = distributional_weight(Money::new(dec!(45_000)), Money::new(dec!(35_000)), 1.3);
    let weight_b = distributional_weight(Money::new(dec!(18_000)), Money::new(dec!(35_000)), 1.3);

    let weighted_a = weighted_benefit(Money::new(dec!(2_000_000)), weight_a);
    let weighted_b = weighted_benefit(Money::new(dec!(2_000_000)), weight_b);

    assert!((weighted_a.value().to_f64().unwrap() - 1_440_000.0).abs() < 50_000.0);
    assert!(weighted_b.value() > weighted_a.value() * dec!(3));
}

/// `additionality-and-deadweight`: the business-support grant (40% deadweight → 900 net jobs) and
/// charity employment programme (15% deadweight → 170 net placements) worked examples.
#[test]
fn additionality_and_deadweight_examples() {
    let net_jobs = net_additional_outcomes(1_500, Percentage::from_percent(40.0));
    assert!((net_jobs - 900.0).abs() < 1e-9);

    let net_placements = net_additional_outcomes(200, Percentage::from_percent(15.0));
    assert!((net_placements - 170.0).abs() < 1e-9);
}

/// `displacement-and-attribution`: the regeneration grant's borough-level versus regional-level net
/// jobs, and the three-partner contribution-analysis shares that must not overstate the observed
/// 30-person reduction in rough sleeping.
#[test]
fn displacement_and_attribution_examples() {
    assert_eq!(net_additional_impact(200, 60), 140);
    assert_eq!(net_additional_impact(200, 60 + 30), 110);

    assert!(attribution_shares_valid(&[12.0, 10.5, 7.5], 30.0));
    assert!(!attribution_shares_valid(&[30.0, 30.0, 30.0], 30.0));
}

/// `counterfactual-analysis`: the employment-programme difference-in-differences worked example.
/// The naive before/after read (+15 points) overstates the true effect (+6 points) once the
/// comparison group's own change is subtracted.
#[test]
fn counterfactual_analysis_difference_in_differences() {
    let naive_before_after = 55.0 - 40.0;
    let did_estimate = difference_in_differences(40.0, 55.0, 38.0, 47.0);

    assert!((did_estimate - 6.0).abs() < 1e-9);
    assert!(did_estimate < naive_before_after);
}
