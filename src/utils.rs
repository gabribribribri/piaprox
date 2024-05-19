use std::{fmt::Display, time::Duration};

pub fn result_message<T>(strategy: &str, iterations: u64, jobs: u64, time: Duration, piaprox: T)
where
    T: Display,
{
    println!("RESULTS ({})", strategy);
    println!(
        "Made {} iterations with {} threads in {:?}.",
        iterations, jobs, time
    );
    println!("Pi approximation : {}", piaprox);
}
