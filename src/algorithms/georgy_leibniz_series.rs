use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Georgy Leibniz series
///
/// $$\frac{\pi}{4}=1-\frac{1}{3}+\frac{1}{5}-\frac{1}{7}+\ldots$$
///
/// It took 998.99085ms (164 658 389 iterations) to compute 10 digits correct.
pub fn georgy_leibniz_series(n: u64) -> Decimal {
    let mut pi_fourths = dec!(0.0);
    for i in 0..n {
        let value = dec!(1.0) / Decimal::from(2 * i + 1);
        let sign = if i % 2 == 0 { dec!(1.0 )} else { dec!(-1.0) };
        pi_fourths += value * sign;
    }
    let pi = pi_fourths * dec!(4.0);
    pi
}
