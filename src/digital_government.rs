//! Public value in digital government: the digital service standard, cost per transaction, channel
//! shift savings, digital inclusion, government as a platform, open data value, public sector
//! cybersecurity value, and AI in government value.

use rust_decimal::Decimal;

use crate::units::{Money, Percentage};

/// One service's assessment against the 14-point GOV.UK Service Standard.
///
/// Ported from `digital-service-standard`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServiceAssessment {
    /// How many of the 14 standard points were assessed as met.
    pub points_met: u32,
    /// The total number of points in the standard (14, as of the current edition).
    pub total_points: u32,
}

impl ServiceAssessment {
    /// True only if every point was met — a service cannot go live, or move phase, on a partial
    /// pass.
    #[must_use]
    pub const fn passes(self) -> bool {
        self.points_met >= self.total_points
    }
}

/// A future annual value, pro-rated for a delay of `delay_months` out of a 12-month year.
///
/// Ported from `digital-service-standard`, used there to price the delay cost of a failed
/// assessment against the channel-shift saving it postponed.
///
/// # Panics
///
/// Panics if the conversion of `delay_months` to `Decimal` overflows (not reachable for realistic
/// delays).
///
/// # Examples
///
/// ```
/// use public_value::digital_government::prorate_annual_value;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// let annual_forgone = Money::new(dec!(59_760));
/// let delay_cost = prorate_annual_value(annual_forgone, 2);
/// assert_eq!(delay_cost.value().round_dp(0), dec!(9_960));
/// ```
#[must_use]
pub fn prorate_annual_value(annual_value: Money, delay_months: u32) -> Money {
    annual_value * Decimal::from(delay_months) / Decimal::from(12)
}

/// The total cost of a failed service assessment: remediation, redesign, and the pro-rated delay
/// to the channel-shift saving the service was funded to deliver.
///
/// Ported from `digital-service-standard`.
///
/// # Examples
///
/// ```
/// use public_value::digital_government::failed_assessment_cost;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // Housing application service: remediation sprint, assisted-digital design, and delay cost.
/// let total = failed_assessment_cost(Money::new(dec!(34_650)), Money::new(dec!(5_000)), Money::new(dec!(9_960)));
/// assert_eq!(total.value(), dec!(49_610));
/// ```
#[must_use]
pub fn failed_assessment_cost(remediation_cost: Money, redesign_cost: Money, prorated_delay_cost: Money) -> Money {
    remediation_cost + redesign_cost + prorated_delay_cost
}

/// The gross (naive) channel-shift saving: volume shifted times the per-transaction cost
/// difference between the old and new channel, before netting off shadow demand, failure-demand
/// leakage, and unretired fixed capacity.
///
/// Ported from `channel-shift-savings`. See [`crate::performance_metrics::cost_per_transaction`]
/// for the per-channel unit cost this feeds from, and [`fte_released`] plus
/// [`crate::performance_metrics::failure_demand_cost`] for the corrections toward a realized
/// saving.
///
/// # Examples
///
/// ```
/// use public_value::digital_government::gross_channel_shift_saving;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // Blue badge renewal: 39,000 shifted, £6.40 phone/paper versus £0.30 digital.
/// let gross = gross_channel_shift_saving(39_000, Money::new(dec!(6.40)), Money::new(dec!(0.30)));
/// assert_eq!(gross.value(), dec!(237_900.00));
/// ```
#[must_use]
pub fn gross_channel_shift_saving(shifted_volume: u32, old_channel_cost: Money, new_channel_cost: Money) -> Money {
    (old_channel_cost - new_channel_cost) * shifted_volume
}

/// The whole-time-equivalent staff released by a call-volume reduction, staffed in discrete bands —
/// contact centres cannot shed a fraction of a post, so this rounds down.
///
/// Ported from `channel-shift-savings`.
///
/// # Panics
///
/// Panics if `calls_per_fte_band` is zero.
///
/// # Examples
///
/// ```
/// use public_value::digital_government::fte_released;
///
/// // 14,000-call drop, staffed in bands of 8,000 calls/FTE: 1.75 FTE, rounded down to 1.
/// assert_eq!(fte_released(14_000, 8_000), 1);
/// ```
#[must_use]
pub const fn fte_released(call_volume_reduction: u32, calls_per_fte_band: u32) -> u32 {
    call_volume_reduction / calls_per_fte_band
}

/// The blended cost per transaction across a self-service digital cohort and an assisted-digital
/// cohort, weighted by cohort size.
///
/// Ported from `digital-inclusion`. Skipping assisted digital to report a lower headline blended
/// figure does not eliminate its cost — it converts it into unclaimed entitlements and downstream
/// demand on a different budget, per the source doc's central warning.
///
/// # Panics
///
/// Panics if both cohorts are empty.
///
/// # Examples
///
/// ```
/// use public_value::digital_government::blended_cost_per_transaction;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // Universal-Credit-style service: 250,000 assisted-digital claims at £9.50, 2,250,000
/// // self-service claims at £0.40.
/// let blended = blended_cost_per_transaction(250_000, Money::new(dec!(9.50)), 2_250_000, Money::new(dec!(0.40)));
/// assert_eq!(blended.value().round_dp(2), dec!(1.31));
/// ```
#[must_use]
pub fn blended_cost_per_transaction(
    assisted_digital_cohort: u32,
    assisted_digital_cost: Money,
    self_service_cohort: u32,
    self_service_cost: Money,
) -> Money {
    let total_cost = assisted_digital_cost * assisted_digital_cohort + self_service_cost * self_service_cohort;
    total_cost / (assisted_digital_cohort + self_service_cohort)
}

