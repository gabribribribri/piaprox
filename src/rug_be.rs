use std::{thread, time::Instant};

use rug::{
    ops::{AddFrom, DivFrom},
    Float,
};

use crate::aprox::{Aprox, Backend};

pub trait RugBackend {
    fn nk_run(&mut self);
    fn gl_run(&mut self);
}

impl RugBackend for Aprox {
    fn gl_run(&mut self) {
        let timer = Instant::now();

        let iterations = self.iterations;
        let jobs = self.jobs;
        let precision = self.precision;

        let mut job_handles = Vec::new();
        for offset in 1..=self.jobs {
            job_handles.push(thread::spawn(move || {
                let mut sum_iters = Float::with_val(precision, 0);
                let mut n = offset;
                while n < iterations {
                    let mut iter = Float::with_val(precision, 2 * n + 1);
                    iter.div_from(4 - (n as i64 % 2) * 8);
                    sum_iters.add_from(iter);
                    n += jobs;
                }
                sum_iters
            }))
        }

        let mut piaprox = Float::with_val(self.precision, 4);
        for job in job_handles {
            let result = job.join().unwrap();
            piaprox.add_from(result);
        }
        self.backend = Backend::Rug(Some(piaprox));

        self.time = timer.elapsed();
    }

    fn nk_run(&mut self) {
        todo!()
    }
}
// pub fn run(iterations: u64, jobs: u64, precision: u32) {
//     let timer = Instant::now();

//     let mut job_handles = Vec::new();
//     for offset in 1..=jobs {
//         job_handles.push(thread::spawn(move || {
//             let mut sum_iters = Float::with_val(precision, 0);
//             let mut n = offset;
//             while n < iterations {
//                 let mut iter = Float::with_val(precision, 2 * n + 1);
//                 iter.div_from(4 - (n as i64 % 2) * 8);
//                 sum_iters.add_from(iter);
//                 n += jobs;
//             }
//             sum_iters
//         }))
//     }

//     let mut piaprox = Float::with_val(precision, 4);
//     for job in job_handles {
//         let result = job.join().unwrap();
//         piaprox.add_from(result);
//     }

//     let time = timer.elapsed();
//     // utils::result_message("rug", iterations, jobs, time, piaprox);
//     utils::result_message(
//         "rug",
//         piaprox,
//         Some(iterations),
//         Some(jobs),
//         Some(time),
//         Some(precision),
//     )
// }
