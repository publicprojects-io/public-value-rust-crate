//! Comprehensive tests for Group 7 — `societal_indicators` — reproducing the worked examples from
//! the source topics in `~/git/public-value-metrics` (see `spec/topics.md`).

use public_value::economic_appraisal::{monetize_wellbys, wellbys};
use public_value::societal_indicators::{
    ecosystem_asset_value, genuine_progress_indicator, gnh_sufficiency, human_development_index,
    imd_composite_score, imd_decile, imd_domain_contribution, mpi_headcount_ratio,
    multidimensional_poverty_index, present_value_across_schedule, social_capital_pillar_gap,
};
use public_value::units::{Percentage, money_ratio};
use rust_decimal_macros::dec;
use rusty_money::{Money, iso};

/// `gdp-alternatives`: the region's GPI worked example (49 $bn, below the $55bn GDP figure) and the
/// GNH sufficiency worked example (7 of 9 domains clears the 6-domain bar).
#[test]
fn gdp_alternatives_examples() {
    let gpi = genuine_progress_indicator(
        Money::from_decimal(dec!(50), iso::USD),
        Money::from_decimal(dec!(12), iso::USD),
        Money::from_decimal(dec!(3), iso::USD).add(Money::from_decimal(dec!(4), iso::USD)).unwrap(),
        Money::from_decimal(dec!(6), iso::USD),
    );
    assert_eq!(*gpi.amount(), dec!(49));
    assert!(*gpi.amount() < dec!(55)); // GDP grew to $55bn while GPI did not keep pace

    assert!(gnh_sufficiency(7, 6));
    assert!(!gnh_sufficiency(5, 6));
}

/// `human-development-index`: the middle-income country worked example, HDI ≈ 0.713, and the
/// pitfall check that a geometric mean penalizes imbalance an arithmetic mean would hide.
#[test]
fn human_development_index_middle_income_country() {
    let result = human_development_index(72.0, 8.0, 13.0, 12_000.0);
    assert!((result.life_expectancy_index - 0.800).abs() < 0.001);
    assert!((result.education_index - 0.628).abs() < 0.001);
    assert!((result.income_index - 0.723).abs() < 0.001);
    assert!((result.hdi - 0.713).abs() < 0.001);

    // If mean years of schooling were 4 instead of 8, HDI should drop a full band even though
    // nothing else changed (the source doc's sensitivity illustration).
    let weaker_schooling = human_development_index(72.0, 4.0, 13.0, 12_000.0);
    assert!((weaker_schooling.hdi - 0.659).abs() < 0.01);
    assert!(weaker_schooling.hdi < result.hdi);

    let arithmetic_mean =
        (result.life_expectancy_index + result.education_index + result.income_index) / 3.0;
    assert!(result.hdi < arithmetic_mean); // geometric mean never exceeds arithmetic mean
}

/// `multidimensional-poverty-index`: the national-survey worked example (MPI = 0.1575) and the
/// two-district comparison where equal headcount hides a 50% higher MPI in the more severely
/// deprived district.
#[test]
fn multidimensional_poverty_index_examples() {
    let h = mpi_headcount_ratio(350, 1_000);
    let mpi = multidimensional_poverty_index(h, Percentage::from_percent(45.0));
    assert!((mpi - 0.1575).abs() < 0.0001);

    let mpi_district_a = multidimensional_poverty_index(Percentage::from_percent(30.0), Percentage::from_percent(40.0));
    let mpi_district_b = multidimensional_poverty_index(Percentage::from_percent(30.0), Percentage::from_percent(60.0));
    assert!((mpi_district_a - 0.120).abs() < 0.0001);
    assert!((mpi_district_b - 0.180).abs() < 0.0001);
    assert!(mpi_district_b > mpi_district_a);
}

/// `wellbeing-adjusted-life-years`: the loneliness-service worked example, reusing the WELLBY
/// functions from `economic_appraisal` (the same formula the `wellbeing-valuation` topic ports).
#[test]
fn wellbeing_adjusted_life_years_loneliness_service() {
    let total = wellbys(400, 0.8, 2.0);
    assert!((total - 640.0).abs() < 1e-9);

    let value = monetize_wellbys(total, Money::from_decimal(dec!(13_000), iso::USD));
    assert_eq!(*value.amount(), dec!(8_320_000));

    let bcr = money_ratio(value, Money::from_decimal(dec!(600_000), iso::USD));
    assert!((bcr.value() - 13.9).abs() < 0.1);
}

