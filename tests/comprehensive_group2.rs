//! Comprehensive tests for Group 2 — `economic_appraisal` — reproducing the worked examples from
//! the source topics in `~/git/public-value-metrics` (see `spec/topics.md`).

use public_value::economic_appraisal::{
    aggregate_stated_preference_value, annuity_factor, benefit_cost_ratio, cost_effectiveness_ratio,
    hedonic_implicit_price, incremental_cost_effectiveness_ratio, monetize_wellbys,
    net_present_social_value, net_social_benefit_per_hour, shadow_carbon_benefit, shadow_wage_rate,
    travel_cost_annual_value, weighted_score, wellbys, FiveCaseModel,
};
use public_value::units::money_ratio;
use rust_decimal_macros::dec;
use rusty_money::{Money, iso};

/// `green-book-appraisal`: a strong economic case does not excuse failing the commercial case.
#[test]
fn five_case_model_single_tender_supplier() {
    let single_tender = FiveCaseModel {
        strategic_case: true,
        economic_case: true,
        commercial_case: false,
        financial_case: true,
        management_case: true,
    };
    assert!(!single_tender.proceeds());

    let fully_cleared = FiveCaseModel {
        strategic_case: true,
        economic_case: true,
        commercial_case: true,
        financial_case: true,
        management_case: true,
    };
    assert!(fully_cleared.proceeds());
}

/// `social-cost-benefit-analysis`: the cycling-network worked example, NPSV ≈ +£0.27m and BCR ≈
/// 1.07 ("low" value for money).
#[test]
fn social_cost_benefit_analysis_cycling_network() {
    let factor = annuity_factor(dec!(0.035), 20);
    let maintenance_pv = Money::from_decimal(dec!(50_000), iso::USD).mul(factor).unwrap();
    let cost_pv = Money::from_decimal(dec!(3_000_000), iso::USD).add(maintenance_pv).unwrap();
    let benefit_pv = Money::from_decimal(dec!(280_000), iso::USD).mul(factor).unwrap();

    let npsv = net_present_social_value(benefit_pv, cost_pv);
    let bcr = benefit_cost_ratio(benefit_pv, cost_pv);

    assert!((npsv.amount() - dec!(268_852)).abs() < dec!(1_000));
    assert!((bcr.value() - 1.072).abs() < 0.01);
}

/// `cost-effectiveness-analysis-in-government`: the rough-sleeping worked example. Outreach is
/// dominated on average cost, but the incremental step to Hostel is cheaper than the incremental
/// step from Hostel to Housing First.
#[test]
fn cost_effectiveness_rough_sleeping_options() {
    let housing_first = cost_effectiveness_ratio(Money::from_decimal(dec!(900_000), iso::USD), 60);
    let hostel = cost_effectiveness_ratio(Money::from_decimal(dec!(600_000), iso::USD), 50);
    let outreach = cost_effectiveness_ratio(Money::from_decimal(dec!(350_000), iso::USD), 20);

    assert_eq!(*housing_first.amount(), dec!(15_000));
    assert_eq!(*hostel.amount(), dec!(12_000));
    assert_eq!(*outreach.amount(), dec!(17_500));

    let icer_hostel_vs_outreach =
        incremental_cost_effectiveness_ratio(Money::from_decimal(dec!(600_000), iso::USD), 50, Money::from_decimal(dec!(350_000), iso::USD), 20);
    let icer_housing_first_vs_hostel =
        incremental_cost_effectiveness_ratio(Money::from_decimal(dec!(900_000), iso::USD), 60, Money::from_decimal(dec!(600_000), iso::USD), 50);

    assert!((icer_hostel_vs_outreach.amount() - dec!(8333.33)).abs() < dec!(1));
    assert_eq!(*icer_housing_first_vs_hostel.amount(), dec!(30_000));
    assert!(icer_hostel_vs_outreach.lt(&icer_housing_first_vs_hostel).unwrap());
}

/// `multi-criteria-decision-analysis`: the recycling-centre site-selection worked example. Site C
/// ranks highest.
#[test]
fn multi_criteria_decision_analysis_site_selection() {
    let weights = [0.30, 0.25, 0.25, 0.20];
    let site_a = weighted_score(&[80.0, 60.0, 40.0, 70.0], &weights);
    let site_b = weighted_score(&[60.0, 90.0, 70.0, 50.0], &weights);
    let site_c = weighted_score(&[90.0, 50.0, 80.0, 60.0], &weights);

    assert!((site_a - 63.0).abs() < 1e-9);
    assert!((site_b - 68.0).abs() < 1e-9);
    assert!((site_c - 71.5).abs() < 1e-9);
    assert!(site_c > site_b && site_b > site_a);
}

/// `wellbeing-valuation`: the community befriending scheme worked example, 560 WELLBYs monetized
/// at £13,000 each to £7.28m against a £450,000 programme cost.
#[test]
fn wellbeing_valuation_befriending_scheme() {
    let total = wellbys(400, 0.7, 2.0);
    assert!((total - 560.0).abs() < 1e-9);

    let value = monetize_wellbys(total, Money::from_decimal(dec!(13_000), iso::USD));
    assert_eq!(*value.amount(), dec!(7_280_000));

    let bcr = money_ratio(value, Money::from_decimal(dec!(450_000), iso::USD));
    assert!((bcr.value() - 16.18).abs() < 0.1);
}

/// `stated-preference-valuation`: the Defra water-quality contingent-valuation worked example,
/// aggregating to £9.52m/year and ≈£135m over 20 years at 3.5%.
#[test]
fn stated_preference_water_quality() {
    let aggregate = aggregate_stated_preference_value(Money::from_decimal(dec!(28), iso::USD), 340_000);
    assert_eq!(*aggregate.amount(), dec!(9_520_000));

    let factor = annuity_factor(dec!(0.035), 20);
    let pv = aggregate.mul(factor).unwrap();
    assert!((pv.amount() - dec!(135_302_079)).abs() < dec!(500_000));
}

/// `revealed-preference-valuation`: the aircraft-noise hedonic worked example (£75.6m aggregate
/// cost) and the nature-reserve travel-cost worked example (£920,000/year).
#[test]
fn revealed_preference_examples() {
    let implicit_price = hedonic_implicit_price(Money::from_decimal(dec!(280_000), iso::USD), 0.005);
    assert_eq!(implicit_price.amount().round_dp(2), dec!(1_400));

    let aggregate_noise_cost = implicit_price.mul(dec!(3)).unwrap().mul(18_000_u32).unwrap();
    assert_eq!(aggregate_noise_cost.amount().round_dp(2), dec!(75_600_000));

    let value = travel_cost_annual_value(40_000, Money::from_decimal(dec!(14), iso::USD), Money::from_decimal(dec!(9), iso::USD));
    assert_eq!(*value.amount(), dec!(920_000));
}

/// `shadow-pricing`: the flood-defence carbon-benefit worked example (£112,000) and the
/// employment-programme shadow-wage worked example (£4.40/hour net social benefit).
#[test]
fn shadow_pricing_examples() {
    let carbon_benefit = shadow_carbon_benefit(400, Money::from_decimal(dec!(280), iso::USD));
    assert_eq!(*carbon_benefit.amount(), dec!(112_000));

    let market_wage = Money::from_decimal(dec!(11.00), iso::USD);
    let shadow_wage = shadow_wage_rate(market_wage, dec!(0.6));
    let net_benefit = net_social_benefit_per_hour(market_wage, shadow_wage);

    assert_eq!(*shadow_wage.amount(), dec!(6.600));
    assert_eq!(*net_benefit.amount(), dec!(4.400));
}
