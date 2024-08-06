
/// Georgy Leibniz series
///
/// $$\frac{\pi}{4}=1-\frac{1}{3}+\frac{1}{5}-\frac{1}{7}+\ldots$$
///
/// It took 998.99085ms (164 658 389 iterations) to compute 10 digits correct.
pub fn georgy_leibniz_series(n: u64) -> f64 {
    let mut pi_fourths = 0.0;
    for i in 0..n {
        let value = 1.0 / (2 * i + 1) as f64;
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        pi_fourths += value * sign;
    }
    let pi = pi_fourths * 4.0;
    pi
}
