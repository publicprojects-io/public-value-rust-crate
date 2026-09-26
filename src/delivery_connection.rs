//! Software engineering and digital delivery connection: cost of delay, DORA metrics translated
//! into public value terms, flow metrics, technical debt as public value erosion, total cost of
//! ownership, build vs buy, benefits realization, and AI productivity in the public sector.

use rust_decimal::Decimal;

use crate::economic_appraisal::{annuity_factor, wellbys};
use crate::units::{CURRENCY_INVARIANT, Money, Percentage, Ratio, money_ratio};

/// The Cost of Delay per week: an annual benefit stream divided across the 52 weeks it is not yet
/// being delivered.
///
/// Ported from `cost-of-delay-in-public-programmes` — the "master bridge metric" that converts
/// schedule slippage into the same currency as the business case itself.
///
/// # Panics
///
/// Panics if the division overflows (not reachable for realistic values).
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::cost_of_delay_per_week;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // Housing-benefit system upgrade: £3,000,000/year in avoided overpayment error.
/// let cod = cost_of_delay_per_week(Money::from_decimal(dec!(3_000_000), iso::USD));
/// assert_eq!(cod.amount().round_dp(0), dec!(57_692));
/// ```
#[must_use]
pub fn cost_of_delay_per_week(annual_benefit: Money) -> Money {
    annual_benefit.div(Decimal::from(52)).expect("division by zero or overflow")
}

/// The total value lost to a delay: weekly Cost of Delay times the number of weeks slipped.
///
/// Ported from `cost-of-delay-in-public-programmes`.
///
/// # Panics
///
/// Panics if the multiplication overflows.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::{cost_of_delay_per_week, total_delay_loss};
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// let cod = cost_of_delay_per_week(Money::from_decimal(dec!(3_000_000), iso::USD));
/// let loss = total_delay_loss(cod, 52);
/// assert_eq!(loss.amount().round_dp(0), dec!(3_000_000));
/// ```
#[must_use]
pub fn total_delay_loss(cost_of_delay_per_week: Money, delay_weeks: u32) -> Money {
    cost_of_delay_per_week.mul(delay_weeks).expect("multiplication overflow")
}

/// A cycle time via Little's Law: work in progress divided by throughput.
///
/// Ported from `flow-metrics-in-government-delivery`. Works identically for a casework queue or a
/// pull-request queue — both are governed by the same law.
///
/// # Panics
///
/// Panics if `throughput_per_period` is zero.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::cycle_time;
///
/// // Planning department: 400 applications open, 50 resolved/week.
/// let weeks = cycle_time(400, 50);
/// assert!((weeks - 8.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn cycle_time(work_in_progress: u32, throughput_per_period: u32) -> f64 {
    f64::from(work_in_progress) / f64::from(throughput_per_period)
}

/// Flow efficiency: the fraction of total cycle time that is actual active (touch) time, per
/// Vacanti.
///
/// Ported from `flow-metrics-in-government-delivery`.
///
/// # Panics
///
/// Panics if `total_cycle_time_days` or `working_hours_per_day` is zero.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::flow_efficiency;
///
/// // 6 hours of active processing across an 8-week (56-day), 8-hour-day cycle.
/// let efficiency = flow_efficiency(6.0, 56.0, 8.0);
/// assert!((efficiency.as_percent() - 1.3).abs() < 0.05);
/// ```
#[must_use]
pub fn flow_efficiency(active_time_hours: f64, total_cycle_time_days: f64, working_hours_per_day: f64) -> Percentage {
    Percentage::from_fraction(active_time_hours / (total_cycle_time_days * working_hours_per_day))
}

/// The technical debt principal: lines of code times an estimated remediation cost per line.
///
/// Ported from `technical-debt-as-public-value-erosion`.
///
/// # Panics
///
/// Panics if the multiplication overflows.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::technical_debt_principal;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // 250,000-line legacy claims engine at the CAST Appmarq benchmark (≈£2.85/line).
/// let principal = technical_debt_principal(250_000, Money::from_decimal(dec!(2.85), iso::USD));
/// assert_eq!(*principal.amount(), dec!(712_500.00));
/// ```
#[must_use]
pub fn technical_debt_principal(lines_of_code: u32, remediation_cost_per_line: Money) -> Money {
    remediation_cost_per_line.mul(lines_of_code).expect("multiplication overflow")
}

