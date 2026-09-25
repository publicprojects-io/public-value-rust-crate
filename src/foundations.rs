//! Public value foundations: the eight concepts everything else in this crate builds on —
//! Mark Moore's strategic triangle, value for money, opportunity cost, the social discount rate,
//! distributional weighting, and the additionality/attribution/counterfactual trio that tests
//! whether a claimed impact is real.

use rust_decimal::Decimal;
use rust_decimal::prelude::MathematicalOps;

use crate::units::{Money, Percentage, Ratio};

/// Mark Moore's strategic triangle test: a public initiative is justified only when it is
/// legitimate and supported, substantively valuable, and operationally deliverable, all three at
/// once.
///
/// Ported from `public-value`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StrategicTriangle {
    /// Is the initiative authorized, and does the authorizing environment still back it?
    pub legitimacy_and_support: bool,
    /// Does it produce a specific, describable good for citizens or society?
    pub public_value: bool,
    /// Can the organization actually deliver it with current staff, technology, and authority?
    pub operational_capacity: bool,
}

impl StrategicTriangle {
    /// Proceeds only if all three legs hold — the source doc's explicit decision rule. A
    /// programme that is deliverable and popular but never actually authorized should not proceed
    /// as scoped, and neither should one that is authorized and deliverable but produces no
    /// describable public good.
    ///
    /// # Examples
    ///
    /// ```
    /// use public_value::foundations::StrategicTriangle;
    ///
    /// // Local authority AI triage tool: legitimacy and operational capacity both fail.
    /// let housing_triage = StrategicTriangle {
    ///     legitimacy_and_support: false,
    ///     public_value: true,
    ///     operational_capacity: false,
    /// };
    /// assert!(!housing_triage.passes());
    /// ```
    #[must_use]
    pub const fn passes(self) -> bool {
        self.legitimacy_and_support && self.public_value && self.operational_capacity
    }
}

/// One option in a value-for-money comparison: an upfront ("economy") cost plus an ongoing
/// per-case staff-time cost that determines efficiency.
///
/// Ported from `value-for-money`. The Green Book's three-E test (economy, efficiency,
/// effectiveness) is a sequential diagnostic, not a single ratio — this type represents the
/// economy and efficiency legs; effectiveness (whether outputs become outcomes) is programme-
/// specific and left to [`crate::impact_measurement`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValueForMoneyOption {
    /// The one-off ("economy") cost of choosing this option.
    pub upfront_cost: Money,
    /// Staff minutes spent per case under this option — the efficiency driver.
    pub minutes_per_case: Decimal,
    /// Fully loaded staff cost per hour.
    pub staff_cost_per_hour: Money,
}

impl ValueForMoneyOption {
    /// The annual staff-time ("efficiency") cost at a given case volume.
    ///
    /// # Panics
    ///
    /// Panics if a decimal conversion overflows (not reachable for realistic case volumes).
    ///
    /// # Examples
    ///
    /// ```
    /// use public_value::foundations::ValueForMoneyOption;
    /// use public_value::units::Money;
    /// use rust_decimal_macros::dec;
    ///
    /// let option_a = ValueForMoneyOption {
    ///     upfront_cost: Money::new(dec!(600_000)),
    ///     minutes_per_case: dec!(22),
    ///     staff_cost_per_hour: Money::new(dec!(28)),
    /// };
    /// let annual = option_a.annual_efficiency_cost(40_000);
    /// assert_eq!(annual.value().round_dp(0), dec!(410667));
    /// ```
    #[must_use]
    pub fn annual_efficiency_cost(&self, cases_per_year: u32) -> Money {
        let hours_per_case = self.minutes_per_case / Decimal::from(60);
        self.staff_cost_per_hour * hours_per_case * Decimal::from(cases_per_year)
    }

    /// Total cost after `years`: the one-off economy cost plus `years` of accumulated efficiency
    /// cost. Because economy and efficiency trade off (a cheaper licence can cost more overall
    /// once staff time is counted), the cheaper option by `upfront_cost` alone is not always
    /// cheaper by `total_cost` once enough years have passed.
    ///
    /// # Examples
    ///
    /// ```
    /// use public_value::foundations::ValueForMoneyOption;
    /// use public_value::units::Money;
    /// use rust_decimal_macros::dec;
    ///
    /// let option_a = ValueForMoneyOption {
    ///     upfront_cost: Money::new(dec!(600_000)),
    ///     minutes_per_case: dec!(22),
    ///     staff_cost_per_hour: Money::new(dec!(28)),
    /// };
    /// let option_b = ValueForMoneyOption {
    ///     upfront_cost: Money::new(dec!(900_000)),
    ///     minutes_per_case: dec!(9),
    ///     staff_cost_per_hour: Money::new(dec!(28)),
    /// };
    /// // Cheaper economy wins in year 1 ...
    /// assert!(option_a.total_cost(40_000, 1).value() < option_b.total_cost(40_000, 1).value());
    /// // ... but the efficiency gap swamps it by year 2.
    /// assert!(option_a.total_cost(40_000, 2).value() > option_b.total_cost(40_000, 2).value());
    /// ```
    #[must_use]
    pub fn total_cost(&self, cases_per_year: u32, years: u32) -> Money {
        self.upfront_cost + self.annual_efficiency_cost(cases_per_year) * Decimal::from(years)
    }
}