/// `index-of-multiple-deprivation`: the illustrative LSOA composite-score worked example (≈0.21489)
/// and its decile (1, the most deprived 10%).
#[test]
fn index_of_multiple_deprivation_lsoa_example() {
    let contributions = [
        imd_domain_contribution(0.35, Percentage::from_percent(22.5)),
        imd_domain_contribution(0.30, Percentage::from_percent(22.5)),
        imd_domain_contribution(0.20, Percentage::from_percent(13.5)),
        imd_domain_contribution(0.15, Percentage::from_percent(13.5)),
        imd_domain_contribution(0.10, Percentage::from_percent(9.3)),
        imd_domain_contribution(0.05, Percentage::from_percent(9.3)),
        imd_domain_contribution(0.08, Percentage::from_percent(9.3)),
    ];
    let composite = imd_composite_score(&contributions);
    assert!((composite - 0.21489).abs() < 0.0001);

    assert_eq!(imd_decile(2_950, 32_844), 1);
}

/// `social-capital-metrics`: the neighbourhood snapshot worked example. Civic engagement (a
/// 6-point gap) is a larger relative deficit than trust (a 4-point gap).
#[test]
fn social_capital_metrics_neighbourhood_snapshot() {
    let trust_gap = social_capital_pillar_gap(Percentage::from_percent(41.0), Percentage::from_percent(45.0));
    let civic_engagement_gap = social_capital_pillar_gap(Percentage::from_percent(24.0), Percentage::from_percent(30.0));

    assert!((trust_gap.as_percent() - 4.0).abs() < 0.01);
    assert!((civic_engagement_gap.as_percent() - 6.0).abs() < 0.01);
    assert!(civic_engagement_gap.as_percent() > trust_gap.as_percent());
}

/// `natural-capital-accounting`: the urban woodland worked example, combining recreational and
/// carbon-storage value over a 50-year horizon at 3.5%.
#[test]
fn natural_capital_accounting_urban_woodland() {
    let recreational_annual = Money::from_decimal(dec!(3), iso::USD).mul(80_000_u32).unwrap();
    let carbon_annual = Money::from_decimal(dec!(75), iso::USD).mul(400_u32).unwrap();

    let recreational_asset = ecosystem_asset_value(recreational_annual, dec!(0.035), 50);
    let carbon_asset = ecosystem_asset_value(carbon_annual, dec!(0.035), 50);
    let total_asset = recreational_asset.add(carbon_asset).unwrap();

    assert!((recreational_asset.amount() - dec!(5_629_348)).abs() < dec!(1_000));
    assert!((carbon_asset.amount() - dec!(703_668)).abs() < dec!(1_000));
    assert!((total_asset.amount() - dec!(6_333_017)).abs() < dec!(1_000));
}

/// `intergenerational-equity-and-sustainability-discounting`: the three-regime worked example for
/// £1 of avoided harm in 100 years. The discounting convention chosen changes the answer by
/// roughly eightfold.
#[test]
fn intergenerational_equity_three_discounting_regimes() {
    let flat = present_value_across_schedule(Money::from_decimal(dec!(1), iso::USD), &[(dec!(0.035), 100)]);
    let declining = present_value_across_schedule(Money::from_decimal(dec!(1), iso::USD), &[(dec!(0.035), 30), (dec!(0.03), 45), (dec!(0.025), 25)]);
    let stern = present_value_across_schedule(Money::from_decimal(dec!(1), iso::USD), &[(dec!(0.014), 100)]);

    assert!((flat.amount() - dec!(0.032)).abs() < dec!(0.001));
    assert!((declining.amount() - dec!(0.051)).abs() < dec!(0.001));
    assert!((stern.amount() - dec!(0.249)).abs() < dec!(0.001));
    assert!(stern.gt(&declining).unwrap() && declining.gt(&flat).unwrap());
}
