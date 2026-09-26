//! Social sector and philanthropy metrics: cost per outcome, cost per beneficiary, blended value,
//! effective-altruism-style cost-effectiveness, charity overhead ratio, donor return on investment,
//! grant outcomes reporting, and volunteer time value.

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::units::{Money, Percentage, money_ratio};

/// Cost per outcome: total programme cost divided by the number of beneficiaries who achieved the
/// defined outcome (not merely received a service).
///
/// Ported from `cost-per-outcome`. Contrast with [`cost_per_beneficiary`], which divides by
/// everyone served rather than everyone who achieved the outcome; cost per outcome is always the
/// larger (or equal) figure because the outcome population is a subset of the beneficiary
/// population.
///
/// # Panics
///
/// Panics if `beneficiaries_achieving_outcome` is zero.
///
/// # Examples
///
/// ```
/// use public_value::philanthropy_metrics::cost_per_outcome;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// let cost = cost_per_outcome(Money::from_decimal(dec!(450_000), iso::USD), 630);
/// assert_eq!(cost.amount().round_dp(2), dec!(714.29));
/// ```
#[must_use]
pub fn cost_per_outcome(total_programme_cost: Money, beneficiaries_achieving_outcome: u32) -> Money {
    total_programme_cost.div(beneficiaries_achieving_outcome).expect("division by zero or overflow")
}

/// Cost per beneficiary: total programme cost divided by the number of unique people served,
/// regardless of whether their circumstances changed.
///
/// Ported from `cost-per-beneficiary`. This is a reach metric, not an impact metric — see
/// [`cost_per_outcome`] for the number that engages with whether the money worked.
///
/// # Panics
///
/// Panics if `unique_people_served` is zero.
///
/// # Examples
///
/// ```
/// use public_value::philanthropy_metrics::cost_per_beneficiary;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// let cost = cost_per_beneficiary(Money::from_decimal(dec!(450_000), iso::USD), 1_800);
/// assert_eq!(*cost.amount(), dec!(250));
/// ```
#[must_use]
pub fn cost_per_beneficiary(total_programme_cost: Money, unique_people_served: u32) -> Money {
    total_programme_cost.div(unique_people_served).expect("division by zero or overflow")
}

/// Jed Emerson's blended value framework: every unit of capital deployed produces an economic, a
/// social, and an environmental effect at once, reported side by side rather than netted into one
/// number.
///
/// Ported from `blended-value`. Deliberately has no method that sums the three lines: Emerson's
/// own writing warns that summing hides which line is doing the work and invites cherry-picking, so
/// this type refuses to offer a single "blended score."
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlendedValue {
    /// The annual financial effect (e.g. interest earned, cost saved, revenue generated).
    pub economic_annual: Money,
    /// A named description of the social effect and how it would be evidenced (wellbeing,
    /// capability, or equity change for people).
    pub social_line: String,
    /// A named description of the environmental effect (natural capital protected, degraded, or
    /// restored).
    pub environmental_line: String,
}

impl BlendedValue {
    /// Creates a new blended value record for one investment or grant.
    #[must_use]
    pub const fn new(economic_annual: Money, social_line: String, environmental_line: String) -> Self {
        Self {
            economic_annual,
            social_line,
            environmental_line,
        }
    }
}

/// GiveWell-style cost-effectiveness: the cost of an intervention divided by the units of good it
/// produces (e.g. dollars per life saved, dollars per DALY averted).
///
/// Ported from `effective-altruism-cost-effectiveness`. Returns a price *per unit of good*, not a
/// dimensionless ratio, because the two sides of the division have different units.
///
/// # Panics
///
/// Panics if `units_of_good` is zero.
///
/// # Examples
///
/// ```
/// use public_value::philanthropy_metrics::effective_altruism_cost_effectiveness;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // GiveWell's published Against Malaria Foundation worked example: roughly $4,500 per life saved.
/// let cost = effective_altruism_cost_effectiveness(Money::from_decimal(dec!(4_500), iso::USD), dec!(1));
/// assert_eq!(*cost.amount(), dec!(4_500));
/// ```
#[must_use]
pub fn effective_altruism_cost_effectiveness(intervention_cost: Money, units_of_good: Decimal) -> Money {
    intervention_cost.div(units_of_good).expect("division by zero or overflow")
}

