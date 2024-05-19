use std::{fmt::Display, time::Duration};

pub fn result_message<T>(strategy: &str, max: usize, jobs: usize, time: Duration, piaprox: T)
where
    T: Display,
{
    println!("RESULTS ({})", strategy);
    println!(
        "Made {} iterations with {} threads in {:?}.",
        max, jobs, time
    );
    println!("Pi approximation : {}", piaprox);
}
