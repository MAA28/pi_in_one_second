use std::time::{Duration, Instant};

use num_format::ToFormattedString;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::LOCALE;

const PI: Decimal = dec!(3.1415926535897932384626433832);

pub fn time(approximator: impl Fn(u64) -> Decimal, i_min: u64, i_max: u64) {
    let mut lower_bound = i_min;
    let mut upper_bound = i_max;
    let mut calculated = dec!(0.0);
    let mut dt = Duration::from_secs(0);
    loop {
        log::debug!("Range of iterations: {} - {}", lower_bound, upper_bound);
        let i = (lower_bound + upper_bound) / 2;
        let t0 = Instant::now();
        calculated = approximator(i);
        let t1 = Instant::now();
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
        (PI - calculated) / PI * dec!(100.0)
    );
}

fn accurate_to_digit(a: Decimal, b: Decimal) -> u8 {
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
