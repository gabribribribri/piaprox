use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::{thread, time::Instant};

use crate::utils;

pub fn run(max: u64, jobs: u64) {
    let timer = Instant::now();

    let mut job_handles = Vec::new();
    for offset in 1..=jobs {
        job_handles.push(thread::spawn(move || {
            let mut sum_iters = Decimal::ZERO;
            let mut n = offset;

            while n < max {
                sum_iters += Decimal::from(4 - (n as i64 % 2) * 8) / Decimal::from(2 * n + 1);
                n += jobs;
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
