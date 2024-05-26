use bigdecimal::{BigDecimal, Zero};
use std::thread;

use crate::aprox::{Aprox, Backend};

pub trait BigDecimalBackend {
    fn gl_run(&mut self);
    fn nk_run(&mut self);
}

impl BigDecimalBackend for Aprox {
    fn gl_run(&mut self) {
        let iterations = self.iterations;
        let jobs = self.jobs;

        let mut job_handles = Vec::new();
        for offset in 1..=jobs {
            job_handles.push(thread::spawn(move || {
                let mut sum_iters = BigDecimal::zero();

                for n in (offset..iterations).step_by(jobs as usize) {
                    sum_iters +=
                        BigDecimal::from(4 - (n as i64 % 2) * 8) / BigDecimal::from(2 * n + 1);
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
    }

    fn nk_run(&mut self) {
        let iterations = self.iterations;
        let jobs = self.jobs;

        let mut job_handles = Vec::new();
        for offset in 1..=jobs {
            job_handles.push(thread::spawn(move || {
                let mut sum_iters = BigDecimal::zero();

                for n in (offset..iterations).step_by(jobs as usize) {
                    sum_iters += BigDecimal::from(4 - ((n as i64 + 1) % 2) * 8)
                        / BigDecimal::from((2 * n) * (2 * n + 1) * (2 * n + 2));
                }
                sum_iters
            }))
        }
        let piaprox = job_handles
            .into_iter()
            .map(|j| j.join().unwrap())
            .sum::<BigDecimal>()
            + BigDecimal::from(3);

        self.backend = Backend::BigDecimal(Some(piaprox));
    }
}
