//be means backend
mod bigdecimal_be;
mod f64_be;
mod rug_be;
mod rust_decimal_be;
mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true, short, long, help = "The backend library that will be used", value_parser = ["rust_decimal", "bigdecimal", "rug", "f64"])]
    back_end: String,

    #[arg(short, long, default_value_t = 1_000_000)]
    iterations: u32,

    #[arg(short, long, default_value_t = 1)]
    jobs: u32,

    #[arg(short, long, default_value_t = 128)]
    precision: u32,
}

fn main() {
    let args = Args::parse();

    match args.back_end.as_str() {
        "rust_decimal" => rust_decimal_be::run(args.iterations, args.jobs),
        "bigdecimal" => bigdecimal_be::run(args.iterations, args.jobs),
        "f64" => f64_be::run(args.iterations, args.jobs),
        "rug" => rug_be::run(args.iterations, args.jobs, args.precision),
        _ => (),
    }
}
