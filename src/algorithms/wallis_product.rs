use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub fn wallis_product(n: u64) -> Decimal {
    let mut half_pi = dec!(1.0);
    for i in 1..n {
        let two_i = Decimal::from(2 *i);

        let value = (two_i / (two_i - dec!(1.0))) * (two_i / (two_i + dec!(1.0)));
        half_pi *= value;
    }
    dec!(2.0) * half_pi
}