/// The technical debt ratio: remediation cost as a fraction of full redevelopment cost
/// (`SonarQube` grades: A ≤5%, B ≤10%, C ≤20%, D ≤50%).
///
/// Ported from `technical-debt-as-public-value-erosion`.
///
/// # Panics
///
/// Panics if `redevelopment_cost` is zero.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::technical_debt_ratio;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// let tdr = technical_debt_ratio(Money::from_decimal(dec!(712_500), iso::USD), Money::from_decimal(dec!(4_453_125), iso::USD));
/// assert!((tdr.as_percent() - 16.0).abs() < 0.01); // SonarQube grade C
/// ```
#[must_use]
pub fn technical_debt_ratio(remediation_cost: Money, redevelopment_cost: Money) -> Percentage {
    Percentage::from_fraction(money_ratio(remediation_cost, redevelopment_cost).value())
}

/// The annual interest reduction from a targeted remediation: current interest times the modelled
/// reduction fraction.
///
/// Ported from `technical-debt-as-public-value-erosion`. Principal states the liability; interest
/// is what justifies paying it down to a spending approver.
///
/// # Panics
///
/// Panics if `reduction_fraction` cannot be represented as a `Decimal` (not reachable for a finite
/// value), or if the multiplication overflows.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::{interest_reduction, payback_period_years};
/// use public_value::units::{Money, Percentage, iso};
/// use rust_decimal_macros::dec;
///
/// // £680,000/year interest (skills premium + redirected-contact cost), 70% cut, £1.2m remediation.
/// let reduction = interest_reduction(Money::from_decimal(dec!(680_000), iso::USD), Percentage::from_percent(70.0));
/// assert_eq!(reduction.amount().round_dp(2), dec!(476_000.00));
///
/// let payback = payback_period_years(Money::from_decimal(dec!(1_200_000), iso::USD), reduction);
/// assert!((payback - 2.52).abs() < 0.01);
/// ```
#[must_use]
pub fn interest_reduction(current_annual_interest: Money, reduction_fraction: Percentage) -> Money {
    let fraction = Decimal::from_f64_retain(reduction_fraction.as_fraction()).expect("finite fraction converts to Decimal");
    current_annual_interest.mul(fraction).expect("multiplication overflow")
}

/// The payback period, in years, of a remediation cost against its annual interest reduction.
///
/// Ported from `technical-debt-as-public-value-erosion`.
///
/// # Panics
///
/// Panics if `annual_interest_reduction` is zero.
#[must_use]
pub fn payback_period_years(remediation_cost: Money, annual_interest_reduction: Money) -> f64 {
    money_ratio(remediation_cost, annual_interest_reduction).value()
}

/// Total cost of ownership: acquisition cost plus the discounted sum of annual operating costs
/// over the appraisal horizon, via [`crate::economic_appraisal::annuity_factor`].
///
/// Ported from `total-cost-of-ownership-in-government-it`.
///
/// # Panics
///
/// Panics if `annual_rate` is zero, if a multiplication overflows, or if the two amounts are not
/// in the same currency (not reachable within this crate).
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::total_cost_of_ownership;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // System A: £3.5m capex, £250,000/year opex, 5 years at 3.5%.
/// let tco_a = total_cost_of_ownership(Money::from_decimal(dec!(3_500_000), iso::USD), Money::from_decimal(dec!(250_000), iso::USD), dec!(0.035), 5);
/// assert!((tco_a.amount() - dec!(4_628_763)).abs() < dec!(1_000));
/// ```
#[must_use]
pub fn total_cost_of_ownership(acquisition_cost: Money, annual_operating_cost: Money, annual_rate: Decimal, years: u32) -> Money {
    let operating_cost_pv = annual_operating_cost.mul(annuity_factor(annual_rate, years)).expect("multiplication overflow");
    acquisition_cost.add(operating_cost_pv).expect(CURRENCY_INVARIANT)
}

/// A risk-adjusted build cost: the raw estimate scaled by an optimism-bias uplift multiplier (the
/// Green Book's Mott MacDonald-derived IT project uplift range is roughly 1.1–3.0).
///
/// Ported from `build-vs-buy-in-government`.
///
/// # Panics
///
/// Panics if the multiplication overflows.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::risk_adjusted_build_cost;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// let adjusted = risk_adjusted_build_cost(Money::from_decimal(dec!(900_000), iso::USD), dec!(1.4));
/// assert_eq!(*adjusted.amount(), dec!(1_260_000.0));
/// ```
#[must_use]
pub fn risk_adjusted_build_cost(build_cost_estimate: Money, optimism_bias_multiplier: Decimal) -> Money {
    build_cost_estimate.mul(optimism_bias_multiplier).expect("multiplication overflow")
}

