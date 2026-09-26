//! Governmental performance and delivery metrics: public sector KPIs, the public value scorecard,
//! outcomes-based accountability, payment by results, public service productivity, citizen
//! satisfaction, service standards, and trust/legitimacy metrics.

use rust_decimal::Decimal;

use crate::impact_measurement::net_outcome_uplift;
use crate::units::{CURRENCY_INVARIANT, Money, Percentage};

/// The headline percentage a KPI reports: how many observations met the target, out of the total
/// observed.
///
/// Ported from `public-sector-kpis`. A single headline figure like this is exactly what Goodhart's
/// law warns against reading in isolation — the source doc's ambulance worked example shows a
/// trust hitting 75% on this number while its slowest-decile response time got dramatically worse,
/// which is why a defensible KPI is always paired with a counter-metric rather than reported alone.
///
/// # Panics
///
/// Panics if `total_observations` is zero.
///
/// # Examples
///
/// ```
/// use public_value::performance_metrics::kpi_percentage;
/// use public_value::units::Percentage;
///
/// // Ambulance trust: 4,500 of 6,000 Category A calls met within 8 minutes.
/// let headline = kpi_percentage(4_500, 6_000);
/// assert!((headline.as_percent() - 75.0).abs() < 0.01);
/// ```
#[must_use]
pub fn kpi_percentage(observations_meeting_target: u32, total_observations: u32) -> Percentage {
    Percentage::from_fraction(f64::from(observations_meeting_target) / f64::from(total_observations))
}

/// A public value scorecard for one service: one headline indicator per Kaplan-and-Norton-derived
/// perspective (mission, stewardship, customer, legitimacy, process), deliberately never collapsed
/// into a single score.
///
/// Ported from `public-value-scorecard`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PublicValueScorecard {
    /// Mission perspective: the population outcome rate achieved.
    pub mission_outcome_rate: Percentage,
    /// Mission perspective: the target outcome rate.
    pub mission_outcome_target: Percentage,
    /// Stewardship perspective: actual cost per completed unit of delivery.
    pub stewardship_cost_per_episode: Money,
    /// Stewardship perspective: the budgeted cost per completed unit of delivery.
    pub stewardship_budgeted_cost_per_episode: Money,
    /// Process perspective: current staff vacancy rate.
    pub process_staff_vacancy_rate: Percentage,
    /// Process perspective: current average caseload.
    pub process_caseload: u32,
    /// Process perspective: the caseload ceiling considered safe.
    pub process_safe_caseload_ceiling: u32,
}

impl PublicValueScorecard {
    /// True if the mission and stewardship perspectives both look like a success story on their
    /// own (outcome above target, cost under budget).
    ///
    /// # Examples
    ///
    /// ```
    /// use public_value::performance_metrics::PublicValueScorecard;
    /// use public_value::units::{Money, Percentage, iso};
    /// use rust_decimal_macros::dec;
    ///
    /// let reablement_service = PublicValueScorecard {
    ///     mission_outcome_rate: Percentage::from_percent(68.0),
    ///     mission_outcome_target: Percentage::from_percent(65.0),
    ///     stewardship_cost_per_episode: Money::from_decimal(dec!(1_850), iso::USD),
    ///     stewardship_budgeted_cost_per_episode: Money::from_decimal(dec!(2_000), iso::USD),
    ///     process_staff_vacancy_rate: Percentage::from_percent(14.0),
    ///     process_caseload: 23,
    ///     process_safe_caseload_ceiling: 25,
    /// };
    /// assert!(reablement_service.looks_like_a_success_on_headline_numbers());
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the two `Money` fields are not in the same currency (not reachable within this
    /// crate).
    #[must_use]
    pub fn looks_like_a_success_on_headline_numbers(&self) -> bool {
        self.mission_outcome_rate.as_fraction() >= self.mission_outcome_target.as_fraction()
            && self.stewardship_cost_per_episode.lte(&self.stewardship_budgeted_cost_per_episode).expect(CURRENCY_INVARIANT)
    }

    /// True if the process perspective shows a staffing risk the mission and stewardship numbers
    /// alone would never surface: a vacancy rate above `risk_threshold`, running close to the safe
    /// caseload ceiling.
    ///
    /// # Examples
    ///
    /// ```
    /// use public_value::performance_metrics::PublicValueScorecard;
    /// use public_value::units::{Money, Percentage, iso};
    /// use rust_decimal_macros::dec;
    ///
    /// let reablement_service = PublicValueScorecard {
    ///     mission_outcome_rate: Percentage::from_percent(68.0),
    ///     mission_outcome_target: Percentage::from_percent(65.0),
    ///     stewardship_cost_per_episode: Money::from_decimal(dec!(1_850), iso::USD),
    ///     stewardship_budgeted_cost_per_episode: Money::from_decimal(dec!(2_000), iso::USD),
    ///     process_staff_vacancy_rate: Percentage::from_percent(14.0),
    ///     process_caseload: 23,
    ///     process_safe_caseload_ceiling: 25,
    /// };
    /// // The "success story" mission and stewardship numbers hide a real staffing risk.
    /// assert!(reablement_service.looks_like_a_success_on_headline_numbers());
    /// assert!(reablement_service.has_hidden_staffing_risk(Percentage::from_percent(10.0)));
    /// ```
    #[must_use]
    pub fn has_hidden_staffing_risk(&self, risk_threshold: Percentage) -> bool {
        self.process_staff_vacancy_rate.as_fraction() > risk_threshold.as_fraction()
    }
}

/// Mark Friedman's three performance questions for one programme: how much did we do, how well did
/// we do it, and is anyone better off — judged only against a comparison group, never against a
/// population-level indicator the programme cannot control.
///
/// Ported from `outcomes-based-accountability`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PerformanceAccountability {
    /// How much: activity volume actually achieved.
    pub activity_achieved: u32,
    /// How much: the activity target.
    pub activity_target: u32,
    /// How well: the share of participants who completed the programme.
    pub completion_rate: Percentage,
    /// Better off: the outcome rate among this programme's completers.
    pub outcome_rate: Percentage,
    /// Better off: the outcome rate in a matched comparison group over the same period.
    pub comparison_group_outcome_rate: Percentage,
}

