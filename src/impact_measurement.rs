//! Social value and impact measurement: social return on investment, theory of change, logic
//! models, the outcomes-vs-outputs distinction, the UK Social Value Act, unit cost databases, and
//! impact evaluation methods.

use rust_decimal::Decimal;

use crate::units::{Money, Percentage, Ratio, money_ratio};

/// Applies a retained fraction (`1 − rate`) to a value — the shared shape behind social return on
/// investment's deadweight, attribution, and drop-off adjustments, each of which multiplies the
/// remaining value by `(1 − rate)` in sequence.
///
/// Ported from `social-return-on-investment`.
///
/// # Panics
///
/// Panics if the retained fraction cannot be represented as a `Decimal` (not reachable for a
/// finite percentage).
///
/// # Examples
///
/// ```
/// use public_value::impact_measurement::value_net_of_rate;
/// use public_value::units::{Money, Percentage, iso};
/// use rust_decimal_macros::dec;
///
/// // SROI worked example: £510,000 gross, less 40% deadweight.
/// let net_of_deadweight = value_net_of_rate(Money::from_decimal(dec!(510_000), iso::USD), Percentage::from_percent(40.0));
/// assert_eq!(net_of_deadweight.amount().round_dp(2), dec!(306_000.0));
/// ```
#[must_use]
pub fn value_net_of_rate(value: Money, rate: Percentage) -> Money {
    let retained_fraction = 1.0 - rate.as_fraction();
    let decimal_fraction =
        Decimal::from_f64_retain(retained_fraction).expect("finite retained fraction converts to Decimal");
    value.mul(decimal_fraction).expect("multiplication overflow")
}

/// The SROI ratio: present value of impact divided by the value of inputs.
///
/// Ported from `social-return-on-investment`. Compose this with [`value_net_of_rate`] (for
/// deadweight, attribution, and drop-off) and [`crate::foundations::present_value`] (for
/// discounting a later year's impact) to reach the present value of impact — this function is
/// deliberately just the final division, not a monolithic calculator, so a caller can inspect each
/// named intermediate the source doc's worked example calls out.
///
/// # Panics
///
/// Panics if `value_of_inputs` is zero.
///
/// # Examples
///
/// ```
/// use public_value::foundations::present_value;
/// use public_value::impact_measurement::{sroi_ratio, value_net_of_rate};
/// use public_value::units::{Money, Percentage, iso};
/// use rust_decimal_macros::dec;
///
/// // Local authority employment programme: £250,000 input, 60 participants at £8,500/year proxy.
/// let gross = Money::from_decimal(dec!(8_500), iso::USD).mul(60_u32).unwrap();
/// let year1_impact = value_net_of_rate(
///     value_net_of_rate(gross, Percentage::from_percent(40.0)), // deadweight
///     Percentage::from_percent(30.0),                           // attribution
/// );
/// let year2_impact = present_value(
///     value_net_of_rate(year1_impact, Percentage::from_percent(30.0)), // 30% drop-off
///     dec!(0.035),
///     1,
/// );
/// let total_impact = year1_impact.add(year2_impact).unwrap();
/// let ratio = sroi_ratio(total_impact, Money::from_decimal(dec!(250_000), iso::USD));
/// assert!((ratio.value() - 1.436).abs() < 0.01);
/// ```
#[must_use]
pub fn sroi_ratio(present_value_of_impact: Money, value_of_inputs: Money) -> Ratio {
    money_ratio(present_value_of_impact, value_of_inputs)
}

/// One link in a backward-mapped causal chain: a precondition, the assumption connecting it to the
/// outcome above it, and an indicator that could show the assumption is false.
///
/// Ported from `theory-of-change`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalLink {
    /// What has to be true immediately before the outcome above this link, for it to happen.
    pub precondition: String,
    /// The (testable, falsifiable) belief connecting this precondition to the outcome above it.
    pub assumption: String,
    /// What would show the assumption is false.
    pub indicator: String,
}

/// A backward-mapped theory of change: a long-term outcome plus the chain of preconditions,
/// assumptions, and indicators that must hold for it to be reached.
///
/// Ported from `theory-of-change`. Structural rather than numeric — see
/// [`assumption_hold_rate`] for the one quantitative step the source doc's worked example performs
/// (testing whether a specific link's assumption held in a pilot cohort).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheoryOfChange {
    /// The long-term goal the chain works backward from.
    pub long_term_outcome: String,
    /// The backward-mapped chain, ordered from the activities nearest the long-term outcome.
    pub links: Vec<CausalLink>,
}