/// The total subscription cost of a "buy" option over `years`.
///
/// Ported from `build-vs-buy-in-government`.
///
/// # Panics
///
/// Panics if the multiplication overflows.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::buy_total_cost;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// let cost = buy_total_cost(Money::from_decimal(dec!(180_000), iso::USD), 5);
/// assert_eq!(*cost.amount(), dec!(900_000));
/// ```
#[must_use]
pub fn buy_total_cost(annual_subscription: Money, years: u32) -> Money {
    annual_subscription.mul(years).expect("multiplication overflow")
}

/// A benefits realization rate: benefits actually realized divided by the benefits forecast at
/// approval, per benefit line.
///
/// Ported from `benefits-realization`. A rate below 100% is not automatically a failure — it is
/// evidence that recalibrates the next forecast, provided a baseline was captured before go-live.
///
/// # Panics
///
/// Panics if `benefits_forecast` is zero.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::realization_rate;
///
/// // Digital planning-application portal: cash savings forecast £300,000, realized £210,000.
/// let rate = realization_rate(210_000.0, 300_000.0);
/// assert!((rate.as_percent() - 70.0).abs() < 0.01);
/// ```
#[must_use]
pub fn realization_rate(benefits_realized: f64, benefits_forecast: f64) -> Percentage {
    Percentage::from_fraction(benefits_realized / benefits_forecast)
}

/// The number of developers eligible to use an AI tool, given the share of the estate cleared for
/// its data classification.
///
/// Ported from `ai-productivity-in-the-public-sector`. The eligible-coverage factor has no
/// private-sector equivalent: security classification, not licensing, is often the binding
/// constraint on realized value.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::eligible_developers;
/// use public_value::units::Percentage;
///
/// let eligible = eligible_developers(300, Percentage::from_percent(70.0));
/// assert_eq!(eligible, 210);
/// ```
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
// A developer headcount is always small and non-negative, so the f64→u32 cast below cannot
// meaningfully lose precision or go negative.
pub fn eligible_developers(total_developers: u32, eligible_share: Percentage) -> u32 {
    (f64::from(total_developers) * eligible_share.as_fraction()).round() as u32
}

/// The value of measured (not self-reported) AI-assisted capacity freed: eligible developers times
/// hours saved per day, working days per year, loaded hourly rate, and utilization.
///
/// Ported from `ai-productivity-in-the-public-sector`. Uses the *measured* task-level saving, not
/// the self-reported figure — the source doc's central methodological point, following the METR
/// study's finding of a large perception-versus-measured gap.
///
/// # Panics
///
/// Panics if the computed hours figure cannot be represented as a `Decimal` (not reachable for
/// finite, realistic inputs), or if the multiplication overflows.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::ai_capacity_value;
/// use public_value::units::{Money, Percentage, iso};
/// use rust_decimal_macros::dec;
///
/// let value = ai_capacity_value(210, 0.2, 220, Money::from_decimal(dec!(55), iso::USD), Percentage::from_percent(60.0));
/// assert_eq!(value.amount().round_dp(0), dec!(304_920));
/// ```
#[must_use]
pub fn ai_capacity_value(eligible_developers: u32, hours_saved_per_day: f64, working_days_per_year: u32, loaded_hourly_rate: Money, utilization: Percentage) -> Money {
    let total_hours = f64::from(eligible_developers) * hours_saved_per_day * f64::from(working_days_per_year) * utilization.as_fraction();
    let hours_decimal = Decimal::from_f64_retain(total_hours).expect("finite hours total converts to Decimal");
    loaded_hourly_rate.mul(hours_decimal).expect("multiplication overflow")
}

/// The net capacity ratio: measured AI capacity value divided by licensing cost.
///
/// Ported from `ai-productivity-in-the-public-sector`.
///
/// # Panics
///
/// Panics if `licensing_cost` is zero.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::net_capacity_ratio;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// let ratio = net_capacity_ratio(Money::from_decimal(dec!(304_920), iso::USD), Money::from_decimal(dec!(55_440), iso::USD));
/// assert!((ratio.value() - 5.5).abs() < 0.1);
/// ```
#[must_use]
pub fn net_capacity_ratio(capacity_value: Money, licensing_cost: Money) -> Ratio {
    money_ratio(capacity_value, licensing_cost)
}

