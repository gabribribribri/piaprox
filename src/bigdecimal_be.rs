use bigdecimal::{BigDecimal, Zero};
use std::{thread, time::Instant};

use crate::aprox::{Aprox, Backend};

pub trait BigDecimalBackend {
    fn gl_run(&mut self);
    fn nk_run(&mut self);
}

impl BigDecimalBackend for Aprox {
    fn gl_run(&mut self) {
        let timer = Instant::now();

        let iterations = self.iterations;
        let jobs = self.jobs;

        let mut job_handles = Vec::new();
        for offset in 1..=self.jobs {
            job_handles.push(thread::spawn(move || {
                let mut sum_iters = BigDecimal::zero();
                let mut n = offset;

                while n < iterations {
                    sum_iters +=
                        BigDecimal::from(4 - (n as i64 % 2) * 8) / BigDecimal::from(2 * n + 1);
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

        self.backend = Backend::BigDecimal(Some(piaprox));
        self.time = timer.elapsed();
    }

    fn nk_run(&mut self) {
        todo!()
    }
}

// pub fn run(iterations: u64, jobs: u64) -> Aprox<BigDecimal> {
//     let timer = Instant::now();

//     let mut job_handles = Vec::new();
//     for offset in 1..=jobs {
//         job_handles.push(thread::spawn(move || {
//             let mut sum_iters = BigDecimal::zero();
//             let mut n = offset;

//             while n < iterations {
//                 sum_iters += BigDecimal::from(4 - (n as i64 % 2) * 8) / BigDecimal::from(2 * n + 1);
//                 n += jobs;
//             }
//             sum_iters
//         }))
//     }
//     let piaprox = job_handles
//         .into_iter()
//         .map(|j| j.join().unwrap())
//         .sum::<BigDecimal>()
//         + BigDecimal::from(4);

//     let time = timer.elapsed();
//     return aprox::Aprox::new(aprox::Backend::BigDecimal, piaprox)
//         .set_iterations(iterations)
//         .set_jobs(jobs)
//         .set_time(time);
// }