impl TheoryOfChange {
    /// The number of causal links in the chain.
    #[must_use]
    pub fn link_count(&self) -> usize {
        self.links.len()
    }
}

/// The rate at which a specific causal link's assumption held in an observed cohort — the one
/// quantitative check a theory of change performs, testing one link at a time rather than making a
/// single end-to-end claim about the long-term outcome.
///
/// Ported from `theory-of-change`.
///
/// # Panics
///
/// Panics if `total_cohort` is zero.
///
/// # Examples
///
/// ```
/// use public_value::impact_measurement::assumption_hold_rate;
///
/// // Homelessness prevention pilot: the benefits-calculator assumption held for 102 of 120
/// // households.
/// let rate = assumption_hold_rate(102, 120);
/// assert!((rate.as_percent() - 85.0).abs() < 0.01);
/// ```
#[must_use]
pub fn assumption_hold_rate(cohort_where_assumption_held: u32, total_cohort: u32) -> Percentage {
    Percentage::from_fraction(f64::from(cohort_where_assumption_held) / f64::from(total_cohort))
}

/// A logic model's five-column accountability chain: inputs, activities, outputs, outcomes, and
/// impact.
///
/// Ported from `logic-model`. The forward-facing counterpart to [`TheoryOfChange`]'s backward
/// mapping.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LogicModel {
    /// Resources committed (e.g. budget, staff time).
    pub inputs: Vec<String>,
    /// What is done with the inputs.
    pub activities: Vec<String>,
    /// Direct, countable products of the activities, true regardless of effect.
    pub outputs: Vec<String>,
    /// Changes for beneficiaries that followed the outputs.
    pub outcomes: Vec<String>,
    /// Long-term or population-level change, often only partly attributable to this programme
    /// alone.
    pub impact: Vec<String>,
}

impl LogicModel {
    /// True if the outcomes column is empty — the source doc's central pitfall: a model with
    /// populated inputs, activities, and outputs but nothing recorded as actually changing for
    /// beneficiaries.
    ///
    /// # Examples
    ///
    /// ```
    /// use public_value::impact_measurement::LogicModel;
    ///
    /// let outputs_only = LogicModel {
    ///     outputs: vec!["900 appointments delivered".to_owned()],
    ///     ..LogicModel::default()
    /// };
    /// assert!(outputs_only.stops_at_outputs());
    /// ```
    #[must_use]
    pub fn stops_at_outputs(&self) -> bool {
        !self.outputs.is_empty() && self.outcomes.is_empty()
    }
}

/// The net outcome uplift: the treated group's outcome rate minus a comparison group's baseline
/// rate over the same period.
///
/// Ported from `outcomes-vs-outputs`.
///
/// # Examples
///
/// ```
/// use public_value::impact_measurement::net_outcome_uplift;
/// use public_value::units::Percentage;
///
/// // Employment support: 28% of attendees in sustained work at 12 months, versus a 15% baseline.
/// let uplift = net_outcome_uplift(Percentage::from_percent(28.0), Percentage::from_percent(15.0));
/// assert!((uplift.as_percent() - 13.0).abs() < 0.01);
/// ```
#[must_use]
pub fn net_outcome_uplift(treated_rate: Percentage, comparison_rate: Percentage) -> Percentage {
    Percentage::from_fraction(treated_rate.as_fraction() - comparison_rate.as_fraction())
}

/// The estimated number of people attributably helped: population size times the net outcome
/// uplift — distinct from both the raw output count and the raw (un-net-of-baseline) outcome
/// count.
///
/// Ported from `outcomes-vs-outputs`.
///
/// # Examples
///
/// ```
/// use public_value::impact_measurement::{attributable_outcome_count, net_outcome_uplift};
/// use public_value::units::Percentage;
///
/// let uplift = net_outcome_uplift(Percentage::from_percent(28.0), Percentage::from_percent(15.0));
/// let attributable = attributable_outcome_count(500, uplift);
/// assert!((attributable - 65.0).abs() < 0.01);
/// ```
#[must_use]
pub fn attributable_outcome_count(population: u32, net_uplift: Percentage) -> f64 {
    f64::from(population) * net_uplift.as_fraction()
}

