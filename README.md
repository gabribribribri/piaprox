# Simple Pi Approximator

Baised on the Gregory Leibniz Series (pi = 4 - 4/3 + 4/5 - 4/7 + 4/9...)
I might implement the Nilakantha series later.
Also able to do multithreading by precising the number of jobs.

```
Usage: piaprox [OPTIONS] --back-end <BACK_END>

Options:
  -b, --back-end <BACK_END>      The backend library that will be used [possible values: rust_decimal, bigdecimal, rug, f64]
  -i, --iterations <ITERATIONS>  [default: 1000000]
  -j, --jobs <JOBS>              [default: 1]
  -h, --help                     Print help
  -V, --version                  Print version
```

