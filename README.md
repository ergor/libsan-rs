# libsan-rs
Rust implementation of [standard-algebraic-notation](https://github.com/chesszebra/standard-algebraic-notation) for parsing standard algebraic notation in chess.

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
san-rs = "0"
```

then add this use declaration:

```rust
use san_rs::*;
```

Parsing input string:

```rust
let move_data = Move::parse("Re4").unwrap();
```

Converting movement struct back to string:

```rust
let move_data = Move::new();
// ...
let san_string = Move::to_string(move_data);
```
