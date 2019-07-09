# libsan-rs
Rust implementation for parsing standard algebraic notation in chess inspired by 
[standard-algebraic-notation](https://github.com/chesszebra/standard-algebraic-notation).

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
san-rs = "0"
```

Short example usage:

```rust
use san_rs::*;

fn main() {
    // parse input string:
    let move_data = Move::parse("Re4").unwrap(); // -> data struct

    // convert back to string:
    let san_string = move_data.compile().unwrap(); // -> "Re4"
}
```