impl PerformanceAccountability {
    /// True if the activity volume target was met.
    #[must_use]
    pub const fn met_activity_target(&self) -> bool {
        self.activity_achieved >= self.activity_target
    }

    /// The "is anyone better off" uplift: this programme's outcome rate minus the matched
    /// comparison group's, i.e. the performance-accountability answer, never the raw population
    /// indicator.
    ///
    /// # Examples
    ///
    /// ```
    /// use public_value::performance_metrics::PerformanceAccountability;
    /// use public_value::units::Percentage;
    ///
    /// let programme = PerformanceAccountability {
    ///     activity_achieved: 500,
    ///     activity_target: 480,
    ///     completion_rate: Percentage::from_percent(78.0),
    ///     outcome_rate: Percentage::from_fraction(260.0 / 390.0),
    ///     comparison_group_outcome_rate: Percentage::from_percent(41.0),
    /// };
    /// assert!(programme.met_activity_target());
    /// assert!((programme.better_off_uplift().as_percent() - 25.7).abs() < 0.1);
    /// ```
    #[must_use]
    pub fn better_off_uplift(&self) -> Percentage {
        net_outcome_uplift(self.outcome_rate, self.comparison_group_outcome_rate)
    }
}

/// A payment-by-results contract's total payment: activity (base) payments plus outcome payments,
/// each counted and priced separately.
///
/// Ported from `payment-by-results-and-social-impact-bonds`.
///
/// # Examples
///
/// ```
/// use public_value::performance_metrics::payment_by_results_total;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // Family-intervention service: £4,000/referral activity payment, £6,000/outcome payment.
/// let total = payment_by_results_total(200, Money::from_decimal(dec!(4_000), iso::USD), 96, Money::from_decimal(dec!(6_000), iso::USD));
/// assert_eq!(*total.amount(), dec!(1_376_000));
/// ```
///
/// # Panics
///
/// Panics if a multiplication overflows, or if the two payment totals are not in the same
/// currency (not reachable within this crate).
#[must_use]
pub fn payment_by_results_total(
    activity_count: u32,
    activity_unit_price: Money,
    outcome_count: u32,
    outcome_unit_price: Money,
) -> Money {
    let activity_payment = activity_unit_price.mul(activity_count).expect("multiplication overflow");
    let outcome_payment = outcome_unit_price.mul(outcome_count).expect("multiplication overflow");
    activity_payment.add(outcome_payment).expect(CURRENCY_INVARIANT)
}

/// A quality-adjusted output index: a base index scaled by activity growth and a quality
/// adjustment factor, so that a service doing more of something lower-quality does not register as
/// more productive.
///
/// Ported from `public-service-productivity`.
///
/// # Examples
///
/// ```
/// use public_value::performance_metrics::quality_adjusted_output_index;
///
/// // NHS acute sector: +3.0% activity, −1.0% quality adjustment.
/// let index = quality_adjusted_output_index(100.0, 0.030, -0.010);
/// assert!((index - 101.97).abs() < 0.01);
/// ```
#[must_use]
pub fn quality_adjusted_output_index(base_index: f64, activity_growth: f64, quality_adjustment: f64) -> f64 {
    base_index * (1.0 + activity_growth) * (1.0 + quality_adjustment)
}

