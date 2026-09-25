//! Comprehensive tests for Group 6 — `digital_government` — reproducing the worked examples from
//! the source topics in `~/git/public-value-metrics` (see `spec/topics.md`).

use public_value::digital_government::{
    annualized_loss_expectancy, blended_cost_per_transaction, cost_avoided_open_data_value,
    failed_assessment_cost, fte_released, gross_channel_shift_saving, handling_time_cost,
    net_ai_value, platform_adoption_saving, prorate_annual_value, security_control_value,
    ServiceAssessment,
};
use public_value::performance_metrics::cost_per_transaction;
use public_value::units::{Money, Percentage};
use rust_decimal_macros::dec;

/// `digital-service-standard`: the housing application service worked example. 11 of 14 points met
/// is not a pass, and the failed assessment has a real, calculable cost.
#[test]
fn digital_service_standard_housing_application() {
    let assessment = ServiceAssessment { points_met: 11, total_points: 14 };
    assert!(!assessment.passes());

    let delay_cost = prorate_annual_value(Money::new(dec!(59_760)), 2);
    assert_eq!(delay_cost.value().round_dp(0), dec!(9_960));

    let total = failed_assessment_cost(Money::new(dec!(34_650)), Money::new(dec!(5_000)), delay_cost);
    assert_eq!(total.value().round_dp(0), dec!(49_610));
}

/// `cost-per-transaction`: the vehicle tax renewal worked example. The marginal-only figure is
/// roughly 7.5x cheaper than the fully-loaded one, but both are cheaper than the phone comparator.
#[test]
fn cost_per_transaction_vehicle_tax_renewal() {
    let marginal_only = cost_per_transaction(Money::new(dec!(180_000)), 4_000_000);
    let fully_loaded = cost_per_transaction(Money::new(dec!(1_350_000)), 4_000_000);

    assert_eq!(marginal_only.value(), dec!(0.045));
    assert_eq!(fully_loaded.value(), dec!(0.3375));
    assert!(fully_loaded.value() < dec!(2.83)); // still cheaper than the phone comparator
}

/// `channel-shift-savings`: the blue badge renewal worked example. The gross saving is far larger
/// than what a discretely-staffed contact centre can actually realize.
#[test]
fn channel_shift_savings_blue_badge_renewal() {
    let gross = gross_channel_shift_saving(39_000, Money::new(dec!(6.40)), Money::new(dec!(0.30)));
    assert_eq!(gross.value(), dec!(237_900.00));

    let released = fte_released(14_000, 8_000);
    assert_eq!(released, 1);

    let realized_staffing_saving = Money::new(dec!(34_000)) * released;
    assert!(realized_staffing_saving.value() < gross.value());
}

/// `digital-inclusion`: the Universal-Credit-style national benefit service worked example, blended
/// cost per transaction ≈£1.31.
#[test]
fn digital_inclusion_national_benefit_service() {
    let blended = blended_cost_per_transaction(250_000, Money::new(dec!(9.50)), 2_250_000, Money::new(dec!(0.40)));
    assert_eq!(blended.value().round_dp(2), dec!(1.31));
}

/// `government-as-a-platform`: the GOV.UK Pay adoption worked example, £73,000 first-year saving.
#[test]
fn government_as_a_platform_pay_adoption() {
    let saving = platform_adoption_saving(Money::new(dec!(85_000)), Money::new(dec!(12_000)));
    assert_eq!(saving.value(), dec!(73_000));
}

/// `open-data-value`: the illustrative national address-data worked example, £60m/year cost-avoided
/// lower bound.
#[test]
fn open_data_value_address_dataset() {
    let value = cost_avoided_open_data_value(15_000, Money::new(dec!(4_000)));
    assert_eq!(value.value(), dec!(60_000_000));
}

/// `public-sector-cybersecurity-value`: the county council case-management system worked example,
/// a security control worth £60,000/year net.
#[test]
fn public_sector_cybersecurity_case_management_system() {
    let ale_before = annualized_loss_expectancy(Money::new(dec!(2_100_000)), Percentage::from_percent(8.0));
    let ale_after = annualized_loss_expectancy(Money::new(dec!(2_100_000)), Percentage::from_percent(3.0));

    assert_eq!(ale_before.value().round_dp(0), dec!(168_000));
    assert_eq!(ale_after.value().round_dp(0), dec!(63_000));

    let value = security_control_value(ale_before, ale_after, Money::new(dec!(45_000)));
    assert_eq!(value.value().round_dp(0), dec!(60_000));
}

/// `ai-in-government-value`: the council tax enquiry drafting tool worked example. The real saving
/// (£61,333/year) is well under half the pilot's headline claim.
#[test]
fn ai_in_government_value_council_tax_enquiries() {
    let baseline = handling_time_cost(25_000, dec!(14), Money::new(dec!(34)));
    assert_eq!(baseline.value().round_dp(2), dec!(198_333.33));

    let pilot_headline_cost = handling_time_cost(25_000, dec!(3), Money::new(dec!(34)));
    let pilot_claimed_saving = baseline - pilot_headline_cost;

    let production_review_cost = handling_time_cost(25_000, dec!(6), Money::new(dec!(34)));
    let real_net = net_ai_value(baseline, production_review_cost, Money::new(dec!(38_000)), Money::new(dec!(14_000)));

    assert_eq!(real_net.value().round_dp(0), dec!(61_333));
    assert!(real_net.value() < pilot_claimed_saving.value() / dec!(2));
}
