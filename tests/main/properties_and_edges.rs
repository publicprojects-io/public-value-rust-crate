//! Cross-module properties and edge cases that complement the source worked examples.

use proptest::prelude::*;
use public_value::delivery_connection::technical_debt_ratio;
use public_value::economic_appraisal::internal_rate_of_return;
use public_value::foundations::discount_factor;
use public_value::impact_measurement::{CombinedDiagnosis, diagnose};
use public_value::societal_indicators::gini_coefficient;
use public_value::units::{Money, iso, money_ratio};
use rust_decimal::Decimal;

proptest! {
    #[test]
    fn money_ratios_are_reciprocal(a in 1_i64..1_000_000, b in 1_i64..1_000_000) {
        let a = Money::from_decimal(Decimal::from(a), iso::USD);
        let b = Money::from_decimal(Decimal::from(b), iso::USD);

        let forward = money_ratio(a, b).value();
        let reverse = money_ratio(b, a).value();

        prop_assert!((forward * reverse - 1.0).abs() < 1e-10);
    }

    #[test]
    fn discount_factor_increases_with_the_rate(
        lower_basis_points in 0_i64..1_000,
        rate_gap_basis_points in 1_i64..1_000,
        years in 1_u32..100,
    ) {
        let lower = Decimal::new(lower_basis_points, 4);
        let higher = Decimal::new(lower_basis_points + rate_gap_basis_points, 4);

        // A larger discount factor means the corresponding present value decreases.
        prop_assert!(discount_factor(lower, years) < discount_factor(higher, years));
    }

    #[test]
    fn gini_is_bounded_for_non_negative_distributions(
        mut incomes in proptest::collection::vec(1_u32..100_000, 1..30),
    ) {
        incomes.sort_unstable();
        let population = f64::from(u32::try_from(incomes.len()).expect("test vector length fits u32"));
        let total_income = incomes.iter().map(|&income| f64::from(income)).sum::<f64>();
        let mut running_income = 0.0;
        let mut population_shares = Vec::with_capacity(incomes.len());
        let mut income_shares = Vec::with_capacity(incomes.len());

        for (index, income) in incomes.into_iter().enumerate() {
            running_income += f64::from(income);
            let cumulative_population =
                u32::try_from(index + 1).expect("test vector index fits u32");
            population_shares.push(f64::from(cumulative_population) / population);
            income_shares.push(running_income / total_income);
        }

        let gini = gini_coefficient(&population_shares, &income_shares);
        prop_assert!(gini >= -1e-12);
        prop_assert!(gini <= 1.0 + 1e-12);
    }
}

#[test]
fn gini_equality_and_single_member_are_zero() {
    assert!(gini_coefficient(&[0.25, 0.5, 0.75, 1.0], &[0.25, 0.5, 0.75, 1.0]).abs() < 1e-12);
    assert!(gini_coefficient(&[1.0], &[1.0]).abs() < 1e-12);
}

#[test]
#[should_panic(expected = "same length")]
fn gini_rejects_mismatched_slices() {
    let _ = gini_coefficient(&[1.0], &[0.5, 1.0]);
}

#[test]
#[should_panic(expected = "must not be empty")]
fn gini_rejects_empty_slices() {
    let _ = gini_coefficient(&[], &[]);
}

#[test]
fn irr_reports_no_root_and_honours_zero_iterations() {
    assert_eq!(internal_rate_of_return(&[100.0, 50.0], 1e-9, 200), None);
    assert_eq!(
        internal_rate_of_return(&[-100.0, 110.0], 1e-9, 0),
        Some(-0.99)
    );
}

#[test]
fn every_combined_diagnosis_is_reachable() {
    assert_eq!(diagnose(false, true), CombinedDiagnosis::TheoryFailure);
    assert_eq!(
        diagnose(false, false),
        CombinedDiagnosis::ImplementationFailure
    );
    assert_eq!(
        diagnose(true, true),
        CombinedDiagnosis::ReplicateWithConfidence
    );
    assert_eq!(diagnose(true, false), CombinedDiagnosis::InvestigateFurther);
}

#[test]
fn technical_debt_ratio_handles_exact_boundaries() {
    let remediation = Money::from_decimal(Decimal::from(5), iso::USD);
    let redevelopment = Money::from_decimal(Decimal::from(100), iso::USD);
    assert!((technical_debt_ratio(remediation, redevelopment).as_percent() - 5.0).abs() < 1e-12);
}

#[test]
fn demo_binary_prints_its_worked_example() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_public-value"))
        .output()
        .expect("demo binary runs");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Cost per outcome: $714.29"));
    assert!(stdout.contains("Social return on investment: 1.44:1"));
    assert!(stdout.contains("Human Development Index:"));
    assert!(stdout.contains("Net public value: $250,000.00"));
    assert!(stdout.contains("Cost of delay: $10,000.00/week"));
    assert!(stdout.contains("Net Promoter Score: 42"));
    assert_eq!(output.stderr, Vec::<u8>::new());

    let help = std::process::Command::new(env!("CARGO_BIN_EXE_public-value"))
        .arg("--help")
        .output()
        .expect("demo help runs");
    assert!(help.status.success());
    assert!(
        String::from_utf8(help.stdout)
            .unwrap()
            .contains("Usage: public-value [--help]")
    );
}
