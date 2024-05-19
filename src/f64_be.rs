use std::{thread, time::Instant};

use crate::utils;

pub fn run(iterations: u32, jobs: u32) {
    let timer = Instant::now();
    let jobs_f64 = jobs as f64;
    let iterations_f64 = iterations as f64;

    let mut job_handles = Vec::new();
    for offset in 1..=jobs {
        job_handles.push(thread::spawn(move || {
            let mut sum_iters = 0f64;
            let mut n = offset as f64;

            while n < iterations_f64 {
                // if n % 1_000_000 == 0 {
                //     println!("[THREAD {}]: {}th iteration", offset, n);
                // }

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
    utils::result_message("f64", iterations, jobs, time, piaprox);
}
