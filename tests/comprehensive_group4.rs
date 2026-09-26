//! Comprehensive tests for Group 4 — `performance_metrics` — reproducing the worked examples from
//! the source topics in `~/git/public-value-metrics` (see `spec/topics.md`).

use public_value::economic_appraisal::cost_effectiveness_ratio;
use public_value::performance_metrics::{
    channel_shift_saving, cost_per_transaction, failure_demand_cost, kpi_percentage, net_satisfaction,
    payment_by_results_total, productivity_growth_percentage_points, quality_adjusted_output_index,
    LegitimacyTriangulation, PerformanceAccountability, PublicValueScorecard,
};
use public_value::units::Percentage;
use rust_decimal_macros::dec;
use rusty_money::{Money, iso};

/// `public-sector-kpis`: the ambulance trust worked example, 75.0% headline KPI.
#[test]
fn public_sector_kpis_ambulance_trust() {
    let headline = kpi_percentage(4_500, 6_000);
    assert!((headline.as_percent() - 75.0).abs() < 0.01);
}

/// `public-value-scorecard`: the reablement service worked example. Mission and stewardship look
/// like a success story, but the process perspective reveals a hidden staffing risk.
#[test]
fn public_value_scorecard_reablement_service() {
    let reablement_service = PublicValueScorecard {
        mission_outcome_rate: Percentage::from_percent(68.0),
        mission_outcome_target: Percentage::from_percent(65.0),
        stewardship_cost_per_episode: Money::from_decimal(dec!(1_850), iso::USD),
        stewardship_budgeted_cost_per_episode: Money::from_decimal(dec!(2_000), iso::USD),
        process_staff_vacancy_rate: Percentage::from_percent(14.0),
        process_caseload: 23,
        process_safe_caseload_ceiling: 25,
    };

    assert!(reablement_service.looks_like_a_success_on_headline_numbers());
    assert!(reablement_service.has_hidden_staffing_risk(Percentage::from_percent(10.0)));
    assert!(reablement_service.process_caseload < reablement_service.process_safe_caseload_ceiling);
}

/// `outcomes-based-accountability`: the city-funded employment programme worked example. It meets
/// its activity target and shows a 25.7-percentage-point uplift over a matched comparison group.
#[test]
fn outcomes_based_accountability_employment_programme() {
    let programme = PerformanceAccountability {
        activity_achieved: 500,
        activity_target: 480,
        completion_rate: Percentage::from_percent(78.0),
        outcome_rate: Percentage::from_fraction(260.0 / 390.0),
        comparison_group_outcome_rate: Percentage::from_percent(41.0),
    };

    assert!(programme.met_activity_target());
    assert!((programme.better_off_uplift().as_percent() - 25.7).abs() < 0.1);

    let cost_per_completer = cost_effectiveness_ratio(Money::from_decimal(dec!(340_000), iso::USD), 390);
    assert!((cost_per_completer.amount() - dec!(872)).abs() < dec!(1));
}

/// `payment-by-results-and-social-impact-bonds`: the family-intervention contract worked example,
/// £1,376,000 total for 96 confirmed sustained outcomes.
#[test]
fn payment_by_results_family_intervention() {
    let total = payment_by_results_total(200, Money::from_decimal(dec!(4_000), iso::USD), 96, Money::from_decimal(dec!(6_000), iso::USD));
    assert_eq!(*total.amount(), dec!(1_376_000));

    let cost_per_outcome = cost_effectiveness_ratio(total, 96);
    assert!((cost_per_outcome.amount() - dec!(14_333)).abs() < dec!(1));
}

/// `public-service-productivity`: the illustrative NHS acute-sector worked example, productivity
/// falls 1.23 percentage points even as raw activity rises.
#[test]
fn public_service_productivity_nhs_illustration() {
    let output_index = quality_adjusted_output_index(100.0, 0.030, -0.010);
    let input_index = 100.0 * 1.032;

    assert!((output_index - 101.97).abs() < 0.01);
    let growth = productivity_growth_percentage_points(output_index, input_index, 100.0);
    assert!((growth - -1.23).abs() < 0.01);
}

/// `citizen-satisfaction-metrics`: the council tax e-billing worked example, net satisfaction
/// +58.3.
#[test]
fn citizen_satisfaction_council_tax_ebilling() {
    let net = net_satisfaction(1_650, 250, 2_400);
    assert!((net.as_percent() - 58.3).abs() < 0.1);
}

/// `service-standards-and-transaction-metrics`: the licence-renewal service worked example.
/// Take-up shift saves £1,350,000/year and the failure-demand saving from improved completion is
/// £132,000/year.
#[test]
fn service_standards_licence_renewal() {
    let take_up_shift = channel_shift_saving(2_000_000, Percentage::from_percent(25.0), Money::from_decimal(dec!(3.00), iso::USD), Money::from_decimal(dec!(0.30), iso::USD));
    assert_eq!(take_up_shift.amount().round_dp(2), dec!(1_350_000.00));

    let before = failure_demand_cost(700_000, Percentage::from_percent(80.0), Money::from_decimal(dec!(3.00), iso::USD));
    let after = failure_demand_cost(1_200_000, Percentage::from_percent(92.0), Money::from_decimal(dec!(3.00), iso::USD));
    assert_eq!(before.amount().round_dp(2), dec!(420_000.00));
    assert_eq!(after.amount().round_dp(2), dec!(288_000.00));

    let net_failure_demand_saving = before.sub(after).unwrap();
    assert_eq!(net_failure_demand_saving.amount().round_dp(2), dec!(132_000.00));

    let total_saving = take_up_shift.add(net_failure_demand_saving).unwrap();
    assert_eq!(total_saving.amount().round_dp(2), dec!(1_482_000.00));

    let cost_per_txn = cost_per_transaction(Money::from_decimal(dec!(600_000), iso::USD), 2_000_000);
    assert_eq!(*cost_per_txn.amount(), dec!(0.3));
}

/// `trust-and-legitimacy-metrics`: the national tax authority worked example. Trust falling,
/// upheld complaints rising, and ombudsman findings increasingly against the authority together
/// form a credible legitimacy finding.
#[test]
fn trust_and_legitimacy_tax_authority() {
    let tax_authority = LegitimacyTriangulation {
        trust_index: Percentage::from_percent(58.0),
        trust_index_prior: Percentage::from_percent(64.0),
        upheld_complaints_per_1000: 4.2,
        upheld_complaints_per_1000_prior: 3.1,
        ombudsman_uphold_rate: Percentage::from_percent(61.0),
        ombudsman_uphold_rate_prior: Percentage::from_percent(48.0),
    };
    assert!(tax_authority.converging_negative_signal());

    let stable = LegitimacyTriangulation {
        trust_index: Percentage::from_percent(64.0),
        ..tax_authority
    };
    assert!(!stable.converging_negative_signal());
}
