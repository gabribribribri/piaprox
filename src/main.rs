use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::thread;

fn main() {
    let max = std::env::args().nth(1).unwrap().parse::<Decimal>().unwrap();
    let jobs = std::env::args().nth(2).unwrap().parse::<i64>().unwrap();
    let djobs = Decimal::new(jobs, 0);

    let mut job_handles = Vec::new();
    for offset in 1..=jobs {
        job_handles.push(thread::spawn(move || {
            let mut sum_iters = Decimal::ZERO;
            let mut n = Decimal::new(offset, 0);
            while n < max {
                sum_iters += (dec!(1) - (n % dec!(2)) * dec!(2)) / (dec!(2) * n + dec!(1));
                n += djobs;
            }
            return sum_iters;
        }))
    }

    let pi = (job_handles
        .into_iter()
        .map(|j| j.join().unwrap())
        .sum::<Decimal>()
        + Decimal::ONE)
        * dec!(4);

    println!("{}", pi);
}
