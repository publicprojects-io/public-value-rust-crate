//! Governmental economic appraisal: the Green Book five-case model, social cost-benefit analysis,
//! cost-effectiveness analysis, multi-criteria decision analysis, and the valuation methods
//! (wellbeing, stated preference, revealed preference, shadow pricing) that price what markets
//! never priced.

use rust_decimal::Decimal;

use crate::foundations::discount_factor;
use crate::units::{Money, Ratio};

/// HM Treasury's five-case model: a business case must clear the strategic, economic, commercial,
/// financial, and management cases independently — a proposal can fail on any one regardless of
/// how strong the others are.
///
/// Ported from `green-book-appraisal`.
// Five independent yes/no gates is the Green Book's actual structure (not a state machine or a
// mutually exclusive choice, which is what this lint steers toward), so five `bool` fields is the
// correct shape here.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FiveCaseModel {
    /// Is there evidence of a spending objective linked to organisational strategy?
    pub strategic_case: bool,
    /// Does the preferred option maximize net public value against a "do minimum" baseline?
    pub economic_case: bool,
    /// Is the preferred option procurable on acceptable terms?
    pub commercial_case: bool,
    /// Can the organization afford it, this year and every year after?
    pub financial_case: bool,
    /// Can the organization actually deliver it?
    pub management_case: bool,
}

impl FiveCaseModel {
    /// Proceeds only if all five cases pass. A strong economic case (e.g. a high NPV) does not
    /// excuse failing the commercial or management case — see the source doc's single-tender-supplier
    /// worked example, where a £40m NPV proposal still fails without competitive tension.
    ///
    /// # Examples
    ///
    /// ```
    /// use public_value::economic_appraisal::FiveCaseModel;
    ///
    /// // Strong economic case, but only one accredited supplier exists.
    /// let single_tender = FiveCaseModel {
    ///     strategic_case: true,
    ///     economic_case: true,
    ///     commercial_case: false,
    ///     financial_case: true,
    ///     management_case: true,
    /// };
    /// assert!(!single_tender.proceeds());
    /// ```
    #[must_use]
    pub const fn proceeds(self) -> bool {
        self.strategic_case && self.economic_case && self.commercial_case && self.financial_case && self.management_case
    }
}

/// The present-value annuity factor `[1 − (1+r)^−n] / r`: the present value of £1 received at the
/// end of each of `years` years, used throughout Green Book appraisal to convert a level annual
/// cost or benefit stream into a lump-sum present value.
///
/// Ported from `social-cost-benefit-analysis` (and reused by `stated-preference-valuation`'s
/// worked example). Built on [`crate::foundations::discount_factor`] rather than duplicating the
/// compounding logic.
///
/// # Panics
///
/// Panics if `annual_rate` is zero.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::annuity_factor;
/// use rust_decimal_macros::dec;
///
/// let factor = annuity_factor(dec!(0.035), 20);
/// assert!((factor - dec!(14.2124)).abs() < dec!(0.001));
/// ```
#[must_use]
pub fn annuity_factor(annual_rate: Decimal, years: u32) -> Decimal {
    let factor = discount_factor(annual_rate, years);
    (Decimal::ONE - Decimal::ONE / factor) / annual_rate
}

/// Net present social value: the present value of benefits minus the present value of costs.
///
/// Ported from `social-cost-benefit-analysis`.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::{annuity_factor, net_present_social_value};
/// use public_value::units::Money;
/// use rust_decimal::Decimal;
/// use rust_decimal_macros::dec;
///
/// // Cycling network: £3m capital plus £50,000/year maintenance against £280,000/year benefits,
/// // over 20 years at 3.5%.
/// let factor = annuity_factor(dec!(0.035), 20);
/// let cost_pv = Money::new(dec!(3_000_000)) + Money::new(dec!(50_000)) * factor;
/// let benefit_pv = Money::new(dec!(280_000)) * factor;
/// let npsv = net_present_social_value(benefit_pv, cost_pv);
/// assert!(npsv.value() > Decimal::ZERO);
/// assert!((npsv.value() - dec!(268_852)).abs() < dec!(1_000));
/// ```
#[must_use]
pub fn net_present_social_value(present_value_of_benefits: Money, present_value_of_costs: Money) -> Money {
    present_value_of_benefits - present_value_of_costs
}