/// A Social Value Act tender's proportional social value score: the maximum available points,
/// scaled by this bid's monetized social value relative to the strongest bid's.
///
/// Ported from `social-value-act`.
///
/// # Panics
///
/// Panics if `strongest_bid_value` is zero.
///
/// # Examples
///
/// ```
/// use public_value::impact_measurement::proportional_social_value_score;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // A £2m IT contract with a 10-point social value weighting; the strongest bid monetizes
/// // £90,000 of social value, this bid £40,000.
/// let score = proportional_social_value_score(Money::from_decimal(dec!(40_000), iso::USD), Money::from_decimal(dec!(90_000), iso::USD), 10.0);
/// assert!((score - 4.44).abs() < 0.01);
/// ```
#[must_use]
pub fn proportional_social_value_score(bid_value: Money, strongest_bid_value: Money, max_points: f64) -> f64 {
    max_points * money_ratio(bid_value, strongest_bid_value).value()
}

/// The applied value of an outcome, using a published unit cost database proxy: outcomes achieved
/// times the proxy's per-unit value.
///
/// Ported from `unit-cost-databases`.
///
/// # Panics
///
/// Panics if the multiplication overflows.
///
/// # Examples
///
/// ```
/// use public_value::impact_measurement::applied_unit_cost_value;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // Befriending service: 80 beneficiaries at an illustrative £1,100/person/year loneliness proxy.
/// let value = applied_unit_cost_value(80, Money::from_decimal(dec!(1_100), iso::USD));
/// assert_eq!(*value.amount(), dec!(88_000));
/// ```
#[must_use]
pub fn applied_unit_cost_value(outcomes_achieved: u32, unit_proxy_value: Money) -> Money {
    unit_proxy_value.mul(outcomes_achieved).expect("multiplication overflow")
}

/// A difference-in-differences impact estimate, re-exported here under the name
/// `impact-evaluation-methods` uses for it (see [`crate::foundations::difference_in_differences`]
/// for the shared implementation and its doctest).
pub use crate::foundations::difference_in_differences as difference_in_differences_impact;

/// A propensity score matching (PSM) impact estimate: the treated group's outcome rate minus the
/// matched comparison group's outcome rate.
///
/// Ported from `impact-evaluation-methods`. Valid only conditional on no unobserved confounder
/// (such as motivation) driving both participation and the outcome — matching balances *observed*
/// covariates only.
///
/// # Examples
///
/// ```
/// use public_value::impact_measurement::propensity_score_matching_impact;
/// use public_value::units::Percentage;
///
/// // Employability charity: matched treated 46% employed at 12 months, matched comparison 33%.
/// let impact = propensity_score_matching_impact(Percentage::from_percent(46.0), Percentage::from_percent(33.0));
/// assert!((impact.as_percent() - 13.0).abs() < 0.01);
/// ```
#[must_use]
pub fn propensity_score_matching_impact(treated_rate: Percentage, matched_comparison_rate: Percentage) -> Percentage {
    Percentage::from_fraction(treated_rate.as_fraction() - matched_comparison_rate.as_fraction())
}

/// The combined diagnosis from pairing an impact evaluation (did an effect appear?) with a process
/// evaluation (was the programme delivered with fidelity?), per the source doc's 2×2 diagnostic
/// table.
///
/// Ported from `impact-evaluation-vs-process-evaluation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombinedDiagnosis {
    /// No effect, delivered faithfully: the programme model itself did not produce the outcome.
    TheoryFailure,
    /// No effect, delivered poorly: the model was never properly tested.
    ImplementationFailure,
    /// Effect found, delivered faithfully: replicate with confidence.
    ReplicateWithConfidence,
    /// Effect found, delivered poorly: the effect may be fragile or site-specific.
    InvestigateFurther,
}

/// Diagnoses a paired impact/process evaluation result.
///
/// Ported from `impact-evaluation-vs-process-evaluation`.
///
/// # Examples
///
/// ```
/// use public_value::impact_measurement::{diagnose, CombinedDiagnosis};
///
/// // Parenting programme: a non-significant impact result, and only 19% of planned reach achieved
/// // the pre-specified fidelity threshold.
/// assert_eq!(diagnose(false, false), CombinedDiagnosis::ImplementationFailure);
///
/// // Digital literacy programme: a strong effect, and 92% fidelity across all delivery sites.
/// assert_eq!(diagnose(true, true), CombinedDiagnosis::ReplicateWithConfidence);
/// ```
#[must_use]
pub const fn diagnose(effect_found: bool, high_fidelity: bool) -> CombinedDiagnosis {
    match (effect_found, high_fidelity) {
        (false, true) => CombinedDiagnosis::TheoryFailure,
        (false, false) => CombinedDiagnosis::ImplementationFailure,
        (true, true) => CombinedDiagnosis::ReplicateWithConfidence,
        (true, false) => CombinedDiagnosis::InvestigateFurther,
    }
}
