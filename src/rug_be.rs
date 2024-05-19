const PRECISION: u32 = 512;

use std::{thread, time::Instant};

use rug::{
    ops::{AddFrom, DivFrom, MulFrom},
    Float,
};

use crate::utils;

pub fn run(max: usize, jobs: usize) {
    let timer = Instant::now();
    let jobs_u32 = jobs as u32;
    let max_u32 = max as u32;

    let mut job_handles = Vec::new();
    for offset_u32 in 1..=jobs_u32 {
        job_handles.push(thread::spawn(move || {
            let mut sum_iters = Float::with_val(PRECISION, 0);
            let mut n = offset_u32;
            while n < max_u32 {
                let up = if n % 2 == 0 {
                    Float::with_val(PRECISION, 4)
                } else {
                    Float::with_val(PRECISION, -4)
                };
                let mut down = Float::with_val(PRECISION, n);
                down.mul_from(2);
                down.add_from(1);
                down.div_from(up);
                sum_iters.add_from(down);
                n += jobs_u32;
            }
            sum_iters
        }))
    }

    let mut piaprox = Float::with_val(PRECISION, 4);
    for job in job_handles {
        let result = job.join().unwrap();
        piaprox.add_from(result);
    }

    let time = timer.elapsed();
    utils::result_message("rug", max, jobs, time, piaprox);
}