/// The benefit-cost ratio: present value of benefits divided by present value of costs. Above 1.0
/// indicates net social value; the Green Book labels 1.0–1.5 "low", 1.5–2.0 "medium", 2.0–4.0
/// "high", and above 4.0 "very high" value for money.
///
/// Ported from `social-cost-benefit-analysis`.
///
/// # Panics
///
/// Panics if `present_value_of_costs` is zero.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::{annuity_factor, benefit_cost_ratio};
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// let factor = annuity_factor(dec!(0.035), 20);
/// let cost_pv = Money::new(dec!(3_000_000)) + Money::new(dec!(50_000)) * factor;
/// let benefit_pv = Money::new(dec!(280_000)) * factor;
/// let bcr = benefit_cost_ratio(benefit_pv, cost_pv);
/// assert!((bcr.value() - 1.072).abs() < 0.01);
/// ```
#[must_use]
pub fn benefit_cost_ratio(present_value_of_benefits: Money, present_value_of_costs: Money) -> Ratio {
    present_value_of_benefits.ratio_to(present_value_of_costs)
}

/// The (average) cost-effectiveness ratio: total cost divided by outcome units achieved, in the
/// outcome's own natural unit rather than money.
///
/// Ported from `cost-effectiveness-analysis-in-government`.
///
/// # Panics
///
/// Panics if `outcome_units_achieved` is zero.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::cost_effectiveness_ratio;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // Housing First: £900,000 for 60 people moved into settled accommodation.
/// let cer = cost_effectiveness_ratio(Money::new(dec!(900_000)), 60);
/// assert_eq!(cer.value(), dec!(15_000));
/// ```
#[must_use]
pub fn cost_effectiveness_ratio(total_cost: Money, outcome_units_achieved: u32) -> Money {
    total_cost / outcome_units_achieved
}

/// The incremental cost-effectiveness ratio (ICER) between two options: the extra cost of option A
/// over option B, divided by the extra outcome units A achieves over B.
///
/// Ported from `cost-effectiveness-analysis-in-government`. Ranking by the *incremental*, not
/// average, ratio matters when scaling up a budget-constrained programme: the source doc's worked
/// example shows the cheapest average-cost option is not always the cheapest next unit to buy.
///
/// # Panics
///
/// Panics if `outcome_a` equals `outcome_b`.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::incremental_cost_effectiveness_ratio;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // Hostel + move-on support (£600,000, 50 outcomes) versus outreach (£350,000, 20 outcomes).
/// let icer = incremental_cost_effectiveness_ratio(Money::new(dec!(600_000)), 50, Money::new(dec!(350_000)), 20);
/// assert!((icer.value() - dec!(8333.33)).abs() < dec!(1));
/// ```
#[must_use]
pub fn incremental_cost_effectiveness_ratio(cost_a: Money, outcome_a: u32, cost_b: Money, outcome_b: u32) -> Money {
    let cost_diff = cost_a - cost_b;
    let outcome_diff = i64::from(outcome_a) - i64::from(outcome_b);
    cost_diff / Decimal::from(outcome_diff)
}

/// A multi-criteria decision analysis weighted score: `Σ (score_j × weight_j)` across criteria.
///
/// Ported from `multi-criteria-decision-analysis`. `scores` and `weights` must be the same length
/// (weights conventionally sum to 1.0, agreed and published *before* any option is scored — the
/// Green Book's central safeguard against working backwards from a preferred option).
///
/// # Panics
///
/// Panics if `scores` and `weights` have different lengths (in debug builds; in release builds the
/// shorter slice truncates the zip silently, which is why callers should validate lengths match).
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::weighted_score;
///
/// // Site C: cost 90, access 50, community 80, environment 60, weighted 30/25/25/20%.
/// let weights = [0.30, 0.25, 0.25, 0.20];
/// let site_c = weighted_score(&[90.0, 50.0, 80.0, 60.0], &weights);
/// assert!((site_c - 71.5).abs() < 1e-9);
/// ```
#[must_use]
pub fn weighted_score(scores: &[f64], weights: &[f64]) -> f64 {
    debug_assert_eq!(scores.len(), weights.len(), "scores and weights must have the same length");
    scores.iter().zip(weights).map(|(score, weight)| score * weight).sum()
}

/// The total WELLBYs (wellbeing-adjusted life years) a policy generates: the change in
/// life-satisfaction score, times the number of people affected, times the duration in years.
///
/// Ported from `wellbeing-valuation`.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::wellbys;
///
/// // Community befriending scheme: 400 people, 0.7-point life-satisfaction gain, 2 years.
/// let total = wellbys(400, 0.7, 2.0);
/// assert!((total - 560.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn wellbys(people_affected: u32, life_satisfaction_change: f64, duration_years: f64) -> f64 {
    f64::from(people_affected) * life_satisfaction_change * duration_years
}

