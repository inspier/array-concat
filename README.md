[![Current Crates.io Version](https://img.shields.io/crates/v/array-concat.svg)](https://crates.io/crates/array-concat)
[![docs-rs](https://docs.rs/array-concat/badge.svg)](https://docs.rs/array-concat)

# array-concat

Macros for concatenating const arrays.

To add to your Cargo.toml:
```toml
array-concat = "0.3.0"
```

## Basic Usage

```
concat_arrays!(<comma-seperated arrays>, <copyable value to be used for array initialization>)
```

## Example
```rust
use array_concat::*;

const A: [u32; 3] = [1, 2, 3];
const B: [u32; 2] = [4, 5];
const C: [u32; concat_arrays_size!(A, B)] = concat_arrays!(A, B; u32::MIN); // compiles

// Non-Copy struct
struct S {}
const D: [S; 1] = [S{}];
const E: [S; 1] = [S{}];
const F: [S; concat_arrays_size!(D, E)] = concat_arrays!(D, E; S{}); // doesn't compile

fn main() {
    let c = concat_arrays!(A, B; u32::MIN);
    assert_eq!(C[..], A.iter().chain(&B).copied().collect::<Vec<u32>>()[..]);
    assert_eq!(c[..], A.iter().chain(&B).copied().collect::<Vec<u32>>()[..]);
}
```
