//! Demo binary: exercises calculations from across the library.

use public_value::delivery_connection::cost_of_delay_per_week;
use public_value::foundations::net_public_value;
use public_value::impact_measurement::sroi_ratio;
use public_value::performance_metrics::net_promoter_score;
use public_value::philanthropy_metrics::cost_per_outcome;
use public_value::societal_indicators::human_development_index;
use public_value::units::{Money, iso};
use rust_decimal_macros::dec;

/// Prints usage information or a set of worked examples.
fn main() {
    if std::env::args().any(|argument| argument == "--help" || argument == "-h") {
        println!(
            "public-value-demo\n\nUsage: public-value [--help]\n\nRuns six worked calculations from the public-value library."
        );
        return;
    }

    let cost = cost_per_outcome(Money::from_decimal(dec!(450_000), iso::USD), 630);
    println!("Cost per outcome: {cost}");

    let sroi = sroi_ratio(
        Money::from_decimal(dec!(359_070), iso::USD),
        Money::from_decimal(dec!(250_000), iso::USD),
    );
    println!("Social return on investment: {sroi}:1");

    let hdi = human_development_index(72.0, 8.0, 13.0, 12_000.0);
    println!("Human Development Index: {:.3}", hdi.hdi);

    let public_value = net_public_value(
        Money::from_decimal(dec!(900_000), iso::USD),
        Money::from_decimal(dec!(650_000), iso::USD),
    );
    println!("Net public value: {public_value}");

    let delay = cost_of_delay_per_week(Money::from_decimal(dec!(520_000), iso::USD));
    println!("Cost of delay: {delay}/week");

    let nps = net_promoter_score(62, 20, 100);
    println!("Net Promoter Score: {nps:.0}");
}