/// Units of good produced by a budget at a given cost per unit — the inverse of
/// [`effective_altruism_cost_effectiveness`], useful for turning a headline "$X per life saved"
/// figure into "how many lives does my budget buy."
///
/// # Panics
///
/// Panics if `cost_per_unit` is zero, or if the resulting count cannot be represented as an `f64`.
///
/// # Examples
///
/// ```
/// use public_value::philanthropy_metrics::units_produced;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // roughly 20 lives saved per £100,000 at $4,500/life, per GiveWell's worked example.
/// let lives = units_produced(Money::from_decimal(dec!(100_000), iso::USD), Money::from_decimal(dec!(4_500), iso::USD));
/// assert!((lives - 22.222_222).abs() < 0.001);
/// ```
#[must_use]
pub fn units_produced(budget: Money, cost_per_unit: Money) -> f64 {
    money_ratio(budget, cost_per_unit).value()
}

/// The charity overhead ratio: administrative and fundraising expenditure as a fraction of total
/// expenditure.
///
/// Ported from `charity-overhead-ratio`. This formula contains no information about outcomes
/// achieved — see [`cost_per_outcome`] for the metric that engages with whether the money worked.
/// The organizations that popularized overhead ratio (`GuideStar`, BBB Wise Giving Alliance,
/// Charity Navigator) jointly disowned it as an efficiency measure in their 2013 "Overhead Myth"
/// letter.
///
/// # Panics
///
/// Panics if `total_expenditure` is zero.
///
/// # Examples
///
/// ```
/// use public_value::philanthropy_metrics::charity_overhead_ratio;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// let ratio = charity_overhead_ratio(Money::from_decimal(dec!(220_000), iso::USD), Money::from_decimal(dec!(1_000_000), iso::USD));
/// assert!((ratio.as_percent() - 22.0).abs() < 0.001);
/// ```
#[must_use]
pub fn charity_overhead_ratio(administrative_and_fundraising_cost: Money, total_expenditure: Money) -> Percentage {
    let ratio = money_ratio(administrative_and_fundraising_cost, total_expenditure);
    Percentage::from_fraction(ratio.value())
}

/// The programme ratio: the complement of [`charity_overhead_ratio`], i.e. `1 − overhead ratio`.
///
/// # Examples
///
/// ```
/// use public_value::philanthropy_metrics::programme_ratio;
/// use public_value::units::Percentage;
///
/// let programme = programme_ratio(Percentage::from_percent(22.0));
/// assert!((programme.as_percent() - 78.0).abs() < 0.001);
/// ```
#[must_use]
pub fn programme_ratio(overhead_ratio: Percentage) -> Percentage {
    Percentage::from_fraction(1.0 - overhead_ratio.as_fraction())
}

