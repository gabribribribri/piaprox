use std::{thread, time::Instant};

use crate::utils;

pub fn run(iterations: u64, jobs: u64) {
    let timer = Instant::now();
    let jobs_f64 = jobs as f64;
    let iterations_f64 = iterations as f64;

    let mut job_handles = Vec::new();
    for offset in 1..=jobs {
        job_handles.push(thread::spawn(move || {
            let mut sum_iters = 0f64;
            let mut n = offset as f64;

            while n < iterations_f64 {
                sum_iters += (4f64 - (n % 2f64) * 8f64) / (2f64 * n + 1f64);
                n += jobs_f64;
            }
            sum_iters
        }))
    }

    let piaprox = job_handles
        .into_iter()
        .map(|j| j.join().unwrap())
        .sum::<f64>()
        + 4f64;

    let time = timer.elapsed();
    utils::result_message(
        "f64",
        piaprox,
        Some(iterations),
        Some(jobs),
        Some(time),
        None,
    )
}
