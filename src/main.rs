use core::f64;
use std::{f64::consts::PI, time::Duration};

use num_format::{Locale, ToFormattedString};
use simple_logger::SimpleLogger;

const LOCALE: Locale = Locale::de;

fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Computing pi with the Georgy Leibniz Series...");
    time(georgy_leibniz_series, 100000000, 200000000);

    log::info!("Computing pi with the Nilakantha Series...");
    time(nilakantha_series, 100000000, 200000000);
}

/// Nilakantha Series
///
/// $$\pi=3+\frac{4}{2*3*4}-\frac{4}{4*5*6}+\frac{4}{5*6*7}-\ldots$$
///
/// It took 997.002421ms (156 257 628 iterations) to compute 15 digits correct.
fn nilakantha_series(n: u64) -> f64 {
    let mut pi = 3.0;
    for i in 0..n {
        let x = 2.0 * i as f64;
        let value = 4.0 / ((x + 2.0) * (x + 3.0) * (x + 4.0));
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        pi += value * sign;
    }
    pi
}
/// Georgy Leibniz series
///
/// $$\frac{\pi}{4}=1-\frac{1}{3}+\frac{1}{5}-\frac{1}{7}+\ldots$$
///
/// It took 998.99085ms (164 658 389 iterations) to compute 10 digits correct.
fn georgy_leibniz_series(n: u64) -> f64 {
    let mut pi_fourths = 0.0;
    for i in 0..n {
        let value = 1.0 / (2 * i + 1) as f64;
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        pi_fourths += value * sign;
    }
    let pi = pi_fourths * 4.0;
    pi
}

fn time(approximator: impl Fn(u64) -> f64, i_min: u64, i_max: u64) {
    let mut lower_bound = i_min;
    let mut upper_bound = i_max;
    let mut calculated = 0.0;
    let mut dt = Duration::from_secs(0);
    loop {
        let i = (lower_bound + upper_bound) / 2;
        let t0 = std::time::Instant::now();
        calculated = approximator(i);
        let t1 = std::time::Instant::now();
        dt = t1 - t0;

        if dt < Duration::from_secs(1) {
            lower_bound = i;
            if lower_bound >= upper_bound - 1000 {
                break;
            }
        } else {
            upper_bound = i;
        }
    }

    let correct_n = accurate_to_digit(PI, calculated);

    log::info!(
        "It took {:?} ({} iterations) to compute {} digits correct with a delta of {} - {} = {} ({} %)",
        dt,
        lower_bound.to_formatted_string(&LOCALE),
        correct_n.to_formatted_string(&LOCALE),
        PI,
        calculated,
        PI - calculated,
        (PI - calculated) / PI * 100.0
    );
}

fn accurate_to_digit(a: f64, b: f64) -> u8 {
    let a_string = a.to_string();
    let mut a_chars = a_string.chars();

    let b_string = b.to_string();
    let mut b_chars = b_string.chars();

    let mut n = 0;
    loop {
        let a_char = a_chars.next().unwrap();
        let b_char = b_chars.next().unwrap();

        if a_char != b_char {
            break;
        } else {
            n += 1;
        }
    }
    n
}
