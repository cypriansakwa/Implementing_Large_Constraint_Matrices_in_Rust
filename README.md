# Implementing Large Constraint Matrices in Rust

This project implements large constraint matrices using the `ndarray` crate in Rust. It verifies matrix constraints using randomly generated values and ensures the computed values satisfy the required equations.

## Features
- Uses the `ndarray` crate for matrix operations.
- Generates random values for computation.
- Constructs and verifies constraint matrices.
- Ensures consistency using Rust's strong type system and assertions.

## Dependencies
This project requires Rust and Cargo. Ensure you have the following dependencies in `Cargo.toml`:

```toml
[dependencies]
rand = "0.9.0"
ndarray = "0.16.1"
```
## Installation
Clone the repository and navigate to the project directory:
```sh
clone git https://github.com/cypriansakwa/Implementing_Large_Constraint_Matrices_in_Rust.git
cd cypriansakwa/Implementing_Large_Constraint_Matrices_in_Rust
```
## Usage
Build and run the program using Cargo:
```sh
cargo build
cargo run
```
## Example output:
```sql
Generated values: x = 210, y = 895
Computed values: w = [1, 119346253, 210, 895, 132300, 118408500], shape=[6], strides=[1], layout=CFcf (0xf), const ndim=1
Left side: [132300, 118408500, 939750], shape=[3], strides=[1], layout=CFcf (0xf), const ndim=1
Right side: [132300, 118408500, 939750], shape=[3], strides=[1], layout=CFcf (0xf), const ndim=1
Verification successful!
```
