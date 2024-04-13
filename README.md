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

Report benchmark

```sh
     Running benches/sdict_benchmark.rs (target/release/deps/sdict_benchmark-5e6c98c21226ae3f)
Gnuplot not found, using plotters backend
Set value to sdict      time:   [81.240 ns 81.773 ns 82.436 ns]
                        change: [+0.0933% +1.3391% +2.6690%] (p = 0.04 < 0.05)
                        Change within noise threshold.
Get value from sdict by key
                        time:   [35.934 ns 36.116 ns 36.310 ns]
                        change: [-1.3300% +0.2011% +1.5002%] (p = 0.81 > 0.05)
                        No change in performance detected.
Update value to sdict by key
                        time:   [27.953 ns 28.103 ns 28.304 ns]
                        change: [+0.1881% +0.8586% +1.5229%] (p = 0.01 < 0.05)
                        Change within noise threshold.
```

## Incomming

- IDict : The dictionary support key is integer
- SSet  : The hashset support key is string
- ISet  : The hashset support key is integer
- Optimized all data structor

## License

[MIT](https://github.com/pinyoothotaboot/dictionary/blob/main/LICENSE)

## Author
_Pinyoo Thotaboot_
