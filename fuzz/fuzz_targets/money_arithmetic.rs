#![no_main]

use libfuzzer_sys::fuzz_target;
use public_value::units::{Money, iso};
use rust_decimal::Decimal;

fuzz_target!(|data: (i64, i64, u32)| {
    let left = Money::from_decimal(Decimal::from(data.0), iso::USD);
    let right = Money::from_decimal(Decimal::from(data.1), iso::USD);
    let factor = Decimal::from(data.2);

    let _ = left.add(right);
    let _ = left.sub(right);
    let _ = left.mul(factor);
});
