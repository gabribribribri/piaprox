use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::{thread, time::Instant};

use crate::utils;

pub fn run(max: usize, jobs: usize) {
    let timer = Instant::now();
    let djobs = Decimal::new(jobs as i64, 0);
    let dmax = Decimal::new(max as i64, 0);

    let mut job_handles = Vec::new();
    for offset in 1..=jobs {
        job_handles.push(thread::spawn(move || {
            let mut sum_iters = Decimal::ZERO;
            let mut n = Decimal::new(offset as i64, 0);

            while n < dmax {
                if n.clone() % dec!(1_000_000) == Decimal::ZERO {
                    println!("[THREAD {}]: {}th iteration", offset, n)
                }

                sum_iters += (Decimal::ONE - (n % Decimal::TWO) * Decimal::TWO)
                    / (Decimal::TWO * n + Decimal::ONE);
                n += djobs;
            }
            return sum_iters;
        }))
    }

    let piaprox = (job_handles
        .into_iter()
        .map(|j| j.join().unwrap())
        .sum::<Decimal>()
        + Decimal::ONE)
        * dec!(4);

    let time = timer.elapsed();
    utils::result_message("rust_decimal", max, jobs, time, piaprox);
}
