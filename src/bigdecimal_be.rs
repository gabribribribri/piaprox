use bigdecimal::{BigDecimal, Zero};
use std::{thread, time::Instant};

use crate::utils;

pub fn run(max: usize, jobs: usize) {
    let timer = Instant::now();

    let mut job_handles = Vec::new();
    for offset in 1..=jobs {
        let dmax = BigDecimal::from(max as u64);
        let djobs = BigDecimal::from(jobs as u64);

        job_handles.push(thread::spawn(move || {
            let mut sum_iters = BigDecimal::zero();
            let mut n = BigDecimal::from(offset as u64);

            while n < dmax {
                // if n.clone() % BigDecimal::from(1_000_000) == BigDecimal::zero() {
                //     println!("[THREAD {}]: {}th iteration", offset, n)
                // }

                sum_iters += (BigDecimal::from(1)
                    - (n.clone() % BigDecimal::from(2)) * BigDecimal::from(2))
                    / (BigDecimal::from(2) * n.clone() + BigDecimal::from(1));

                n += djobs.clone();
            }
            sum_iters
        }))
    }

    let piaprox = (job_handles
        .into_iter()
        .map(|j| j.join().unwrap())
        .sum::<BigDecimal>()
        + BigDecimal::from(1))
        * BigDecimal::from(4);

    let time = timer.elapsed();
    utils::result_message("rust_decimal", max, jobs, time, piaprox);
}
