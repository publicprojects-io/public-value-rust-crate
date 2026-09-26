//! Comprehensive tests for the metrics beyond the base 64 topics — see the "Extensions" section of
//! `spec/topics.md` for why each was added and its sources.

use public_value::economic_appraisal::{
    clears_qaly_threshold, disability_adjusted_life_years, qaly, years_lived_with_disability,
    years_of_life_lost,
};
use public_value::performance_metrics::net_promoter_score;
use public_value::societal_indicators::{
    atkinson_inequality_measure, gini_coefficient, human_development_index, ihdi_loss_percentage,
    inequality_adjusted_dimension_index, inequality_adjusted_hdi,
};
use rust_decimal_macros::dec;
use rusty_money::{Money, iso};

/// QALY: a treatment gaining 5 years at 0.7 utility, for £100,000, against a comparator gaining 3
/// years at 0.7 utility for £30,000, gives an ICER that clears NICE's 2026 standard threshold band.
#[test]
fn qaly_and_nice_threshold() {
    let qalys_treatment = qaly(5.0, 0.7);
    let qalys_comparator = qaly(3.0, 0.7);
    assert!((qalys_treatment - 3.5).abs() < 1e-9);
    assert!((qalys_comparator - 2.1).abs() < 1e-9);

    let incremental_qalys = qalys_treatment - qalys_comparator;
    assert!((incremental_qalys - 1.4).abs() < 1e-9);
    let incremental_cost = Money::from_decimal(dec!(100_000), iso::USD).sub(Money::from_decimal(dec!(30_000), iso::USD)).unwrap();
    assert_eq!(*incremental_cost.amount(), dec!(70_000));

    let cost_per_incremental_qaly = incremental_cost.div(dec!(1.4)).unwrap(); // £70,000 / 1.4 QALYs
    assert!((cost_per_incremental_qaly.amount() - dec!(50_000)).abs() < dec!(1));
    assert!(clears_qaly_threshold(cost_per_incremental_qaly, Money::from_decimal(dec!(51_000), iso::USD)));
    assert!(!clears_qaly_threshold(cost_per_incremental_qaly, Money::from_decimal(dec!(35_000), iso::USD)));
}

/// DALY: years of life lost plus years lived with disability.
#[test]
fn daly_components() {
    let yll = years_of_life_lost(40, 30.0);
    let yld = years_lived_with_disability(500, 4.0, 0.2);
    let daly = disability_adjusted_life_years(yll, yld);

    assert!((yll - 1_200.0).abs() < 1e-9);
    assert!((yld - 400.0).abs() < 1e-9);
    assert!((daly - 1_600.0).abs() < 1e-9);
}

/// Gini coefficient: a moderately unequal quintile income distribution.
#[test]
fn gini_coefficient_quintile_distribution() {
    let perfectly_equal = gini_coefficient(&[0.2, 0.4, 0.6, 0.8, 1.0], &[0.2, 0.4, 0.6, 0.8, 1.0]);
    let unequal = gini_coefficient(&[0.2, 0.4, 0.6, 0.8, 1.0], &[0.04, 0.14, 0.30, 0.54, 1.00]);

    assert!(perfectly_equal.abs() < 1e-9);
    assert!((unequal - 0.392).abs() < 0.001);
    assert!(unequal > perfectly_equal);
}

/// IHDI: the middle-income-country HDI worked example, discounted by illustrative per-dimension
/// Atkinson inequality measures.
#[test]
fn inequality_adjusted_hdi_example() {
    let hdi = human_development_index(72.0, 8.0, 13.0, 12_000.0);

    let a_health = atkinson_inequality_measure(0.95, 1.0);
    let a_education = atkinson_inequality_measure(0.90, 1.0);
    let a_income = atkinson_inequality_measure(0.80, 1.0);
    assert!((a_health - 0.05).abs() < 1e-9);

    let ihdi = inequality_adjusted_hdi(
        inequality_adjusted_dimension_index(hdi.life_expectancy_index, a_health),
        inequality_adjusted_dimension_index(hdi.education_index, a_education),
        inequality_adjusted_dimension_index(hdi.income_index, a_income),
    );
    assert!((ihdi - 0.6287).abs() < 0.001);
    assert!(ihdi < hdi.hdi); // IHDI never exceeds HDI once any dimension has inequality

    let loss = ihdi_loss_percentage(hdi.hdi, ihdi);
    assert!((loss.as_percent() - 11.89).abs() < 0.1);
}

/// Net Promoter Score: a citizen-satisfaction survey read through the promoter/detractor lens.
#[test]
fn net_promoter_score_citizen_survey() {
    let nps = net_promoter_score(620, 180, 1_000);
    assert!((nps - 44.0).abs() < 1e-9);

    // All-promoter and all-detractor edge cases land on the scale's endpoints.
    assert!((net_promoter_score(1_000, 0, 1_000) - 100.0).abs() < 1e-9);
    assert!((net_promoter_score(0, 1_000, 1_000) - -100.0).abs() < 1e-9);
}
