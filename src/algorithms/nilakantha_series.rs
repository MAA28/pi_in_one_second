/// Nilakantha Series
///
/// $$\pi=3+\frac{4}{2*3*4}-\frac{4}{4*5*6}+\frac{4}{5*6*7}-\ldots$$
///
/// It took 997.002421ms (156 257 628 iterations) to compute 15 digits correct.
pub fn nilakantha_series(n: u64) -> f64 {
    let mut pi = 3.0;
    for i in 0..n {
        let x = 2.0 * i as f64;
        let value = 4.0 / ((x + 2.0) * (x + 3.0) * (x + 4.0));
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        pi += value * sign;
    }
    pi
}
