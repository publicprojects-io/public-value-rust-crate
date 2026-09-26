//! Wellbeing, equity and societal indicators: GDP alternatives, the Human Development Index, the
//! Multidimensional Poverty Index, wellbeing-adjusted life years, the Index of Multiple
//! Deprivation, social capital metrics, natural capital accounting, and intergenerational equity.

use rust_decimal::Decimal;

use crate::economic_appraisal::annuity_factor;
use crate::foundations::discount_factor;
use crate::units::{CURRENCY_INVARIANT, Money, Percentage};

/// The Genuine Progress Indicator (GPI): personal consumption plus non-market benefits GDP omits,
/// minus defensive/social costs and capital-depletion costs GDP wrongly counts as positive.
///
/// Ported from `gdp-alternatives`.
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::genuine_progress_indicator;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // Region: $50bn consumption, +$12bn non-market benefits, −$3bn/−$4bn/−$6bn costs.
/// let gpi = genuine_progress_indicator(
///     Money::from_decimal(dec!(50), iso::USD),
///     Money::from_decimal(dec!(12), iso::USD),
///     Money::from_decimal(dec!(3), iso::USD).add(Money::from_decimal(dec!(4), iso::USD)).unwrap(),
///     Money::from_decimal(dec!(6), iso::USD),
/// );
/// assert_eq!(*gpi.amount(), dec!(49));
/// ```
///
/// # Panics
///
/// Panics if the four amounts are not in the same currency (not reachable within this crate).
#[must_use]
pub fn genuine_progress_indicator(
    personal_consumption: Money,
    non_market_benefits: Money,
    defensive_and_social_costs: Money,
    depletion_costs: Money,
) -> Money {
    personal_consumption
        .add(non_market_benefits)
        .expect(CURRENCY_INVARIANT)
        .sub(defensive_and_social_costs)
        .expect(CURRENCY_INVARIANT)
        .sub(depletion_costs)
        .expect(CURRENCY_INVARIANT)
}

/// Whether a person clears Bhutan's Gross National Happiness "sufficiency" bar: sufficient in at
/// least `domains_required` of the domains scored.
///
/// Ported from `gdp-alternatives`.
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::gnh_sufficiency;
///
/// // Sufficient in 7 of 9 domains; the GNH bar is 6.
/// assert!(gnh_sufficiency(7, 6));
/// ```
#[must_use]
pub const fn gnh_sufficiency(domains_sufficient: u32, domains_required: u32) -> bool {
    domains_sufficient >= domains_required
}

/// The Human Development Index and its three sub-indices, computed as UNDP's 2010-onward
/// methodology defines them: a geometric, not arithmetic, mean of life expectancy, education, and
/// income sub-indices, so a very high score on one dimension cannot fully offset a very low score
/// on another.
///
/// Ported from `human-development-index`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HumanDevelopmentIndex {
    /// Life Expectancy Index: `(LE − 20) / (85 − 20)`.
    pub life_expectancy_index: f64,
    /// Education Index: the mean of the mean-years and expected-years schooling sub-indices.
    pub education_index: f64,
    /// Income Index: a log transform of GNI per capita, reflecting diminishing marginal value of
    /// income.
    pub income_index: f64,
    /// The composite HDI: the geometric mean of the three sub-indices.
    pub hdi: f64,
}

/// Computes the Human Development Index from its four underlying UNDP inputs.
///
/// Ported from `human-development-index`.
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::human_development_index;
///
/// // Middle-income country: life expectancy 72, mean schooling 8, expected schooling 13, GNI
/// // per capita $12,000.
/// let result = human_development_index(72.0, 8.0, 13.0, 12_000.0);
/// assert!((result.life_expectancy_index - 0.800).abs() < 0.001);
/// assert!((result.education_index - 0.628).abs() < 0.001);
/// assert!((result.income_index - 0.723).abs() < 0.001);
/// assert!((result.hdi - 0.713).abs() < 0.001);
/// ```
#[must_use]
pub fn human_development_index(
    life_expectancy_years: f64,
    mean_years_schooling: f64,
    expected_years_schooling: f64,
    gni_per_capita: f64,
) -> HumanDevelopmentIndex {
    let life_expectancy_index = (life_expectancy_years - 20.0) / (85.0 - 20.0);
    let mean_years_index = mean_years_schooling / 15.0;
    let expected_years_index = expected_years_schooling / 18.0;
    let education_index = f64::midpoint(mean_years_index, expected_years_index);
    let income_index = (gni_per_capita.ln() - 100.0_f64.ln()) / (75_000.0_f64.ln() - 100.0_f64.ln());
    // Geometric, not arithmetic, mean: UNDP's 2010 methodology change so a high score on one
    // dimension cannot buy back a shortfall in another, unlike an average would allow.
    let hdi = (life_expectancy_index * education_index * income_index).powf(1.0 / 3.0);
    HumanDevelopmentIndex { life_expectancy_index, education_index, income_index, hdi }
}