/// The net public value of choosing option A over the realistic next-best alternative B, not over
/// a "do nothing" baseline.
///
/// Ported from `opportunity-cost-in-public-spending`. There is no universal formula for
/// opportunity cost because the forgone alternative is context-specific — this function just
/// subtracts once the caller has identified and monetized that alternative.
///
/// # Examples
///
/// ```
/// use public_value::foundations::net_public_value;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // Digital-transformation fund: option A (£7.2m benefit) versus the realistic alternative B
/// // (£6.4m), not versus zero.
/// let net = net_public_value(Money::new(dec!(7_200_000)), Money::new(dec!(6_400_000)));
/// assert_eq!(net.value(), dec!(800_000));
/// ```
#[must_use]
pub fn net_public_value(value_of_chosen_option: Money, value_of_next_best_alternative: Money) -> Money {
    value_of_chosen_option - value_of_next_best_alternative
}

/// The compound discount factor `(1 + rate)^years` used to bring a future sum to present value.
///
/// Ported from `social-discount-rate`. Exposed separately from [`present_value`] so a caller can
/// compose the Green Book's *declining* schedule by multiplying factors for different rates over
/// different year ranges (the schedule is not a single constant rate; it steps down every few
/// decades), rather than this crate hard-coding the current published bands, which change between
/// Green Book editions.
///
/// # Panics
///
/// Panics if the exponentiation overflows `Decimal`'s range (not reachable for realistic discount
/// rates and horizons).
#[must_use]
pub fn discount_factor(annual_rate: Decimal, years: u32) -> Decimal {
    (Decimal::ONE + annual_rate).powi(i64::from(years))
}

/// The present value of a future sum: `FV / (1 + r)^t`.
///
/// Ported from `social-discount-rate`.
///
/// # Panics
///
/// Panics if `discount_factor` is zero (not reachable for a rate greater than `-100%`).
///
/// # Examples
///
/// ```
/// use public_value::foundations::present_value;
/// use public_value::units::Money;
/// use rust_decimal::prelude::ToPrimitive;
/// use rust_decimal_macros::dec;
///
/// // Flood defence: £10m of avoided damage in year 40, at a flat 3.5% discount rate (source doc's
/// // own worked example rounds this to "≈ £2.52 million").
/// let pv = present_value(Money::new(dec!(10_000_000)), dec!(0.035), 40);
/// assert!((pv.value().to_f64().unwrap() - 2_525_725.0).abs() < 1_000.0);
/// ```
#[must_use]
pub fn present_value(future_value: Money, annual_rate: Decimal, years: u32) -> Money {
    future_value / discount_factor(annual_rate, years)
}

/// The Green Book's distributional weight for a pound of benefit accruing to a household at
/// `household_income`, relative to `average_income`: `(average / household)^elasticity`.
///
/// Ported from `distributional-weighting`. The Green Book's own estimate for the elasticity of
/// marginal utility of income is approximately `1.3`; it is a parameter here, not a constant,
/// because the source doc explicitly warns against treating it as universal.
///
/// # Examples
///
/// ```
/// use public_value::foundations::distributional_weight;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // A household earning half the national average.
/// let weight = distributional_weight(Money::new(dec!(17_500)), Money::new(dec!(35_000)), 1.3);
/// assert!((weight.value() - 2.462).abs() < 0.01);
/// ```
#[must_use]
pub fn distributional_weight(household_income: Money, average_income: Money, elasticity: f64) -> Ratio {
    // Weight(y) = (ȳ / y)^e, i.e. average over household — not household over average — so that
    // below-average incomes get a weight greater than 1.
    let ratio = average_income.ratio_to(household_income);
    Ratio::new(ratio.value().powf(elasticity))
}

