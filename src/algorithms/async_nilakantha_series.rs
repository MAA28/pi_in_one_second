use futures::{executor::block_on, future};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Nilakantha Series
///
/// $$\pi=3+\frac{4}{2*3*4}-\frac{4}{4*5*6}+\frac{4}{5*6*7}-\ldots$$
///
/// It took 997.002421ms (156 257 628 iterations) to compute 15 digits correct.
pub fn async_nilakantha_series(n: u64) -> Decimal {
    let pi = block_on(calculate(n));
    pi
}

async fn calculate(n: u64) -> Decimal {
    let futures = (0..n)
        .map(|i| async move {
            let x = Decimal::from(2 * i);
            let value = dec!(4.0) / ((x + dec!(2.0)) * (x + dec!(3.0)) * (x + dec!(4.0)));
            let sign = if i % 2 == 0 { dec!(1.0) } else { dec!(-1.0) };
            value * sign
        })
        .collect::<Vec<_>>();
    let pi = dec!(3.0) + future::join_all(futures).await.iter().sum::<Decimal>();
    pi
}