/// Converts a WELLBY total to a monetary value using HM Treasury's recommended value per WELLBY
/// (£13,000 at 2021 prices — passed in by the caller since Treasury revises it periodically, not
/// hard-coded here).
///
/// Ported from `wellbeing-valuation`.
///
/// # Panics
///
/// Panics if `total_wellbys` cannot be represented as a `Decimal` (not reachable for a finite
/// value).
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::{monetize_wellbys, wellbys};
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// let total = wellbys(400, 0.7, 2.0);
/// let value = monetize_wellbys(total, Money::new(dec!(13_000)));
/// assert_eq!(value.value(), dec!(7_280_000));
/// ```
#[must_use]
pub fn monetize_wellbys(total_wellbys: f64, value_per_wellby: Money) -> Money {
    let wellbys_decimal = Decimal::from_f64_retain(total_wellbys).expect("finite WELLBY total converts to Decimal");
    value_per_wellby * wellbys_decimal
}

/// The aggregate stated-preference value: mean willingness-to-pay per household times the number
/// of households affected.
///
/// Ported from `stated-preference-valuation`.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::aggregate_stated_preference_value;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // Defra water-quality survey: mean WTP £28/household/year across 340,000 households.
/// let aggregate = aggregate_stated_preference_value(Money::new(dec!(28)), 340_000);
/// assert_eq!(aggregate.value(), dec!(9_520_000));
/// ```
#[must_use]
pub fn aggregate_stated_preference_value(mean_willingness_to_pay: Money, affected_population: u32) -> Money {
    mean_willingness_to_pay * affected_population
}

/// The hedonic implicit price of a one-unit change in a non-market attribute (e.g. one decibel of
/// noise), inferred from its estimated fractional effect on a market price such as a house price.
///
/// Ported from `revealed-preference-valuation`.
///
/// # Panics
///
/// Panics if `fractional_price_change_per_unit` cannot be represented as a `Decimal` (not
/// reachable for a finite value).
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::hedonic_implicit_price;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // Each 1dB of aircraft noise is associated with a 0.5% house-price reduction on a £280,000 house.
/// // The 0.5% figure is an approximate regression coefficient, not an exact decimal amount, so
/// // converting it via `f64` can leave a sub-penny remainder; round before comparing.
/// let implicit_price = hedonic_implicit_price(Money::new(dec!(280_000)), 0.005);
/// assert_eq!(implicit_price.value().round_dp(2), dec!(1_400));
/// ```
#[must_use]
pub fn hedonic_implicit_price(market_price: Money, fractional_price_change_per_unit: f64) -> Money {
    let fraction = Decimal::from_f64_retain(fractional_price_change_per_unit)
        .expect("finite fractional price change converts to Decimal");
    market_price * fraction
}

/// The travel-cost method's total annual recreational value: visits times the sum of actual travel
/// cost spent and the estimated consumer surplus above it.
///
/// Ported from `revealed-preference-valuation`.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::travel_cost_annual_value;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // Nature reserve: 40,000 visits/year, £14 travel cost spent, £9 consumer surplus per visit.
/// let value = travel_cost_annual_value(40_000, Money::new(dec!(14)), Money::new(dec!(9)));
/// assert_eq!(value.value(), dec!(920_000));
/// ```
#[must_use]
pub fn travel_cost_annual_value(visits_per_year: u32, cost_per_visit: Money, consumer_surplus_per_visit: Money) -> Money {
    (cost_per_visit + consumer_surplus_per_visit) * visits_per_year
}

/// The shadow-priced benefit of avoided or abated carbon emissions: tonnes times the official
/// shadow price of carbon for the relevant year.
///
/// Ported from `shadow-pricing`. `price_per_tonne` is a parameter, not a constant, because the
/// official schedule rises over the appraisal period — using one year's value throughout
/// understates later-year benefits, per the source doc's explicit pitfall.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::shadow_carbon_benefit;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// let benefit = shadow_carbon_benefit(400, Money::new(dec!(280)));
/// assert_eq!(benefit.value(), dec!(112_000));
/// ```
#[must_use]
pub fn shadow_carbon_benefit(tonnes_co2e: u32, price_per_tonne: Money) -> Money {
    price_per_tonne * tonnes_co2e
}

/// The shadow wage rate: the market wage scaled down by a fraction reflecting that labour drawn
/// from unemployment is not a full net draw on society's resources.
///
/// Ported from `shadow-pricing`.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::shadow_wage_rate;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// let shadow_wage = shadow_wage_rate(Money::new(dec!(11.00)), dec!(0.6));
/// assert_eq!(shadow_wage.value(), dec!(6.600));
/// ```
#[must_use]
pub fn shadow_wage_rate(market_wage: Money, shadow_fraction: Decimal) -> Money {
    market_wage * shadow_fraction
}