/// The Multidimensional Poverty Index headcount ratio (H): the share of the population classed as
/// MPI poor.
///
/// Ported from `multidimensional-poverty-index`.
///
/// # Panics
///
/// Panics if `total_population` is zero.
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::mpi_headcount_ratio;
///
/// let h = mpi_headcount_ratio(350, 1_000);
/// assert!((h.as_fraction() - 0.350).abs() < 0.001);
/// ```
#[must_use]
pub fn mpi_headcount_ratio(mpi_poor_count: u32, total_population: u32) -> Percentage {
    Percentage::from_fraction(f64::from(mpi_poor_count) / f64::from(total_population))
}

/// The Multidimensional Poverty Index: the headcount ratio (H) times the average intensity of
/// deprivation among the poor (A). Two areas with equal headcount can have very different MPI if
/// deprivation is more severe in one.
///
/// Ported from `multidimensional-poverty-index`.
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::{mpi_headcount_ratio, multidimensional_poverty_index};
/// use public_value::units::Percentage;
///
/// let h = mpi_headcount_ratio(350, 1_000);
/// let mpi = multidimensional_poverty_index(h, Percentage::from_percent(45.0));
/// assert!((mpi - 0.1575).abs() < 0.0001);
/// ```
#[must_use]
pub fn multidimensional_poverty_index(headcount_ratio: Percentage, intensity: Percentage) -> f64 {
    headcount_ratio.as_fraction() * intensity.as_fraction()
}

/// The contribution of one Index of Multiple Deprivation domain to the composite score: the
/// domain's standardized deprivation score times its published weight.
///
/// Ported from `index-of-multiple-deprivation`.
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::{imd_composite_score, imd_domain_contribution};
/// use public_value::units::Percentage;
///
/// let income = imd_domain_contribution(0.35, Percentage::from_percent(22.5));
/// let employment = imd_domain_contribution(0.30, Percentage::from_percent(22.5));
/// let composite = imd_composite_score(&[income, employment]);
/// assert!((composite - 0.14625).abs() < 0.0001);
/// ```
#[must_use]
pub fn imd_domain_contribution(standardized_domain_score: f64, domain_weight: Percentage) -> f64 {
    standardized_domain_score * domain_weight.as_fraction()
}

/// The IMD composite score: the sum of every domain's weighted contribution.
///
/// Ported from `index-of-multiple-deprivation`.
#[must_use]
pub fn imd_composite_score(domain_contributions: &[f64]) -> f64 {
    domain_contributions.iter().sum()
}

/// The IMD decile an area falls into, given its rank (1 = most deprived) out of the total number
/// of areas ranked — decile 1 is the most deprived 10%.
///
/// Ported from `index-of-multiple-deprivation`.
///
/// # Panics
///
/// Panics if `total_areas` is zero.
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::imd_decile;
///
/// // Rank 2,950 out of 32,844 LSOAs falls in decile 1 (most deprived 10%).
/// assert_eq!(imd_decile(2_950, 32_844), 1);
/// ```
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
// A decile is always in 1..=10 for any in-range rank, so the f64→u32 cast below cannot lose
// meaningful precision or go negative.
pub fn imd_decile(rank: u32, total_areas: u32) -> u32 {
    let areas_per_decile = f64::from(total_areas) / 10.0;
    ((f64::from(rank) / areas_per_decile).ceil() as u32).max(1)
}