/// Total factor productivity growth, in percentage points: the growth rate of the quality-adjusted
/// output index minus the growth rate of the input index.
///
/// Ported from `public-service-productivity`. This is what lets "more care was delivered" and
/// "productivity fell" both be true at once — activity and inputs can both rise while inputs rise
/// faster.
///
/// # Panics
///
/// Panics if `base_index` is zero.
///
/// # Examples
///
/// ```
/// use public_value::performance_metrics::{productivity_growth_percentage_points, quality_adjusted_output_index};
///
/// let output_index = quality_adjusted_output_index(100.0, 0.030, -0.010);
/// let input_index = 100.0 * 1.032;
/// let growth = productivity_growth_percentage_points(output_index, input_index, 100.0);
/// assert!((growth - -1.23).abs() < 0.01);
/// ```
#[must_use]
pub fn productivity_growth_percentage_points(quality_adjusted_output_index: f64, input_index: f64, base_index: f64) -> f64 {
    (quality_adjusted_output_index / base_index - 1.0) * 100.0 - (input_index / base_index - 1.0) * 100.0
}

/// Net satisfaction: the share satisfied minus the share dissatisfied, both out of total
/// respondents (neutral responses counted in the base but excluded from both terms).
///
/// Ported from `citizen-satisfaction-metrics`.
///
/// # Panics
///
/// Panics if `total_respondents` is zero.
///
/// # Examples
///
/// ```
/// use public_value::performance_metrics::net_satisfaction;
///
/// // Council tax e-billing: 1,650 satisfied, 250 dissatisfied, 2,400 respondents.
/// let net = net_satisfaction(1_650, 250, 2_400);
/// assert!((net.as_percent() - 58.3).abs() < 0.1);
/// ```
#[must_use]
pub fn net_satisfaction(satisfied: u32, dissatisfied: u32, total_respondents: u32) -> Percentage {
    let base = f64::from(total_respondents);
    Percentage::from_fraction(f64::from(satisfied) / base - f64::from(dissatisfied) / base)
}

/// Cost per transaction: total service running cost divided by completed transactions.
///
/// Ported from `service-standards-and-transaction-metrics`.
///
/// # Panics
///
/// Panics if `completed_transactions` is zero.
#[must_use]
pub fn cost_per_transaction(total_running_cost: Money, completed_transactions: u32) -> Money {
    total_running_cost.div(completed_transactions).expect("division by zero or overflow")
}

/// The channel-shift saving from moving a share of transaction volume onto a cheaper channel: the
/// volume shifted times the per-transaction cost difference between the old and new channel.
///
/// Ported from `service-standards-and-transaction-metrics`.
///
/// # Panics
///
/// Panics if `take_up_shift` cannot be represented as a `Decimal` (not reachable for a finite
/// value).
///
/// # Examples
///
/// ```
/// use public_value::performance_metrics::channel_shift_saving;
/// use public_value::units::{Money, Percentage, iso};
/// use rust_decimal_macros::dec;
///
/// // Licence-renewal service: 2m transactions/year, a 25-point digital take-up shift, phone
/// // £3.00/transaction versus digital £0.30/transaction.
/// let saving = channel_shift_saving(2_000_000, Percentage::from_percent(25.0), Money::from_decimal(dec!(3.00), iso::USD), Money::from_decimal(dec!(0.30), iso::USD));
/// assert_eq!(saving.amount().round_dp(2), dec!(1_350_000.00));
/// ```
#[must_use]
pub fn channel_shift_saving(transaction_volume: u32, take_up_shift: Percentage, old_channel_cost: Money, new_channel_cost: Money) -> Money {
    let shift_fraction = Decimal::from_f64_retain(take_up_shift.as_fraction()).expect("finite take-up shift converts to Decimal");
    old_channel_cost
        .sub(new_channel_cost)
        .expect(CURRENCY_INVARIANT)
        .mul(shift_fraction)
        .expect("multiplication overflow")
        .mul(transaction_volume)
        .expect("multiplication overflow")
}

