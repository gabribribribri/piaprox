use std::thread;

use rug::{
    ops::{AddFrom, DivFrom},
    Float,
};

use crate::aprox::{Aprox, Backend};

pub trait RugBackend {
    fn gl_run(&mut self);
    fn nk_run(&mut self);
}

impl RugBackend for Aprox {
    fn gl_run(&mut self) {
        let iterations = self.iterations;
        let jobs = self.jobs;
        let precision = self.precision;

        let mut job_handles = Vec::new();
        for offset in 1..=self.jobs {
            job_handles.push(thread::spawn(move || {
                let mut sum_iters = Float::with_val(precision, 0);
                for n in (offset..iterations).step_by(jobs as usize) {
                    let mut iter = Float::with_val(precision, 2 * n + 1);
                    iter.div_from(4 - (n as i64 % 2) * 8);
                    sum_iters.add_from(iter);
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
    }

    fn nk_run(&mut self) {
        let iterations = self.iterations;
        let jobs = self.jobs;
        let precision = self.precision;

        let mut job_handles = Vec::new();
        for offset in 1..=self.jobs {
            job_handles.push(thread::spawn(move || {
                let mut sum_iters = Float::with_val(precision, 0);
                for n in (offset..iterations).step_by(jobs as usize) {
                    let mut iter = Float::with_val(precision, (2 * n) * (2 * n + 1) * (2 * n + 2));
                    iter.div_from(4 - ((n as i64 + 1) % 2) * 8);
                    sum_iters.add_from(iter);
                }
                sum_iters
            }))
        }

        let mut piaprox = Float::with_val(self.precision, 3);
        for job in job_handles {
            let result = job.join().unwrap();
            piaprox.add_from(result);
        }
        self.backend = Backend::Rug(Some(piaprox));
    }
}