/// A local area's reading on one of the ONS's four social capital pillars, compared against the
/// national average for that pillar, expressed as the shortfall (national minus local). ONS
/// deliberately publishes the four pillars separately rather than one composite score, so this
/// crate does the same rather than inventing an aggregate.
///
/// Ported from `social-capital-metrics`.
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::social_capital_pillar_gap;
/// use public_value::units::Percentage;
///
/// // Civic engagement: 24% local versus 30% national — a 6-point gap.
/// let gap = social_capital_pillar_gap(Percentage::from_percent(24.0), Percentage::from_percent(30.0));
/// assert!((gap.as_percent() - 6.0).abs() < 0.01);
/// ```
#[must_use]
pub fn social_capital_pillar_gap(local_reading: Percentage, national_average: Percentage) -> Percentage {
    Percentage::from_fraction(national_average.as_fraction() - local_reading.as_fraction())
}

/// The net present value of an ecosystem service asset: its annual service flow value, discounted
/// over its expected service life via [`crate::economic_appraisal::annuity_factor`].
///
/// Ported from `natural-capital-accounting`. Uses the same net-present-value structure as any
/// other Green Book appraisal — natural capital accounting's contribution is credible physical
/// quantities and unit values for services that were previously priced at zero, not a different
/// discounting method.
///
/// # Panics
///
/// Panics if `annual_rate` is zero.
///
/// # Examples
///
/// ```
/// use public_value::economic_appraisal::annuity_factor;
/// use public_value::societal_indicators::ecosystem_asset_value;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // Urban woodland: 80,000 recreational visits/year at £3/visit, 50-year horizon, 3.5% discount
/// // rate.
/// let annual_value = Money::from_decimal(dec!(3), iso::USD).mul(80_000_u32).unwrap();
/// let asset_value = ecosystem_asset_value(annual_value, dec!(0.035), 50);
/// assert!((asset_value.amount() - dec!(5_629_348)).abs() < dec!(1_000));
/// ```
#[must_use]
pub fn ecosystem_asset_value(annual_service_flow_value: Money, annual_rate: Decimal, years: u32) -> Money {
    annual_service_flow_value.mul(annuity_factor(annual_rate, years)).expect("multiplication overflow")
}

/// The present value of a future sum discounted across several successive rate bands (a
/// *declining* discount schedule), by multiplying each band's compound discount factor together.
///
/// Ported from `intergenerational-equity-and-sustainability-discounting`. Exposes the schedule as
/// an explicit, visible parameter rather than a single buried rate, per the source doc's central
/// software-engineering recommendation.
///
/// # Panics
///
/// Panics if the combined discount factor is zero (not reachable for realistic rate bands).
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::present_value_across_schedule;
/// use public_value::units::{Money, iso};
/// use rust_decimal_macros::dec;
///
/// // £1 of harm avoided in 100 years, under the Green Book's declining schedule (3.5% for years
/// // 1–30, 3.0% for years 31–75, 2.5% for years 76–100).
/// let pv = present_value_across_schedule(Money::from_decimal(dec!(1), iso::USD), &[(dec!(0.035), 30), (dec!(0.03), 45), (dec!(0.025), 25)]);
/// assert!((pv.amount() - dec!(0.0508)).abs() < dec!(0.001));
/// ```
#[must_use]
pub fn present_value_across_schedule(future_value: Money, rate_year_bands: &[(Decimal, u32)]) -> Money {
    let mut combined_factor = Decimal::ONE;
    for &(rate, years) in rate_year_bands {
        combined_factor *= discount_factor(rate, years);
    }
    future_value.div(combined_factor).expect("division by zero or overflow")
}

/// The Gini coefficient of an income (or other) distribution: twice the area between the Lorenz
/// curve and the line of perfect equality, computed via the trapezoidal rule from cumulative
/// population and income shares. `0` is perfect equality, `1` is maximal inequality.
///
/// This is not one of the 64 topics ported from `public-value-metrics`, but it is the standard
/// companion to [`crate::foundations::distributional_weight`] and [`multidimensional_poverty_index`]
/// for describing how unequally a benefit or an income distribution actually lands, so it is
/// implemented here to complete that family of equity measures.
///
/// # Panics
///
/// Panics if `cumulative_population_shares` and `cumulative_income_shares` have different lengths,
/// or either is empty.
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::gini_coefficient;
///
/// // A population in quintiles earning 4%, 10%, 16%, 24%, and 46% of total income respectively
/// // (cumulative: 4%, 14%, 30%, 54%, 100%) — a moderately unequal distribution.
/// let gini = gini_coefficient(&[0.2, 0.4, 0.6, 0.8, 1.0], &[0.04, 0.14, 0.30, 0.54, 1.00]);
/// assert!((gini - 0.392).abs() < 0.001);
/// ```
#[must_use]
pub fn gini_coefficient(cumulative_population_shares: &[f64], cumulative_income_shares: &[f64]) -> f64 {
    assert_eq!(
        cumulative_population_shares.len(),
        cumulative_income_shares.len(),
        "population and income share slices must have the same length"
    );
    assert!(!cumulative_population_shares.is_empty(), "share slices must not be empty");

    let mut area_under_lorenz_curve = 0.0;
    let mut previous_population_share = 0.0;
    let mut previous_income_share = 0.0;
    for (&population_share, &income_share) in cumulative_population_shares.iter().zip(cumulative_income_shares) {
        // Trapezoidal rule: the area of each slice between consecutive Lorenz-curve points.
        area_under_lorenz_curve +=
            (population_share - previous_population_share) * (income_share + previous_income_share) / 2.0;
        previous_population_share = population_share;
        previous_income_share = income_share;
    }
    1.0 - 2.0 * area_under_lorenz_curve
}

