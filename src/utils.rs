use std::{fmt::Display, str::FromStr, time::Duration};

pub fn print_help_and_exit(exit_code: i32) {
    println!(
        "
A simple pi approximator
How to use:
    <back_end> <iterations> <jobs>

    <back_end> choose between:
        - rust_decimal
        - rug
        - bigdecimal
    "
    );
    std::process::exit(exit_code);
}

pub fn parse_userargs<T>(n: usize) -> Result<T, ()>
where
    T: FromStr,
{
    match std::env::args().nth(n) {
        Some(arg) => match arg.parse::<T>() {
            Ok(r) => Ok(r),
            Err(_) => {
                print_help_and_exit(2);
                Err(())
            }
        },
        None => {
            print_help_and_exit(1);
            Err(())
        }
    }
}

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