/// Donor return on investment: the outcome a gift achieves beyond what would have happened anyway
/// (the counterfactual), per unit of the gift.
///
/// Ported from `donor-return-on-investment`. Unlike [`charity_overhead_ratio`] or
/// [`cost_per_beneficiary`], this is deliberately from the donor's perspective: a well-run charity
/// with no funding gap can still have a near-zero donor ROI, because the marginal gift changes
/// nothing that would not have happened anyway.
///
/// Returns additional outcome achieved per unit of currency in the gift (or `NaN`/infinite if
/// `gift` is zero — division by zero here is a data problem, not a programmer error, so this
/// returns a non-finite float rather than panicking); the caller supplies `outcome_with_gift` and
/// `counterfactual_outcome` in whatever outcome unit the programme uses (people served, DALYs,
/// etc.), since donor ROI is not itself a money-to-money ratio.
///
/// # Examples
///
/// ```
/// use public_value::philanthropy_metrics::donor_return_on_investment;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // Charity D: a funding-constrained programme where the marginal £5,000 buys 20 additional
/// // people served (at the charity's own £250 cost per beneficiary) that would not otherwise be
/// // served.
/// let roi = donor_return_on_investment(20.0, 0.0, Money::from_decimal(dec!(5_000), iso::USD));
/// assert!((roi - 0.004).abs() < 1e-6);
///
/// // Charity C: fully funded already, so the marginal gift is not additional.
/// let roi_c = donor_return_on_investment(0.0, 0.0, Money::from_decimal(dec!(5_000), iso::USD));
/// assert!((roi_c).abs() < 1e-9);
/// assert!(roi > roi_c);
/// ```
#[must_use]
pub fn donor_return_on_investment(outcome_with_gift: f64, counterfactual_outcome: f64, gift: Money) -> f64 {
    let additional_outcome = outcome_with_gift - counterfactual_outcome;
    // The numerator is an outcome count, not a monetary amount, so this divides by the gift's raw
    // decimal value converted to `f64` rather than via `units::money_ratio` (which divides two
    // `Money` values into a `Ratio`).
    let gift_value = gift.amount().to_f64().unwrap_or(f64::NAN);
    additional_outcome / gift_value
}

/// The number of bespoke funder-grantee reporting relationships without a shared metrics standard:
/// every funder asking every grantee for its own definitions.
///
/// Ported from `grant-outcomes-reporting` (IRIS+). See [`standardized_reporting_mappings`] for the
/// number of mappings needed once funders and grantees share one standard instead.
///
/// # Examples
///
/// ```
/// use public_value::philanthropy_metrics::bespoke_reporting_relationships;
///
/// assert_eq!(bespoke_reporting_relationships(10, 20), 200);
/// ```
#[must_use]
pub fn bespoke_reporting_relationships(funders: u32, grantees: u32) -> u32 {
    funders * grantees
}

/// The number of metric-definition mappings needed once funders and grantees share one standard
/// (e.g. IRIS+): each grantee maps once to the standard, and each funder maps once to the standard,
/// rather than every pair mapping to each other.
///
/// # Examples
///
/// ```
/// use public_value::philanthropy_metrics::{bespoke_reporting_relationships, standardized_reporting_mappings};
///
/// let (funders, grantees) = (10, 20);
/// assert_eq!(standardized_reporting_mappings(funders, grantees), 30);
/// assert!(standardized_reporting_mappings(funders, grantees) < bespoke_reporting_relationships(funders, grantees));
/// ```
#[must_use]
pub fn standardized_reporting_mappings(funders: u32, grantees: u32) -> u32 {
    funders + grantees
}

/// The monetary value of unpaid volunteer labour: hours contributed multiplied by an hourly rate.
///
/// Ported from `volunteer-time-value`. The rate is deliberately a parameter rather than a built-in
/// constant: the source topic documents three defensible rate choices (replacement cost, the
/// volunteer's own opportunity cost, and a national-average rate such as the UK ONS's £14.43/hour
/// or the US Independent Sector's $36.14/hour) that can differ by a large multiple for the same
/// hour, so any reported figure must state which rate produced it.
///
/// # Panics
///
/// Panics if the multiplication overflows.
///
/// # Examples
///
/// ```
/// use public_value::philanthropy_metrics::volunteer_time_value;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // UK charity, ONS national-average replacement-cost rate.
/// let value = volunteer_time_value(dec!(5_000), Money::from_decimal(dec!(14.43), iso::USD));
/// assert_eq!(*value.amount(), dec!(72150.00));
/// ```
#[must_use]
pub fn volunteer_time_value(hours: Decimal, hourly_rate: Money) -> Money {
    hourly_rate.mul(hours).expect("multiplication overflow")
}
