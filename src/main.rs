//! Demo binary: exercises a couple of the library's public value calculations.

use public_value::philanthropy_metrics::cost_per_outcome;
use public_value::units::Money;
use rust_decimal_macros::dec;

/// Prints a worked example from the library.
fn main() {
    let cost = cost_per_outcome(Money::new(dec!(450_000)), 630);
    println!("Cost per outcome: {cost}");
}
