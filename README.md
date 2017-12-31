# Kahan Sum

This crate implements a type for computing [Kahan sums](https://en.wikipedia.org/wiki/Kahan_summation_algorithm) over floating point numbers. It also implements a new trait for computing Kahan sums over iterators of floats.

## To use Kahan Sum in your project

Add
```toml
[dependencies]
kahan = "0.1"
```
to your `Cargo.toml`, and
```rust
extern crate kahan;
```
to your `lib.rs` or `main.rs`.

## Example

For examples, please see the [documentation](https://docs.rs/kahan/0.1.0/kahan/).