/// The failure-demand cost from users who attempt a channel but do not complete: the number who
/// attempted but did not complete, times the cost of the fallback channel they then use instead.
///
/// Ported from `service-standards-and-transaction-metrics`.
///
/// # Panics
///
/// Panics if the computed non-completing count cannot be represented as a `Decimal` (not reachable
/// for a finite completion rate).
///
/// # Examples
///
/// ```
/// use public_value::performance_metrics::failure_demand_cost;
/// use public_value::units::{Money, Percentage, iso};
/// use rust_decimal_macros::dec;
///
/// // Before redesign: 700,000 digital attempts/year at 80% completion, £3.00 phone fallback.
/// let before = failure_demand_cost(700_000, Percentage::from_percent(80.0), Money::from_decimal(dec!(3.00), iso::USD));
/// assert_eq!(before.amount().round_dp(2), dec!(420_000.00));
/// ```
#[must_use]
pub fn failure_demand_cost(channel_attempts: u32, completion_rate: Percentage, fallback_channel_cost: Money) -> Money {
    let non_completing_fraction =
        Decimal::from_f64_retain(1.0 - completion_rate.as_fraction()).expect("finite completion rate converts to Decimal");
    fallback_channel_cost
        .mul(non_completing_fraction)
        .expect("multiplication overflow")
        .mul(channel_attempts)
        .expect("multiplication overflow")
}

/// A legitimacy triangulation: current versus prior readings on three independent signals (an
/// institutional trust survey, upheld-complaints rate, and an independent ombudsman's uphold rate).
///
/// Ported from `trust-and-legitimacy-metrics`. No single proxy substitutes for legitimacy; the
/// source doc's discipline is that a credible finding requires several independent signals moving
/// together, not any one of them alone.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LegitimacyTriangulation {
    /// Current institutional trust survey reading.
    pub trust_index: Percentage,
    /// The same trust survey reading from the prior comparison period.
    pub trust_index_prior: Percentage,
    /// Current upheld complaints per 1,000 service users.
    pub upheld_complaints_per_1000: f64,
    /// Upheld complaints per 1,000 service users from the prior comparison period.
    pub upheld_complaints_per_1000_prior: f64,
    /// Current independent ombudsman uphold rate against the body.
    pub ombudsman_uphold_rate: Percentage,
    /// Ombudsman uphold rate from the prior comparison period.
    pub ombudsman_uphold_rate_prior: Percentage,
}

impl LegitimacyTriangulation {
    /// True only if all three signals move together in the same, negative direction (trust
    /// falling, upheld complaints rising, ombudsman findings increasingly against the body) — the
    /// pattern the source doc treats as a credible legitimacy finding rather than noise in any one
    /// series.
    ///
    /// # Examples
    ///
    /// ```
    /// use public_value::performance_metrics::LegitimacyTriangulation;
    /// use public_value::units::Percentage;
    ///
    /// let tax_authority = LegitimacyTriangulation {
    ///     trust_index: Percentage::from_percent(58.0),
    ///     trust_index_prior: Percentage::from_percent(64.0),
    ///     upheld_complaints_per_1000: 4.2,
    ///     upheld_complaints_per_1000_prior: 3.1,
    ///     ombudsman_uphold_rate: Percentage::from_percent(61.0),
    ///     ombudsman_uphold_rate_prior: Percentage::from_percent(48.0),
    /// };
    /// assert!(tax_authority.converging_negative_signal());
    /// ```
    #[must_use]
    pub fn converging_negative_signal(&self) -> bool {
        self.trust_index.as_fraction() < self.trust_index_prior.as_fraction()
            && self.upheld_complaints_per_1000 > self.upheld_complaints_per_1000_prior
            && self.ombudsman_uphold_rate.as_fraction() > self.ombudsman_uphold_rate_prior.as_fraction()
    }
}

/// A Net Promoter Score: the share of respondents who are promoters (typically scoring 9–10 on a
/// 0–10 likelihood-to-recommend question) minus the share who are detractors (typically scoring
/// 0–6), on a −100 to +100 scale.
///
/// This is not one of the 64 topics ported from `public-value-metrics`, but it is a standard
/// companion to [`net_satisfaction`]'s citizen-satisfaction measurement — many public digital
/// services report both alongside each other, and NPS's promoter/detractor framing (rather than
/// `net_satisfaction`'s satisfied/dissatisfied framing) is what some funders and service standards
/// specifically ask for. As with `net_satisfaction`, this is a service-level experience measure,
/// not an outcome measure — see [`crate::impact_measurement::net_outcome_uplift`] for the
/// outcome-level counterpart.
///
/// # Panics
///
/// Panics if `total_respondents` is zero.
///
/// # Examples
///
/// ```
/// use public_value::performance_metrics::net_promoter_score;
///
/// // 620 promoters, 180 detractors, out of 1,000 respondents.
/// let nps = net_promoter_score(620, 180, 1_000);
/// assert!((nps - 44.0).abs() < 1e-9);
/// ```
#[must_use]
pub fn net_promoter_score(promoters: u32, detractors: u32, total_respondents: u32) -> f64 {
    let base = f64::from(total_respondents);
    100.0 * (f64::from(promoters) / base - f64::from(detractors) / base)
}
