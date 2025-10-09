## Running the Benchmark Binary

This crate provides a benchmark binary to compare XML parsing libraries.

### Prerequisites

- [Rust toolchain](https://rustup.rs/) installed

### Build and Run

To build and run the benchmark binary:

```sh
cargo run --release --bin benchmark --features=hotpath
```

This will execute the `benchmark` binary with optimizations enabled with default features.
The default features runs identical workloads for Serialization and Deserialization workloads through the independent crates. Currently tested crates are:

- instant-xml
- yaserde
- xmlity

You can customize what crates to test by specifying the independent crate features explicitly as shown below:

```sh
cargo run --release --bin benchmark --no-default-features --features="hotpath,instant-xml,xmlity"
```

### Notes

- Input XML files are located in the `data/` directory.
- For more details, see the source code in [src/main.rs](src/main.rs).
