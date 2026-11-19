//! A runnable tour with one calculation from every category module.

use public_value::delivery_connection::cost_of_delay_per_week;
use public_value::digital_government::annualized_loss_expectancy;
use public_value::economic_appraisal::qaly;
use public_value::foundations::net_additional_outcomes;
use public_value::impact_measurement::net_outcome_uplift;
use public_value::performance_metrics::kpi_percentage;
use public_value::philanthropy_metrics::cost_per_outcome;
use public_value::societal_indicators::multidimensional_poverty_index;
use public_value::units::{Money, Percentage, iso};
use rust_decimal_macros::dec;

fn main() {
    let programme = Money::from_decimal(dec!(450_000), iso::USD);
    println!(
        "Foundations — additional outcomes: {}",
        net_additional_outcomes(1_000, Percentage::from_percent(30.0))
    );
    println!("Economic appraisal — QALYs: {}", qaly(5.0, 0.7));
    println!(
        "Impact measurement — uplift: {}",
        net_outcome_uplift(
            Percentage::from_percent(46.0),
            Percentage::from_percent(33.0)
        )
    );
    println!("Performance — KPI: {}", kpi_percentage(87, 100));
    println!(
        "Philanthropy — cost/outcome: {}",
        cost_per_outcome(programme, 630)
    );
    println!(
        "Digital government — annualized loss: {}",
        annualized_loss_expectancy(programme, Percentage::from_percent(2.0))
    );
    println!(
        "Societal indicators — MPI: {}",
        multidimensional_poverty_index(
            Percentage::from_percent(40.0),
            Percentage::from_percent(50.0)
        )
    );
    println!(
        "Delivery — weekly delay cost: {}",
        cost_of_delay_per_week(Money::from_decimal(dec!(520_000), iso::USD))
    );
}
