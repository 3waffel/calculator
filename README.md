# Simple Calculator

Define the syntax in `src/*.lalrpop`

Then include the syntax in `src/main.rs`
```rust
#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);
```