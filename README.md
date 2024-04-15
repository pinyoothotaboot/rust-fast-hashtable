# Rust Dictionary
## _The optimization data structor dictionary_

[![Build Status](https://travis-ci.org/joemccann/dillinger.svg?branch=master)](https://travis-ci.org/joemccann/dillinger)

This project created for optimized data structor for Dictionary and Set.The target to improve fast and efficiency of Hash Table ,Hash Set with Get,Set,Update,Delete and safe data.Source code create by Rust 100%

## Features

- SDict -> Support use hashtable by key is string ( range 2 to 11 charactor limit )
- IDict -> Support use hashtable by key is integer
- SSet  -> Support use hashset by key is string
- ISet  -> Support use hashset by key is integer

## Installation

This project requires [Rust](https://www.rust-lang.org/tools/install)

Install the environment for rust compiler

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Development

You can improve the source code to implement to project 

Install dependencies:

```sh
cargo install
```

Run normal:

```sh
cargo run
```

Run unittest

```sh
cargo test
```

#### Building for source

Build source code:

```sh
cargo build
```

Build with optimized for production:

```sh
cargo build --release
```

## Example

```rs
let mut dict : SDict<String>  = SDict::<String>::new();
let key = String::from("Hello");
let value = String::from("World");
let do_set = dict.set(key.clone(), value.clone());
println!("SET : {:?}",do_set);
```

## Benchmark

Run benchmark

```sh
cargo bench
```

Run benchmark specify

```sh
cargo bench --bench vec_benchmark
```

Report benchmark

```sh
    Finished bench [optimized] target(s) in 15.84s
     Running benches/sdict_benchmark.rs (target/release/deps/sdict_benchmark-5e6c98c21226ae3f)
Gnuplot not found, using plotters backend
Set value to sdict      time:   [11.693 ns 11.780 ns 11.869 ns]
                        change: [-4.1864% -3.1820% -2.2108%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

Get value from sdict by key
                        time:   [9.9544 ns 10.001 ns 10.052 ns]
                        change: [-4.3150% -2.8085% -1.6914%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

Update value to sdict by key
                        time:   [10.279 ns 10.325 ns 10.372 ns]
                        change: [-3.4471% -2.2999% -1.1845%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe
```

## Incomming

- ISet  : The hashset support key is integer
- Optimized all data structor

## License

[MIT](https://github.com/pinyoothotaboot/dictionary/blob/main/LICENSE)

## Author
_Pinyoo Thotaboot_
