use bigdecimal::{BigDecimal, Zero};
use std::{thread, time::Instant};

use crate::utils;

pub fn run(iterations: u64, jobs: u64) {
    let timer = Instant::now();

    let mut job_handles = Vec::new();
    for offset in 1..=jobs {
        job_handles.push(thread::spawn(move || {
            let mut sum_iters = BigDecimal::zero();
            let mut n = offset;

            while n < iterations {
                sum_iters += BigDecimal::from(4 - (n as i64 % 2) * 8) / BigDecimal::from(2 * n + 1);
                n += jobs;
            }
            sum_iters
        }))
    }
    let piaprox = job_handles
        .into_iter()
        .map(|j| j.join().unwrap())
        .sum::<BigDecimal>()
        + BigDecimal::from(4);

    let time = timer.elapsed();
    utils::result_message(
        "bigdecimal",
        piaprox,
        Some(iterations),
        Some(jobs),
        Some(time),
        None,
    )
}
