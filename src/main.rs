//be means backend
mod bigdecimal_be;
mod f64_be;
mod rug_be;
mod rust_decimal_be;
mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[arg(short, long, help = "The backend library that will be used", value_parser = ["rust_decimal", "bigdecimal", "rug", "f64"])]
    back_end: String,

    #[arg(short, long, default_value_t = 1_000_000)]
    iterations: usize,

    #[arg(short, long, default_value_t = 1)]
    jobs: usize,
}

fn main() {
    let args = Args::parse();

    match args.back_end.as_str() {
        "rust_decimal" => rust_decimal_be::run(args.iterations, args.jobs),
        "bigdecimal" => bigdecimal_be::run(args.iterations, args.jobs),
        "f64" => f64_be::run(args.iterations, args.jobs),
        "rug" => rug_be::run(args.iterations, args.jobs),
        _ => (),
    }

    // match back_end.as_str() {
    //     "rust_decimal" => rust_decimal_be::run(max, jobs),
    //     "bigdecimal" => bigdecimal_be::run(max, jobs),
    //     "f64" => f64_be::run(max, jobs),
    //     _ => utils::print_help_and_exit(0),
    // }
}
