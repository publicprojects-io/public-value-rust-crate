//! Representative Criterion benchmarks for each public-value category.
// The Criterion macros generate public harness symbols that are not library API.
#![allow(missing_docs)]

use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use public_value::delivery_connection::cost_of_delay_per_week;
use public_value::digital_government::annualized_loss_expectancy;
use public_value::economic_appraisal::annuity_factor;
use public_value::foundations::difference_in_differences;
use public_value::impact_measurement::net_outcome_uplift;
use public_value::performance_metrics::kpi_percentage;
use public_value::philanthropy_metrics::cost_per_outcome;
use public_value::societal_indicators::human_development_index;
use public_value::units::{Money, Percentage, iso, money_ratio};
use rust_decimal_macros::dec;

fn public_api(c: &mut Criterion) {
    let money = Money::from_decimal(dec!(450_000), iso::USD);
    let smaller_money = Money::from_decimal(dec!(250_000), iso::USD);

    c.bench_function("foundations/difference_in_differences", |b| {
        b.iter(|| difference_in_differences(black_box(70.0), 55.0, 61.0, 52.0));
    });
    c.bench_function("economic_appraisal/annuity_factor", |b| {
        b.iter(|| annuity_factor(black_box(dec!(0.035)), 20));
    });
    c.bench_function("impact_measurement/net_outcome_uplift", |b| {
        b.iter(|| {
            net_outcome_uplift(
                black_box(Percentage::from_percent(46.0)),
                Percentage::from_percent(33.0),
            )
        });
    });
    c.bench_function("performance_metrics/kpi_percentage", |b| {
        b.iter(|| kpi_percentage(black_box(870), 1_000));
    });
    c.bench_function("philanthropy_metrics/cost_per_outcome", |b| {
        b.iter(|| cost_per_outcome(black_box(money), 630));
    });
    c.bench_function("digital_government/annualized_loss_expectancy", |b| {
        b.iter(|| annualized_loss_expectancy(black_box(money), Percentage::from_percent(2.0)));
    });
    c.bench_function("societal_indicators/human_development_index", |b| {
        b.iter(|| human_development_index(black_box(72.0), 8.0, 13.0, 12_000.0));
    });
    c.bench_function("delivery_connection/cost_of_delay_per_week", |b| {
        b.iter(|| cost_of_delay_per_week(black_box(money)));
    });
    c.bench_function("units/money_ratio", |b| {
        b.iter(|| money_ratio(black_box(money), smaller_money));
    });
}

criterion_group!(benches, public_api);
criterion_main!(benches);
