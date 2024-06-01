[![Current Crates.io Version](https://img.shields.io/crates/v/array-concat.svg)](https://crates.io/crates/array-concat)
[![docs-rs](https://docs.rs/array-concat/badge.svg)](https://docs.rs/array-concat)

# array-concat

Macros for concatenating arrays.

To add to your Cargo.toml:
```toml
array-concat = "0.5.3"
```

## Example
```rust
use array_concat::*;

const A: [u32; 3] = [1, 2, 3];
const B: [u32; 2] = [4, 5];
const C: [u32; concat_arrays_size!(A, B)] = concat_arrays!(A, B);

// Non-Copy struct
#[derive(Debug, PartialEq)]
struct S(bool);
const D: [S; 1] = [S(true)];
const E: [S; 1] = [S(false)];
const F: [S; concat_arrays_size!(D, E)] = concat_arrays!(D, E);

fn main() {
    let c = concat_arrays!(A, B);
    assert_eq!([1, 2, 3, 4, 5], C);
    assert_eq!([1, 2, 3, 4, 5], c);
    assert_eq!([S(true), S(false)], F);

    let a = [1, 2, 3];
    let b = [4, 5];
    let c = concat_arrays!(a, b); // Size is inferred by the assert below
    assert_eq!([1, 2, 3, 4, 5], c);
}
```
