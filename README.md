# Simple Pi Approximator

Baised on the Tailor Series of atan(1).
Also able to do multithreading by precising the number of jobs.
For now it is using `rust_decimal` which is not very precise.

## Usage

```bash
cargo run --release -- <number of iterations> <jobs>
```
