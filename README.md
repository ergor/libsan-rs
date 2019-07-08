# libsan-rs
Rust implementation of [standard-algebraic-notation](https://github.com/chesszebra/standard-algebraic-notation) for parsing standard algebraic notation in chess.

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
    let san_string = Move::to_string(move_data); // -> "Re4"
}
```
