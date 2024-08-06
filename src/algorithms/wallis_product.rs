pub fn wallis_product(n: u64) -> f64 {
    let mut half_pi = 1.0;
    for i in 1..n {
        let two_i = 2.0 * i as f64;

        let value = (two_i / (two_i - 1.0)) * (two_i / (two_i + 1.0));
        half_pi *= value;
    }
    2.0 * half_pi
}
