//! Shared numeric newtypes used across every public value calculation.
//!
//! Plain primitives everywhere would let a ratio, a percentage, and a money amount be swapped by
//! accident at a call site. [`Money`], [`Ratio`], and [`Percentage`] exist to make that class of
//! mistake a type error instead of a silent bug.
//!
//! [`Money`] wraps [`rusty_money::Money`] rather than a bare `f64` or `Decimal`: monetary figures
//! in the source material are exact decimal amounts (£450,000, £8,500 per person), and repeated
//! addition, subtraction, and percentage scaling of `f64` money accumulates binary
//! floating-point rounding error that has no business appearing in a cost-benefit case.
//! `rusty_money::Money` requires every amount to carry a [`rusty_money::iso::Currency`]; this
//! crate fixes that currency to `USD` for every value it constructs, as a single internal working
//! currency, so that two `Money` values built anywhere in the crate can always be added or
//! compared without a runtime currency-mismatch error. Several worked examples ported from the
//! source material are stated in GBP (`public-value-metrics` is UK-focused); the doc comments and
//! prose still cite the real pound figures HM Treasury and others publish, but the `Money` values
//! those doctests construct are tagged `USD` like every other value in the crate — the tag is a
//! `rusty_money` implementation requirement, not a currency-conversion claim. [`Ratio`] and
//! [`Percentage`] stay `f64` because several formulas need transcendental functions (natural log,
//! roots) that are naturally lossy anyway, and because a ratio or percentage is a reporting
//! figure, not an amount that gets added to other amounts.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rusty_money::{Money as RustyMoney, iso};

/// A currency amount, backed by [`rusty_money::Money`] fixed to `USD` as this crate's single
/// internal working currency (see the module docs) so repeated addition and percentage scaling
/// stay exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Money(RustyMoney<'static, iso::Currency>);

impl Money {
    /// Creates a new money amount from an exact decimal value.
    ///
    /// # Examples
    ///
    /// ```
    /// use public_value::units::Money;
    /// use rust_decimal_macros::dec;
    ///
    /// let cost = Money::new(dec!(450_000));
    /// assert_eq!(cost.value(), dec!(450_000));
    /// ```
    #[must_use]
    pub fn new(amount: Decimal) -> Self {
        Self(RustyMoney::from_decimal(amount, iso::USD))
    }

    /// Returns the underlying decimal amount.
    #[must_use]
    pub fn value(self) -> Decimal {
        *self.0.amount()
    }

    /// Returns the dimensionless ratio of this amount to `other` (`self / other`).
    ///
    /// # Panics
    ///
    /// Panics if `other` is zero, or if the resulting ratio cannot be represented as an `f64`.
    ///
    /// # Examples
    ///
    /// ```
    /// use public_value::units::Money;
    /// use rust_decimal_macros::dec;
    ///
    /// let outcomes = Money::new(dec!(359_070));
    /// let inputs = Money::new(dec!(250_000));
    /// let sroi = outcomes.ratio_to(inputs);
    /// assert!((sroi.value() - 1.436_28).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn ratio_to(self, other: Self) -> Ratio {
        let ratio = self.value() / other.value();
        Ratio::new(ratio.to_f64().expect("decimal ratio fits in f64"))
    }
}

/// A currency mismatch here would mean this crate constructed a `Money` in a currency other than
/// its single internal working currency (`USD`), which is a bug in the crate, not a runtime
/// condition callers need to plan for.
const CURRENCY_INVARIANT: &str = "public-value::units::Money always uses one internal working currency (USD)";

impl Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0.add(rhs.0).expect(CURRENCY_INVARIANT))
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0.sub(rhs.0).expect(CURRENCY_INVARIANT))
    }
}

impl Mul<Decimal> for Money {
    type Output = Self;

    fn mul(self, scale: Decimal) -> Self {
        Self(self.0.mul(scale).expect("multiplication overflow"))
    }
}

impl Div<Decimal> for Money {
    type Output = Self;

    /// # Panics
    ///
    /// Panics if `scale` is zero.
    fn div(self, scale: Decimal) -> Self {
        Self(self.0.div(scale).expect("division by zero or overflow"))
    }
}

impl Mul<u32> for Money {
    type Output = Self;

    fn mul(self, count: u32) -> Self {
        Self(self.0.mul(Decimal::from(count)).expect("multiplication overflow"))
    }
}

impl Div<u32> for Money {
    type Output = Self;

    /// # Panics
    ///
    /// Panics if `count` is zero.
    fn div(self, count: u32) -> Self {
        Self(self.0.div(Decimal::from(count)).expect("division by zero or overflow"))
    }
}

impl PartialOrd for Money {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Money {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.compare(&other.0).expect(CURRENCY_INVARIANT)
    }
}

impl fmt::Display for Money {
    /// Delegates to `rusty_money::Money`'s own currency-aware, locale-aware formatting, which
    /// rounds to the currency's exponent using half-to-even rounding (not truncation).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
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
    use super::{Money, Percentage, Ratio};
    use rust_decimal_macros::dec;

    #[test]
    fn money_arithmetic() {
        let a = Money::new(dec!(100));
        let b = Money::new(dec!(40));
        assert_eq!((a + b).value(), dec!(140));
        assert_eq!((a - b).value(), dec!(60));
        assert_eq!((a * dec!(2)).value(), dec!(200));
        assert_eq!((a / dec!(2)).value(), dec!(50));
        assert_eq!((a * 2_u32).value(), dec!(200));
        assert_eq!((a / 2_u32).value(), dec!(50));
    }

    #[test]
    fn money_ratio_to() {
        let outcomes = Money::new(dec!(510_000));
        let inputs = Money::new(dec!(250_000));
        let ratio: Ratio = outcomes.ratio_to(inputs);
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
