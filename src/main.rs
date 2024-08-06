
use num_format::{Locale};
use simple_logger::SimpleLogger;

mod utils;
mod algorithms;

use algorithms::{georgy_leibniz_series, nilakantha_series, wallis_product};
use utils::time;


const LOCALE: Locale = Locale::de;

fn main() {
    SimpleLogger::new().init().unwrap();

    log::debug!("{}", wallis_product(100));
    log::info!("Computing pi with the Wallis Product...");
    time(wallis_product, 100000000, 200000000);

    log::info!("Computing pi with the Georgy Leibniz Series...");
    time(georgy_leibniz_series, 100000000, 200000000);

    log::info!("Computing pi with the Nilakantha Series...");
    time(nilakantha_series, 100000000, 200000000);
}


