# Public value cheat sheet

Every amount below is `Money` tagged `iso::USD`; `Percentage` stores a fraction and `Ratio` stores
an `f64`. Follow each module link for canonical signatures, panic conditions, and runnable worked
examples.

## Foundations

[`foundations`](../src/foundations.rs): `StrategicTriangle::passes`,
`ValueForMoneyOption::{annual_efficiency_cost,total_cost}`, `net_public_value`, `discount_factor`,
`present_value`, `distributional_weight`, `weighted_benefit`, `net_additional_outcomes`,
`net_additional_impact`, `attribution_shares_valid`, `difference_in_differences`,
`leakage_adjustment`, and `multiplier_effect`. These cover strategic authorization, value for
money, discounting, distribution, and net-impact adjustments.

## Economic appraisal

[`economic_appraisal`](../src/economic_appraisal.rs): `FiveCaseModel::proceeds`, `annuity_factor`,
`net_present_social_value`, `benefit_cost_ratio`, `cost_effectiveness_ratio`,
`incremental_cost_effectiveness_ratio`, `weighted_score`, `wellbys`, `monetize_wellbys`,
`aggregate_stated_preference_value`, `hedonic_implicit_price`, `travel_cost_annual_value`,
`shadow_carbon_benefit`, `shadow_wage_rate`, `net_social_benefit_per_hour`, `qaly`,
`clears_qaly_threshold`, `years_of_life_lost`, `years_lived_with_disability`,
`disability_adjusted_life_years`, `real_value`, `value_of_statistical_life`, and
`internal_rate_of_return`.

## Impact measurement

[`impact_measurement`](../src/impact_measurement.rs): `value_net_of_rate`, `sroi_ratio`,
`TheoryOfChange::link_count`, `assumption_hold_rate`, `LogicModel::stops_at_outputs`,
`net_outcome_uplift`, `attributable_outcome_count`, `proportional_social_value_score`,
`applied_unit_cost_value`, `propensity_score_matching_impact`, and `diagnose`.

## Performance metrics

[`performance_metrics`](../src/performance_metrics.rs): `kpi_percentage`, the
`PublicValueScorecard` and `PerformanceAccountability` methods, `payment_by_results_total`,
`quality_adjusted_output_index`, `productivity_growth_percentage_points`, `net_satisfaction`,
`cost_per_transaction`, `channel_shift_saving`, `failure_demand_cost`,
`LegitimacyTriangulation::converging_negative_signal`, and `net_promoter_score`.

## Philanthropy metrics

[`philanthropy_metrics`](../src/philanthropy_metrics.rs): `cost_per_outcome`,
`cost_per_beneficiary`, `BlendedValue::new`, `effective_altruism_cost_effectiveness`,
`units_produced`, `charity_overhead_ratio`, `programme_ratio`, `donor_return_on_investment`,
`bespoke_reporting_relationships`, `standardized_reporting_mappings`, and `volunteer_time_value`.

## Digital government

[`digital_government`](../src/digital_government.rs): `ServiceAssessment::passes`,
`prorate_annual_value`, `failed_assessment_cost`, `gross_channel_shift_saving`, `fte_released`,
`blended_cost_per_transaction`, `platform_adoption_saving`, `cost_avoided_open_data_value`,
`annualized_loss_expectancy`, `security_control_value`, `handling_time_cost`, and `net_ai_value`.

## Societal indicators

[`societal_indicators`](../src/societal_indicators.rs): `genuine_progress_indicator`,
`gnh_sufficiency`, `human_development_index`, `mpi_headcount_ratio`,
`multidimensional_poverty_index`, `imd_domain_contribution`, `imd_composite_score`, `imd_decile`,
`social_capital_pillar_gap`, `ecosystem_asset_value`, `present_value_across_schedule`,
`gini_coefficient`, `atkinson_inequality_measure`, `inequality_adjusted_dimension_index`,
`inequality_adjusted_hdi`, and `ihdi_loss_percentage`.

## Delivery connection

[`delivery_connection`](../src/delivery_connection.rs): `cost_of_delay_per_week`,
`total_delay_loss`, `cycle_time`, `flow_efficiency`, `technical_debt_principal`,
`technical_debt_ratio`, `interest_reduction`, `payback_period_years`, `total_cost_of_ownership`,
`risk_adjusted_build_cost`, `buy_total_cost`, `realization_rate`, `eligible_developers`,
`ai_capacity_value`, `net_capacity_ratio`, `lead_time_value_pulled_forward`,
`fewer_failed_changes`, `incident_avoidance_saving`, and `delay_wellby_loss`.

## Shared units

[`units`](../src/units.rs): `money_ratio`, `Ratio::{new,value}`, and
`Percentage::{from_fraction,from_percent,as_fraction,as_percent}`.

Example: `cost_per_outcome(Money::from_decimal(dec!(450_000), iso::USD), 630)` returns `$714.29`;
`sroi_ratio($359,070, $250,000)` returns `1.44`; and
`cost_of_delay_per_week($520,000)` returns `$10,000.00`.
