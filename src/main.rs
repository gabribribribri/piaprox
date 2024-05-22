//be means backend
mod aprox;
mod bigdecimal_be;
mod f64_be;
mod rug_be;
mod rust_decimal_be;

use std::time::Duration;

use aprox::{Aprox, Backend, Strategy};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true, short, long, help = "The backend library that will be used", value_parser = ["rust_decimal", "bigdecimal", "rug", "f64"])]
    backend: String,

    #[arg(short, long, default_value_t = String::from("gregoryleibniz"), help = "The Strategy used to approximate pi", value_parser = ["gregory" , "gregoryleibniz" , "gl" , "g", "nilakantha" , "nk" , "n"])]
    strategy: String,

    #[arg(short, long, default_value_t = 1_000_000)]
    iterations: u64,

    #[arg(short, long, default_value_t = 1)]
    jobs: u64,

    #[arg(
        short,
        long,
        default_value_t = 128,
        help = "Only useful with 'rug' backend"
    )]
    precision: u32,
}

fn main() {
    let args = Args::parse();

    let mut aprox = Aprox {
        backend: Backend::from(args.backend),
        strategy: Strategy::from(args.strategy),
        iterations: args.iterations,
        jobs: args.jobs,
        time: Duration::ZERO,
        precision: args.precision,
    };

    aprox.run();
    aprox.result_message();
}