/// The Atkinson inequality measure for one HDI dimension: `1 − (geometric mean / arithmetic
/// mean)` of the underlying distribution. `0` means no inequality in that dimension; it approaches
/// `1` as inequality grows.
///
/// This is not one of the 64 topics ported from `public-value-metrics`, but the source topic
/// behind [`human_development_index`]'s own "Pitfalls" section explicitly names "UNDP's separate
/// Inequality-adjusted HDI" as the tool for the distributional gap HDI itself cannot see, so the
/// Atkinson measure and [`inequality_adjusted_hdi`] are implemented here to complete that
/// connection (UNDP, IHDI technical notes, building on Foster, Lopez-Calva and Szekely's 2005
/// distribution-sensitive composite index method).
///
/// # Panics
///
/// Panics if `arithmetic_mean` is zero.
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::atkinson_inequality_measure;
///
/// let a = atkinson_inequality_measure(0.95, 1.0); // geometric mean 5% below arithmetic mean
/// assert!((a - 0.05).abs() < 1e-9);
/// ```
#[must_use]
pub fn atkinson_inequality_measure(geometric_mean: f64, arithmetic_mean: f64) -> f64 {
    1.0 - geometric_mean / arithmetic_mean
}

/// One HDI sub-index, discounted by that dimension's Atkinson inequality measure: `index × (1 −
/// Atkinson measure)`.
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::inequality_adjusted_dimension_index;
///
/// let adjusted = inequality_adjusted_dimension_index(0.800, 0.05);
/// assert!((adjusted - 0.760).abs() < 1e-9);
/// ```
#[must_use]
pub fn inequality_adjusted_dimension_index(dimension_index: f64, atkinson_measure: f64) -> f64 {
    dimension_index * (1.0 - atkinson_measure)
}

/// The Inequality-adjusted Human Development Index (IHDI): the geometric mean of the three
/// inequality-adjusted dimension indices. The IHDI equals the HDI when there is no inequality in
/// any dimension, and falls below it as inequality rises.
///
/// # Examples
///
/// ```
/// use public_value::societal_indicators::{human_development_index, ihdi_loss_percentage, inequality_adjusted_dimension_index, inequality_adjusted_hdi};
///
/// let hdi = human_development_index(72.0, 8.0, 13.0, 12_000.0);
/// let ihdi = inequality_adjusted_hdi(
///     inequality_adjusted_dimension_index(hdi.life_expectancy_index, 0.05),
///     inequality_adjusted_dimension_index(hdi.education_index, 0.10),
///     inequality_adjusted_dimension_index(hdi.income_index, 0.20),
/// );
/// assert!((ihdi - 0.6287).abs() < 0.001);
///
/// let loss = ihdi_loss_percentage(hdi.hdi, ihdi);
/// assert!((loss.as_percent() - 11.89).abs() < 0.1);
/// ```
#[must_use]
pub fn inequality_adjusted_hdi(adjusted_health_index: f64, adjusted_education_index: f64, adjusted_income_index: f64) -> f64 {
    (adjusted_health_index * adjusted_education_index * adjusted_income_index).powf(1.0 / 3.0)
}

/// The overall loss in human development due to inequality: `1 − IHDI/HDI`.
///
/// # Panics
///
/// Panics if `hdi` is zero.
#[must_use]
pub fn ihdi_loss_percentage(hdi: f64, ihdi: f64) -> Percentage {
    Percentage::from_fraction(1.0 - ihdi / hdi)
}