/// The net social benefit per hour of employing previously idle labour: the market wage minus the
/// shadow wage — the "extra" value created, distinct from the wage itself, which is largely a
/// transfer.
///
/// Ported from `shadow-pricing`.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::{net_social_benefit_per_hour, shadow_wage_rate};
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// let market_wage = Money::new(dec!(11.00));
/// let shadow_wage = shadow_wage_rate(market_wage, dec!(0.6));
/// let net_benefit = net_social_benefit_per_hour(market_wage, shadow_wage);
/// assert_eq!(net_benefit.value(), dec!(4.400));
/// ```
#[must_use]
pub fn net_social_benefit_per_hour(market_wage: Money, shadow_wage: Money) -> Money {
    market_wage - shadow_wage
}

/// A Quality-Adjusted Life Year (QALY): years lived in a health state, times a utility weight for
/// that state on a 0 (dead) to 1 (full health) scale.
///
/// This is not one of the 64 topics ported from `public-value-metrics`, but the QALY is the unit
/// `cost-effectiveness-analysis-in-government` explicitly says this crate's
/// [`cost_effectiveness_ratio`] and [`incremental_cost_effectiveness_ratio`] are "structurally
/// identical" to (NICE's own health-economics method), so it is implemented here to complete that
/// connection. See NICE's technology appraisal guidance for the current cost-per-QALY threshold
/// (£20,000–£30,000 through 2025; £25,000–£35,000 from April 2026).
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::qaly;
///
/// // 5 years lived at a 0.7 utility weight (moderate health impairment).
/// let value = qaly(5.0, 0.7);
/// assert!((value - 3.5).abs() < 1e-9);
/// ```
#[must_use]
pub fn qaly(years_in_health_state: f64, utility_weight: f64) -> f64 {
    years_in_health_state * utility_weight
}

/// Whether an incremental cost-effectiveness ratio, denominated in QALYs, clears a NICE-style
/// cost-per-QALY threshold.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::clears_qaly_threshold;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // £20,000 per QALY gained clears NICE's 2026 standard threshold band of £25,000–£35,000.
/// let icer = Money::new(dec!(20_000));
/// assert!(clears_qaly_threshold(icer, Money::new(dec!(25_000))));
/// ```
#[must_use]
pub fn clears_qaly_threshold(cost_per_qaly: Money, threshold_per_qaly: Money) -> bool {
    cost_per_qaly.value() <= threshold_per_qaly.value()
}

/// Years of Life Lost (YLL): the number of deaths times the standard life expectancy remaining at
/// the age of death.
///
/// This is not one of the 64 topics ported from `public-value-metrics`, but it is the mortality
/// half of the Disability-Adjusted Life Year (DALY) the `effective-altruism-cost-effectiveness`
/// topic mentions as a sibling unit to the dollars-per-life-saved reasoning it does implement (see
/// [`crate::philanthropy_metrics::effective_altruism_cost_effectiveness`]), so it is implemented
/// here to complete that connection.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::years_of_life_lost;
///
/// // 40 deaths, each losing a standard 30 years of life expectancy at age of death.
/// let yll = years_of_life_lost(40, 30.0);
/// assert!((yll - 1_200.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn years_of_life_lost(deaths: u32, standard_life_expectancy_at_death: f64) -> f64 {
    f64::from(deaths) * standard_life_expectancy_at_death
}

/// Years Lived with Disability (YLD): incident cases times average condition duration times a
/// disability weight (0 = full health, 1 = a state equivalent to death).
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::years_lived_with_disability;
///
/// // 500 incident cases, averaging 4 years' duration, at a disability weight of 0.2.
/// let yld = years_lived_with_disability(500, 4.0, 0.2);
/// assert!((yld - 400.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn years_lived_with_disability(incident_cases: u32, average_duration_years: f64, disability_weight: f64) -> f64 {
    f64::from(incident_cases) * average_duration_years * disability_weight
}

/// Disability-Adjusted Life Years (DALYs): years of life lost to premature mortality plus years
/// lived with disability — the WHO Global Burden of Disease unit `effective-altruism-cost-effectiveness`
/// mentions GiveWell-style reasoning converts lives-saved figures into, for cross-cause comparison.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::{disability_adjusted_life_years, years_lived_with_disability, years_of_life_lost};
///
/// let yll = years_of_life_lost(40, 30.0);
/// let yld = years_lived_with_disability(500, 4.0, 0.2);
/// let daly = disability_adjusted_life_years(yll, yld);
/// assert!((daly - 1_600.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn disability_adjusted_life_years(years_of_life_lost: f64, years_lived_with_disability: f64) -> f64 {
    years_of_life_lost + years_lived_with_disability
}