/// The value pulled forward by a lead-time reduction: improvements shipped per year times the
/// weeks of lead time cut times the weekly Cost of Delay each improvement carries.
///
/// Ported from `dora-metrics-for-public-value`, which translates the DORA delivery metrics into
/// public-value terms — lead time maps to [`cost_of_delay_per_week`] the way it maps to money for
/// any other public programme.
///
/// # Panics
///
/// Panics if `lead_time_reduction_weeks` cannot be represented as a `Decimal` (not reachable for a
/// finite value), or if a multiplication overflows.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::lead_time_value_pulled_forward;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // Benefits-claims portal: 25 improvements/year, lead time cut from 8 weeks to 5 days
/// // (≈7.3 weeks saved), £8,000/week Cost of Delay each.
/// let weeks_saved = 8.0 - 5.0 / 7.0;
/// let value = lead_time_value_pulled_forward(25, weeks_saved, Money::from_decimal(dec!(8_000), iso::USD));
/// assert!((value.amount() - dec!(1_457_143)).abs() < dec!(1_000));
/// ```
#[must_use]
pub fn lead_time_value_pulled_forward(improvements_per_year: u32, lead_time_reduction_weeks: f64, weekly_cost_of_delay: Money) -> Money {
    let weeks = Decimal::from_f64_retain(lead_time_reduction_weeks).expect("finite weeks converts to Decimal");
    weekly_cost_of_delay.mul(weeks).expect("multiplication overflow").mul(improvements_per_year).expect("multiplication overflow")
}

/// The number of fewer failed changes per year from a change-failure-rate improvement: changes
/// shipped times the drop in failure rate.
///
/// Ported from `dora-metrics-for-public-value`.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::fewer_failed_changes;
/// use public_value::units::Percentage;
///
/// // 25 changes/year, failure rate cut from 30% to 10%.
/// let fewer = fewer_failed_changes(25, Percentage::from_percent(30.0), Percentage::from_percent(10.0));
/// assert!((fewer - 5.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn fewer_failed_changes(changes_per_year: u32, failure_rate_before: Percentage, failure_rate_after: Percentage) -> f64 {
    f64::from(changes_per_year) * (failure_rate_before.as_fraction() - failure_rate_after.as_fraction())
}

/// The saving from avoiding failed changes: fewer failures times the cost each one causes (e.g.
/// citizens redirected to a more expensive contact-centre channel).
///
/// Ported from `dora-metrics-for-public-value`.
///
/// # Panics
///
/// Panics if `fewer_failures` cannot be represented as a `Decimal` (not reachable for a finite
/// value), or if the multiplication overflows.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::{fewer_failed_changes, incident_avoidance_saving};
/// use public_value::units::{Money, Percentage, iso};
/// use rust_decimal_macros::dec;
///
/// let fewer = fewer_failed_changes(25, Percentage::from_percent(30.0), Percentage::from_percent(10.0));
/// let saving = incident_avoidance_saving(fewer, Money::from_decimal(dec!(16_840), iso::USD));
/// assert_eq!(*saving.amount(), dec!(84_200));
/// ```
#[must_use]
pub fn incident_avoidance_saving(fewer_failures: f64, cost_per_incident: Money) -> Money {
    let count = Decimal::from_f64_retain(fewer_failures).expect("finite failure count converts to Decimal");
    cost_per_incident.mul(count).expect("multiplication overflow")
}

/// The aggregate WELLBY loss from a delay, reusing [`crate::economic_appraisal::wellbys`] with a
/// negative life-satisfaction change — the wellbeing-denominated form of [`cost_of_delay_per_week`]
/// for citizen-facing services.
///
/// Ported from `cost-of-delay-in-public-programmes`.
///
/// # Examples
///
/// ```
/// use public_value::delivery_connection::delay_wellby_loss;
///
/// // Disability-benefit assessment: 200,000 claimants, 3 extra weeks at −0.0018 WELLBY/week each.
/// let loss = delay_wellby_loss(200_000, 3.0, 0.0018);
/// assert!((loss - 1_080.0).abs() < 1e-6);
/// ```
#[must_use]
pub fn delay_wellby_loss(claimants_affected: u32, extra_weeks: f64, wellby_loss_per_week: f64) -> f64 {
    wellbys(claimants_affected, extra_weeks * wellby_loss_per_week, 1.0)
}
