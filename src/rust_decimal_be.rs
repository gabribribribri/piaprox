use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::thread;

use crate::aprox::{Aprox, Backend};

pub trait RustDecimalBackend {
    fn gl_run(&mut self);
    fn nk_run(&mut self);
}

impl RustDecimalBackend for Aprox {
    fn gl_run(&mut self) {
        let iterations = self.iterations;
        let jobs = self.jobs;

        let mut job_handles = Vec::new();
        for offset in 1..=self.jobs {
            job_handles.push(thread::spawn(move || {
                let mut sum_iters = Decimal::ZERO;
                for n in (offset..iterations).step_by(jobs as usize) {
                    sum_iters += Decimal::from(4 - (n as i64 % 2) * 8) / Decimal::from(2 * n + 1);
                }
                sum_iters
            }))
        }

        let piaprox = job_handles
            .into_iter()
            .map(|j| j.join().unwrap())
            .sum::<Decimal>()
            + dec!(4);
        self.backend = Backend::RustDecimal(Some(piaprox));
    }
    fn nk_run(&mut self) {
        let iterations = self.iterations;
        let jobs = self.jobs;

        let mut job_handles = Vec::new();
        for offset in 1..=self.jobs {
            job_handles.push(thread::spawn(move || {
                let mut sum_iters = Decimal::ZERO;
                for n in (offset..iterations).step_by(jobs as usize) {
                    sum_iters += Decimal::from(4 - ((n as i64 + 1) % 2) * 8)
                        / Decimal::from((2 * n) * (2 * n + 1) * (2 * n + 2));
                }
                sum_iters
            }))
        }

        let piaprox = job_handles
            .into_iter()
            .map(|j| j.join().unwrap())
            .sum::<Decimal>()
            + dec!(3);
        self.backend = Backend::RustDecimal(Some(piaprox));
    }
}
