//! Shared numeric types used across every public value calculation.
//!
//! Plain primitives everywhere would let a ratio, a percentage, and a money amount be swapped by
//! accident at a call site. [`Money`], [`Ratio`], and [`Percentage`] exist to make that class of
//! mistake a type error instead of a silent bug.
//!
//! [`Money`] is [`rusty_money::Money`] used directly — not wrapped in a crate-local newtype —
//! fixed to the [`iso::Currency`] `USD` via the [`Money`] type alias below, so callers write
//! `Money::from_decimal(dec!(450_000), iso::USD)` rather than the fully generic
//! `rusty_money::Money<'static, iso::Currency>`. `rusty_money::Money` requires every value to
//! carry a `Currency`; every construction in this crate uses `iso::USD`, as a single internal
//! working currency, so any two `Money` values built anywhere in the crate can always be combined.
//! Because `rusty_money::Money` exposes arithmetic and ordering as fallible, `Result`-returning
//! methods (`add`, `sub`, `mul`, `div`, `compare`) rather than `std::ops` operators, every
//! calculation module calls those methods directly and `.expect(CURRENCY_INVARIANT)`s the result
//! — documented on each call site as `# Panics` — instead of using `+`/`-`/`*`/`/`/`<`/`>`.
//!
//! Several worked examples ported from the source material are stated in GBP
//! (`public-value-metrics` is UK-focused); the doc comments and prose still cite the real pound
//! figures HM Treasury and others publish, but the `Money` values those doctests construct are
//! tagged `USD` like every other value in the crate — the tag is a `rusty_money` implementation
//! requirement, not a currency-conversion claim.
//!
//! [`Ratio`] and [`Percentage`] stay `f64` because several formulas need transcendental functions
//! (natural log, roots) that are naturally lossy anyway, and because a ratio or percentage is a
//! reporting figure, not an amount that gets added to other amounts.

use std::fmt;

use rust_decimal::prelude::ToPrimitive;
pub use rusty_money::iso;

/// A currency amount: [`rusty_money::Money`] fixed to `USD` as this crate's single internal
/// working currency (see the module docs).
pub type Money = rusty_money::Money<'static, iso::Currency>;

/// The message every `.expect()` on a `Money` arithmetic or comparison result carries in this
/// crate. A currency mismatch here would mean the crate constructed a `Money` in a currency other
/// than its single internal working currency (`USD`), which is a bug in the crate, not a runtime
/// condition callers need to plan for.
pub const CURRENCY_INVARIANT: &str = "public-value::units::Money always uses one internal working currency (USD)";

/// The dimensionless ratio of `numerator` to `denominator` (`numerator / denominator`).
///
/// Ported nowhere in particular — this is the shared implementation behind every topic that
/// reports a ratio of two `Money` amounts (e.g. an SROI ratio, a benefit-cost ratio).
///
/// # Panics
///
/// Panics if `denominator` is zero, or if the resulting ratio cannot be represented as an `f64`.
///
/// # Examples
///
/// ```
/// use public_value::units::{iso, money_ratio, Money};
/// use rust_decimal_macros::dec;
///
/// let outcomes = Money::from_decimal(dec!(359_070), iso::USD);
/// let inputs = Money::from_decimal(dec!(250_000), iso::USD);
/// let sroi = money_ratio(outcomes, inputs);
/// assert!((sroi.value() - 1.436_28).abs() < 0.001);
/// ```
#[must_use]
pub fn money_ratio(numerator: Money, denominator: Money) -> Ratio {
    let ratio = *numerator.amount() / *denominator.amount();
    Ratio::new(ratio.to_f64().expect("decimal ratio fits in f64"))
}

/// A dimensionless ratio, such as a social-return-on-investment ratio or a cost-effectiveness
/// ratio.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ratio(f64);

impl Ratio {
    /// Creates a new ratio.
    #[must_use]
    pub const fn new(value: f64) -> Self {
        Self(value)
    }

    /// Returns the underlying value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.0
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

/// A percentage, stored internally as a fraction (`1.0` means 100%).
///
/// Not clamped to `0.0..=1.0`: several source topics (e.g. change failure rate, discount rates
/// applied over many years) describe figures that are legitimately negative or that exceed 100%.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage(f64);

impl Percentage {
    /// Creates a percentage from a fraction (`0.5` means 50%).
    #[must_use]
    pub const fn from_fraction(fraction: f64) -> Self {
        Self(fraction)
    }

    /// Creates a percentage from a `0.0..=100.0`-scaled value (`50.0` means 50%).
    #[must_use]
    pub fn from_percent(percent: f64) -> Self {
        Self(percent / 100.0)
    }

    /// Returns the value as a fraction (`0.5` for 50%).
    #[must_use]
    pub const fn as_fraction(self) -> f64 {
        self.0
    }

    /// Returns the value on a `0.0..=100.0` scale (`50.0` for 50%).
    #[must_use]
    pub fn as_percent(self) -> f64 {
        self.0 * 100.0
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.as_percent())
    }
}

#[cfg(test)]
mod tests {
    use super::{Percentage, Ratio, iso, money_ratio, Money};
    use rust_decimal_macros::dec;

    #[test]
    fn money_arithmetic() {
        let a = Money::from_decimal(dec!(100), iso::USD);
        let b = Money::from_decimal(dec!(40), iso::USD);
        assert_eq!(*a.add(b).unwrap().amount(), dec!(140));
        assert_eq!(*a.sub(b).unwrap().amount(), dec!(60));
        assert_eq!(*a.mul(dec!(2)).unwrap().amount(), dec!(200));
        assert_eq!(*a.div(dec!(2)).unwrap().amount(), dec!(50));
        assert_eq!(*a.mul(2_u32).unwrap().amount(), dec!(200));
        assert_eq!(*a.div(2_u32).unwrap().amount(), dec!(50));
    }

    #[test]
    fn money_ratio_computation() {
        let outcomes = Money::from_decimal(dec!(510_000), iso::USD);
        let inputs = Money::from_decimal(dec!(250_000), iso::USD);
        let ratio: Ratio = money_ratio(outcomes, inputs);
        assert!((ratio.value() - 2.04).abs() < 0.001);
    }

    #[test]
    fn percentage_round_trip() {
        let p = Percentage::from_percent(42.5);
        assert!((p.as_fraction() - 0.425).abs() < f64::EPSILON);
        assert!((p.as_percent() - 42.5).abs() < f64::EPSILON);
        assert_eq!(format!("{p}"), "42.5%");
    }
}
