use std::f64::consts::PI;

use num_format::Locale;
use rust_decimal_macros::dec;
use simple_logger::SimpleLogger;

mod algorithms;
mod utils;

use algorithms::{
    async_nilakantha_series, georgy_leibniz_series, nilakantha_series, wallis_product,
};
use utils::time;

const LOCALE: Locale = Locale::de;

fn main() {
    SimpleLogger::new().init().unwrap();

    // let n = 2115745;
    // let t0 = std::time::Instant::now();
    // log::info!("Sync:  {}", nilakantha_series(n));
    // let t1 = std::time::Instant::now();
    // log::info!("Async: {}", async_nilakantha_series(n));
    // let t2 = std::time::Instant::now();
    // log::info!("Pi:    {}", PI);

    // log::info!("Time:  {:?} (Sync)", t1 - t0);
    // log::info!("Time:  {:?} (Async)", t2 - t1);

    log::info!("Computing pi with the Async Nilakantha Series...");

    // let t0 = std::time::Instant::now();
    // for _ in 0..10 {
        // async_nilakantha_series(952123);
    // }
    // let t1 = std::time::Instant::now();

    time(async_nilakantha_series, 902123, 992123);

    log::info!("Computing pi with the Wallis Product...");

    // let t2 = std::time::Instant::now();
    // for _ in 0..10 {
        // wallis_product(1069723);
    // }
    // let t3 = std::time::Instant::now();

    time(wallis_product, 869723, 1269723);

    log::info!("Computing pi with the Georgy Leibniz Series...");
    
    // let t4 = std::time::Instant::now();
    // for _ in 0..10 {
        // georgy_leibniz_series(4129072);
    // }
    // let t5 = std::time::Instant::now();

    time(georgy_leibniz_series, 4029072, 4529072);

    log::info!("Computing pi with the Nilakantha Series...");

    // let t6 = std::time::Instant::now();
    // for _ in 0..10 {
        // nilakantha_series(2011265);
    // }
    // let t7 = std::time::Instant::now();
    
    time(nilakantha_series, 1811265, 2311265);

    // log::info!("Time:  {:?} (Async)", (t1 - t0) / 10);
    // log::info!("Time:  {:?} (Wallis)", (t3 - t2) / 10);
    // log::info!("Time:  {:?} (Georgy)", (t5 - t4) / 10);
    // log::info!("Time:  {:?} (Nilakantha)", (t7 - t6) / 10);
}
