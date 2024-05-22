use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::{thread, time::Instant};

use crate::aprox::{Aprox, Backend};

pub trait RustDecimalBackend {
    fn gl_run(&mut self);
    fn nk_run(&mut self);
}

impl RustDecimalBackend for Aprox {
    fn gl_run(&mut self) {
        let timer = Instant::now();

        let iterations = self.iterations;
        let jobs = self.jobs;

        let mut job_handles = Vec::new();
        for offset in 1..=self.jobs {
            job_handles.push(thread::spawn(move || {
                let mut sum_iters = Decimal::ZERO;
                let mut n = offset;

                while n < iterations {
                    sum_iters += Decimal::from(4 - (n as i64 % 2) * 8) / Decimal::from(2 * n + 1);
                    n += jobs;
                }
                return sum_iters;
            }))
        }

        let piaprox = job_handles
            .into_iter()
            .map(|j| j.join().unwrap())
            .sum::<Decimal>()
            + dec!(4);
        self.backend = Backend::RustDecimal(Some(piaprox));

        self.time = timer.elapsed();
    }
    fn nk_run(&mut self) {
        todo!()
    }
}
