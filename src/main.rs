//be means backend
mod bigdecimal_be;
mod rust_decimal_be;
mod utils;

fn main() {
    let back_end = utils::parse_userargs::<String>(1).unwrap();
    let max = utils::parse_userargs::<usize>(2).unwrap();
    let jobs = utils::parse_userargs::<usize>(3).unwrap();

    match back_end.as_str() {
        "rust_decimal" => rust_decimal_be::run(max, jobs),
        "bigdecimal" => bigdecimal_be::run(max, jobs),
        _ => utils::print_help_and_exit(0),
    }
}