/// Applies a [`distributional_weight`] to an unweighted benefit.
///
/// # Panics
///
/// Panics if `weight` cannot be represented as a `Decimal` (not reachable for a finite, positive
/// weight).
///
/// # Examples
///
/// ```
/// use public_value::foundations::{distributional_weight, weighted_benefit};
/// use public_value::units::Money;
/// use rust_decimal::prelude::ToPrimitive;
/// use rust_decimal_macros::dec;
///
/// // Programme B: a skills programme in a deprived ward, average household income £18,000
/// // against a national average of £35,000 (the source doc's own worked example rounds the
/// // weight to "≈2.53"; precise exponentiation gives ≈2.37, used here).
/// let weight = distributional_weight(Money::new(dec!(18_000)), Money::new(dec!(35_000)), 1.3);
/// let weighted = weighted_benefit(Money::new(dec!(2_000_000)), weight);
/// assert!((weighted.value().to_f64().unwrap() - 4_747_492.0).abs() < 1_000.0);
/// ```
#[must_use]
pub fn weighted_benefit(unweighted_benefit: Money, weight: Ratio) -> Money {
    let weight_decimal = Decimal::from_f64_retain(weight.value()).expect("finite weight converts to Decimal");
    unweighted_benefit * weight_decimal
}

/// Net additional outcomes once the deadweight rate (what would have happened anyway) is
/// subtracted from the gross observed outcome: `gross × (1 − deadweight rate)`.
///
/// Ported from `additionality-and-deadweight`, the first and most important adjustment in the
/// standard UK evaluation net-impact sequence (gross − deadweight − displacement − leakage, ×
/// multiplier).
///
/// # Examples
///
/// ```
/// use public_value::foundations::net_additional_outcomes;
/// use public_value::units::Percentage;
///
/// // Business-support grant: 1,500 gross jobs claimed, 40% deadweight.
/// let net = net_additional_outcomes(1_500, Percentage::from_percent(40.0));
/// assert!((net - 900.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn net_additional_outcomes(gross_outcomes: u32, deadweight_rate: Percentage) -> f64 {
    f64::from(gross_outcomes) * (1.0 - deadweight_rate.as_fraction())
}

/// Net additional impact once displacement (activity or benefit diverted from elsewhere, not
/// created) is subtracted from a gross outcome: `gross − displaced`.
///
/// Ported from `displacement-and-attribution`. Call this once per geographic scale — local,
/// regional, national — passing the *cumulative* displacement observed at that scale, since an
/// intervention can be additional at a narrow scale while being pure displacement at a wider one.
///
/// # Examples
///
/// ```
/// use public_value::foundations::net_additional_impact;
///
/// // Regeneration grant: 200 gross jobs, 60 displaced locally, a further 30 displaced regionally.
/// let borough_level = net_additional_impact(200, 60);
/// let regional_level = net_additional_impact(200, 60 + 30);
/// assert_eq!(borough_level, 140);
/// assert_eq!(regional_level, 110);
/// ```
#[must_use]
pub const fn net_additional_impact(gross_outcome: u32, displacement: u32) -> u32 {
    gross_outcome.saturating_sub(displacement)
}

/// Whether a set of partner-attributed shares for a jointly delivered outcome sums to no more than
/// the observed total — the discipline `displacement-and-attribution` requires so that joint
/// delivery does not let every partner claim full credit for the same result.
///
/// # Examples
///
/// ```
/// use public_value::foundations::attribution_shares_valid;
///
/// // Three partners' contribution-analysis shares of a 30-person reduction in rough sleeping.
/// assert!(attribution_shares_valid(&[12.0, 10.5, 7.5], 30.0));
/// // Each partner independently claiming the full 30 sums to far more than the observed total.
/// assert!(!attribution_shares_valid(&[30.0, 30.0, 30.0], 30.0));
/// ```
#[must_use]
pub fn attribution_shares_valid(attributed_shares: &[f64], observed_total: f64) -> bool {
    attributed_shares.iter().sum::<f64>() <= observed_total + f64::EPSILON
}

/// A difference-in-differences estimate of a programme's true effect: the treated group's own
/// before/after change, minus the comparison group's before/after change over the same period.
///
/// Ported from `counterfactual-analysis`. Subtracting the comparison group's change removes any
/// trend common to both groups (e.g. a national economic recovery), isolating the effect
/// attributable to the intervention rather than confounding it with everything else that changed
/// at the same time.
///
/// # Examples
///
/// ```
/// use public_value::foundations::difference_in_differences;
///
/// // Employment programme: treated group rose 40% → 55%; comparison group rose 38% → 47% over
/// // the same period (a national recovery was underway).
/// let estimate = difference_in_differences(40.0, 55.0, 38.0, 47.0);
/// assert!((estimate - 6.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn difference_in_differences(
    treated_before: f64,
    treated_after: f64,
    comparison_before: f64,
    comparison_after: f64,
) -> f64 {
    (treated_after - treated_before) - (comparison_after - comparison_before)
}
