use std::thread;

use crate::aprox::{Aprox, Backend};

pub trait F64Backend {
    fn nk_run(&mut self);
    fn gl_run(&mut self);
}

impl F64Backend for Aprox {
    fn gl_run(&mut self) {
        let jobs_f64 = self.jobs as f64;
        let iterations_f64 = self.iterations as f64;

        let mut job_handles = Vec::new();
        for offset in 1..=self.jobs {
            job_handles.push(thread::spawn(move || {
                let mut sum_iters = 0f64;
                let mut n = offset as f64;

                while n < iterations_f64 {
                    sum_iters += (4f64 - (n % 2f64) * 8f64) / (2f64 * n + 1f64);
                    n += jobs_f64;
                }
                sum_iters
            }))
        }

        let piaprox = job_handles
            .into_iter()
            .map(|j| j.join().unwrap())
            .sum::<f64>()
            + 4f64;

        self.backend = Backend::F64(Some(piaprox));
    }

    fn nk_run(&mut self) {
        let jobs_f64 = self.jobs as f64;
        let iterations_f64 = self.iterations as f64;

        let mut job_handles = Vec::new();
        for offset in 1..=self.jobs {
            job_handles.push(thread::spawn(move || {
                let mut sum_iters = 0f64;
                let mut n = offset as f64;

                while n < iterations_f64 {
                    sum_iters += (4f64 - ((n + 1f64) % 2f64) * 8f64)
                        / ((2f64 * n) * (2f64 * n + 1f64) * (2f64 * n + 2f64));
                    n += jobs_f64;
                }
                sum_iters
            }))
        }

        let piaprox = job_handles
            .into_iter()
            .map(|j| j.join().unwrap())
            .sum::<f64>()
            + 3f64;

        self.backend = Backend::F64(Some(piaprox));
    }
}
