use std::{thread, time::Instant};

use rug::{
    ops::{AddFrom, DivFrom, MulFrom},
    Float,
};

use crate::utils;

pub fn run(iterations: u32, jobs: u32, precision: u32) {
    println!("Precision: {}", precision);
    let timer = Instant::now();

    let mut job_handles = Vec::new();
    for offset_u32 in 1..=jobs {
        job_handles.push(thread::spawn(move || {
            let mut sum_iters = Float::with_val(precision, 0);
            let mut n = offset_u32;
            while n < iterations {
                let up = if n % 2 == 0 {
                    Float::with_val(precision, 4)
                } else {
                    Float::with_val(precision, -4)
                };
                let mut down = Float::with_val(precision, n);
                down.mul_from(2);
                down.add_from(1);
                down.div_from(up);
                sum_iters.add_from(down);
                n += jobs;
            }
            sum_iters
        }))
    }

    let mut piaprox = Float::with_val(precision, 4);
    for job in job_handles {
        let result = job.join().unwrap();
        piaprox.add_from(result);
    }

    let time = timer.elapsed();
    utils::result_message("rug", iterations, jobs, time, piaprox);
}
