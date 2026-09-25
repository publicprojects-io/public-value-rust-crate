//! Public value models, structs, calculations, and examples.
//!
//! This crate is a Rust port of the formulas, frameworks, and worked examples documented in
//! [`public-value-metrics`](https://github.com/public-value-metrics/public-value-metrics), a book on
//! public value math for software engineers building for governmental and social sector
//! organizations. See `spec/architecture.md` and `spec/topics.md` in the crate repository for the
//! module layout and the full topic-to-module mapping.
//!
//! Each module below corresponds to one category from that book; each public item corresponds to
//! one topic, and its rustdoc names the source topic it was ported from.

pub mod units;

pub mod delivery_connection;
pub mod digital_government;
pub mod economic_appraisal;
pub mod foundations;
pub mod impact_measurement;
pub mod performance_metrics;
pub mod philanthropy_metrics;
pub mod societal_indicators;
