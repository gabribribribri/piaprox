use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::{thread, time::Instant};

use crate::utils;

pub fn run(max: u64, jobs: u64) {
    let timer = Instant::now();
    let djobs = Decimal::new(jobs as i64, 0);
    let dmax = Decimal::new(max as i64, 0);

    let mut job_handles = Vec::new();
    for offset in 1..=jobs {
        job_handles.push(thread::spawn(move || {
            let mut sum_iters = Decimal::ZERO;
            let mut n = Decimal::new(offset as i64, 0);

            while n < dmax {
                sum_iters += (dec!(4) - (n % dec!(2)) * dec!(8)) / (dec!(2) * n + dec!(1));
                n += djobs;
            }
            return sum_iters;
        }))
    }

    let piaprox = job_handles
        .into_iter()
        .map(|j| j.join().unwrap())
        .sum::<Decimal>()
        + dec!(4);

    let time = timer.elapsed();
    utils::result_message("rust_decimal", max, jobs, time, piaprox);
}
