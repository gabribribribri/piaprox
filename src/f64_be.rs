use std::{thread, time::Instant};

use crate::utils;

pub fn run(max: usize, jobs: usize) {
    let timer = Instant::now();
    let fjobs = jobs as f64;
    let fmax = max as f64;

    let mut job_handles = Vec::new();
    for offset in 1..=jobs {
        job_handles.push(thread::spawn(move || {
            let mut sum_iters = 0f64;
            let mut n = offset as f64;

            while n < fmax {
                // if n % 1_000_000 == 0 {
                //     println!("[THREAD {}]: {}th iteration", offset, n);
                // }

                sum_iters += (1f64 - (n % 2f64) * 2f64) / (2f64 * n + 1f64);
                n += fjobs;
            }
            sum_iters
        }))
    }

    let piaprox = (job_handles
        .into_iter()
        .map(|j| j.join().unwrap())
        .sum::<f64>()
        + 1f64)
        * 4f64;

    let time = timer.elapsed();
    utils::result_message("f64", max, jobs, time, piaprox);
}