/// The first-year saving from adopting a shared platform component instead of building an
/// equivalent bespoke service: the build-your-own estimate minus the platform integration cost.
///
/// Ported from `government-as-a-platform`.
///
/// # Examples
///
/// ```
/// use public_value::digital_government::platform_adoption_saving;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // A local authority adopting GOV.UK Pay instead of building a payment gateway.
/// let saving = platform_adoption_saving(Money::new(dec!(85_000)), Money::new(dec!(12_000)));
/// assert_eq!(saving.value(), dec!(73_000));
/// ```
#[must_use]
pub fn platform_adoption_saving(build_your_own_cost: Money, platform_integration_cost: Money) -> Money {
    build_your_own_cost - platform_integration_cost
}

/// The cost-avoided lower-bound estimate of open data value: what adopting organizations would
/// otherwise have paid to licence equivalent data commercially.
///
/// Ported from `open-data-value`. A credible business case reports this as the solid lower bound
/// and treats any downstream-activity estimate as an upper-bound scenario, not a fact.
///
/// # Examples
///
/// ```
/// use public_value::digital_government::cost_avoided_open_data_value;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // 15,000 SMEs using a free open address-matching dataset, avoiding a £4,000/year licence each.
/// let value = cost_avoided_open_data_value(15_000, Money::new(dec!(4_000)));
/// assert_eq!(value.value(), dec!(60_000_000));
/// ```
#[must_use]
pub fn cost_avoided_open_data_value(organizations_avoiding_a_licence: u32, avoided_annual_licence_cost: Money) -> Money {
    avoided_annual_licence_cost * organizations_avoiding_a_licence
}

/// Annualized Loss Expectancy (ALE): the single loss expectancy of a breach, times its annualized
/// rate of occurrence.
///
/// Ported from `public-sector-cybersecurity-value`.
///
/// # Panics
///
/// Panics if `annual_rate_of_occurrence` cannot be represented as a `Decimal` (not reachable for a
/// finite value).
///
/// # Examples
///
/// ```
/// use public_value::digital_government::annualized_loss_expectancy;
/// use public_value::units::{Money, Percentage};
/// use rust_decimal_macros::dec;
///
/// // £2.1m single loss expectancy at an estimated 8%/year rate of occurrence.
/// let ale = annualized_loss_expectancy(Money::new(dec!(2_100_000)), Percentage::from_percent(8.0));
/// assert_eq!(ale.value().round_dp(2), dec!(168_000.00));
/// ```
#[must_use]
pub fn annualized_loss_expectancy(single_loss_expectancy: Money, annual_rate_of_occurrence: Percentage) -> Money {
    let rate = Decimal::from_f64_retain(annual_rate_of_occurrence.as_fraction()).expect("finite rate converts to Decimal");
    single_loss_expectancy * rate
}

/// The value of a proposed security control: the ALE it removes, minus its own annual cost. A
/// control is worth funding when this is positive.
///
/// Ported from `public-sector-cybersecurity-value`.
///
/// # Examples
///
/// ```
/// use public_value::digital_government::security_control_value;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// let value = security_control_value(Money::new(dec!(168_000)), Money::new(dec!(63_000)), Money::new(dec!(45_000)));
/// assert_eq!(value.value(), dec!(60_000));
/// ```
#[must_use]
pub fn security_control_value(ale_before_control: Money, ale_after_control: Money, annual_control_cost: Money) -> Money {
    (ale_before_control - ale_after_control) - annual_control_cost
}

/// The staff-time cost of handling a volume of enquiries at a given average handling time and
/// loaded hourly cost — used both for the no-AI baseline and for the fully-loaded review-and-correct
/// cost once an AI drafting tool is introduced.
///
/// Ported from `ai-in-government-value`.
///
/// # Panics
///
/// Panics if the conversion of `minutes_per_enquiry` overflows (not reachable for realistic
/// handling times).
///
/// # Examples
///
/// ```
/// use public_value::digital_government::handling_time_cost;
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// // Baseline: 25,000 council tax enquiries/year at 14 minutes each, £34/hour loaded cost.
/// let baseline = handling_time_cost(25_000, dec!(14), Money::new(dec!(34)));
/// assert_eq!(baseline.value().round_dp(2), dec!(198_333.33));
/// ```
#[must_use]
pub fn handling_time_cost(enquiries: u32, minutes_per_enquiry: Decimal, loaded_hourly_cost: Money) -> Money {
    loaded_hourly_cost * (minutes_per_enquiry / Decimal::from(60)) * enquiries
}

/// The real net value of an AI system once productivity gain is measured against its full,
/// production cost — licence/compute, human verification and oversight, and documentation — rather
/// than a pilot's best-case oversight time.
///
/// Ported from `ai-in-government-value`.
///
/// # Examples
///
/// ```
/// use public_value::digital_government::{handling_time_cost, net_ai_value};
/// use public_value::units::Money;
/// use rust_decimal_macros::dec;
///
/// let baseline_cost = handling_time_cost(25_000, dec!(14), Money::new(dec!(34)));
/// let production_review_cost = handling_time_cost(25_000, dec!(6), Money::new(dec!(34)));
/// let net = net_ai_value(baseline_cost, production_review_cost, Money::new(dec!(38_000)), Money::new(dec!(14_000)));
/// assert_eq!(net.value().round_dp(0), dec!(61_333));
/// ```
#[must_use]
pub fn net_ai_value(baseline_cost: Money, production_review_cost: Money, licence_and_compute_cost: Money, documentation_cost: Money) -> Money {
    baseline_cost - (production_review_cost + licence_and_compute_cost + documentation_cost)
}
