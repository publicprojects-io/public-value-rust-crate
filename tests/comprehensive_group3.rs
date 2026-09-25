//! Comprehensive tests for Group 3 — `impact_measurement` — reproducing the worked examples from
//! the source topics in `~/git/public-value-metrics` (see `spec/topics.md`).

use public_value::foundations::present_value;
use public_value::impact_measurement::{
    applied_unit_cost_value, assumption_hold_rate, attributable_outcome_count, diagnose,
    difference_in_differences_impact, net_outcome_uplift, propensity_score_matching_impact,
    proportional_social_value_score, sroi_ratio, value_net_of_rate, CausalLink, CombinedDiagnosis,
    LogicModel, TheoryOfChange,
};
use public_value::units::{Money, Percentage};
use rust_decimal_macros::dec;

/// `social-return-on-investment`: the local authority employment programme worked example, SROI
/// ratio ≈ 1.44.
#[test]
fn sroi_employment_programme() {
    let gross = Money::new(dec!(8_500)) * 60_u32;
    assert_eq!(gross.value(), dec!(510_000));

    let year1_impact = value_net_of_rate(
        value_net_of_rate(gross, Percentage::from_percent(40.0)),
        Percentage::from_percent(30.0),
    );
    assert_eq!(year1_impact.value().round_dp(2), dec!(214_200.00));

    let year2_impact = present_value(value_net_of_rate(year1_impact, Percentage::from_percent(30.0)), dec!(0.035), 1);
    assert!((year2_impact.value().round_dp(0) - dec!(144_870)).abs() <= dec!(1));

    let total_impact = year1_impact + year2_impact;
    let ratio = sroi_ratio(total_impact, Money::new(dec!(250_000)));
    assert!((ratio.value() - 1.436).abs() < 0.01);
}

/// `theory-of-change`: the homelessness-prevention worked example, backward-mapped links and the
/// 85% assumption-hold rate observed in the pilot cohort.
#[test]
fn theory_of_change_homelessness_prevention() {
    let theory = TheoryOfChange {
        long_term_outcome: "Sustained tenancies at 12 months for households at risk of eviction".to_owned(),
        links: vec![
            CausalLink {
                precondition: "Households have a realistic, affordable repayment plan for arrears".to_owned(),
                assumption: "Caseworker-negotiated plans are more sustainable than court-ordered ones".to_owned(),
                indicator: "% of plans still active at 6 months".to_owned(),
            },
            CausalLink {
                precondition: "Households claim the benefits they are entitled to".to_owned(),
                assumption: "A digital benefits calculator increases correct claims versus paper forms".to_owned(),
                indicator: "Claim accuracy rate, compared pre/post tool rollout".to_owned(),
            },
        ],
    };
    assert_eq!(theory.link_count(), 2);

    let hold_rate = assumption_hold_rate(102, 120);
    assert!((hold_rate.as_percent() - 85.0).abs() < 0.01);
}

/// `logic-model`: the digital debt advice service worked example. A model with populated outputs
/// but no outcomes is diagnosable as "stopping at outputs".
#[test]
fn logic_model_debt_advice_service() {
    let debt_advice = LogicModel {
        inputs: vec!["£180,000 annual budget".to_owned(), "4.0 FTE advisers".to_owned()],
        activities: vec!["One-to-one debt advice appointments".to_owned()],
        outputs: vec!["900 appointments delivered".to_owned(), "750 debt and benefit plans issued".to_owned()],
        outcomes: vec!["450 of 750 clients report reduced arrears at 6-month follow-up".to_owned()],
        impact: vec![],
    };
    assert!(!debt_advice.stops_at_outputs());

    let outputs_only = LogicModel {
        outputs: vec!["900 appointments delivered".to_owned()],
        ..LogicModel::default()
    };
    assert!(outputs_only.stops_at_outputs());
}

/// `outcomes-vs-outputs`: the employment-support worked example, net uplift 13 percentage points
/// and 65 attributable additional people in work.
#[test]
fn outcomes_vs_outputs_employment_support() {
    let uplift = net_outcome_uplift(Percentage::from_percent(28.0), Percentage::from_percent(15.0));
    assert!((uplift.as_percent() - 13.0).abs() < 0.01);

    let attributable = attributable_outcome_count(500, uplift);
    assert!((attributable - 65.0).abs() < 0.01);
}

/// `social-value-act`: the £2m IT contract worked example, Bidder A scores the full 10 points and
/// Bidder B scores ≈4.4.
#[test]
fn social_value_act_it_contract() {
    let strongest_bid = Money::new(dec!(90_000));
    let score_for_strongest = proportional_social_value_score(strongest_bid, strongest_bid, 10.0);
    let score_for_weaker = proportional_social_value_score(Money::new(dec!(40_000)), strongest_bid, 10.0);

    assert!((score_for_strongest - 10.0).abs() < 0.01);
    assert!((score_for_weaker - 4.44).abs() < 0.01);
}

/// `unit-cost-databases`: the befriending-service and job-club worked examples.
#[test]
fn unit_cost_databases_examples() {
    let loneliness_value = applied_unit_cost_value(80, Money::new(dec!(1_100)));
    let employment_value = applied_unit_cost_value(45, Money::new(dec!(8_500)));

    assert_eq!(loneliness_value.value(), dec!(88_000));
    assert_eq!(employment_value.value(), dec!(382_500));
}

/// `impact-evaluation-methods`: the troubled-families difference-in-differences worked example
/// (+3 percentage points) and the employability-charity PSM worked example (+13 percentage points).
#[test]
fn impact_evaluation_methods_examples() {
    let did_estimate = difference_in_differences_impact(84.0, 89.0, 85.0, 87.0);
    assert!((did_estimate - 3.0).abs() < 1e-9);

    let psm_estimate = propensity_score_matching_impact(Percentage::from_percent(46.0), Percentage::from_percent(33.0));
    assert!((psm_estimate.as_percent() - 13.0).abs() < 0.01);
}

/// `impact-evaluation-vs-process-evaluation`: the parenting-programme (implementation failure) and
/// digital-literacy-programme (replicate with confidence) worked examples.
#[test]
fn impact_vs_process_evaluation_examples() {
    // Parenting programme: non-significant impact, only 19% of planned reach hit the fidelity
    // threshold.
    assert_eq!(diagnose(false, false), CombinedDiagnosis::ImplementationFailure);

    // Digital literacy programme: a strong effect, 92% fidelity across all sites.
    assert_eq!(diagnose(true, true), CombinedDiagnosis::ReplicateWithConfidence);
}
