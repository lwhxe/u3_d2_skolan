# How to Compile and Run this Rust Program

This project is a simple Rust application. To compile and run it, follow these steps:

## Prerequisites

Make sure you have Rust and Cargo installed. If not, you can install them by following the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Compilation

Navigate to the root directory of this project in your terminal. Then, use Cargo to build the project:

```bash
cargo build
```

This command will compile the `src/main.rs` file and create an executable in the `target/debug/` directory.

## Running the Program

After successful compilation, you can run the program using Cargo:

```bash
cargo run
```

Alternatively, you can directly execute the compiled binary:

```bash
./target/debug/u3_d2_skolan
```

(Note: Replace `u3_d2_skolan` with the actual name of your executable if it's different, which is typically derived from the `[package] name` in `Cargo.toml`).
