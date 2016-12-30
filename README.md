# tinyexpr-rs

[![Crates.io](https://img.shields.io/crates/v/tinyexpr.svg)](https://crates.io/crates/tinyexpr)
[![Documentation](https://docs.rs/tinyexpr/badge.svg)](https://docs.rs/tinyexpr)
[![Build Status](https://travis-ci.org/kondrak/tinyexpr-rs.svg)](https://travis-ci.org/kondrak/tinyexpr-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/gmkbjqne3be843is?svg=true)](https://ci.appveyor.com/project/kondrak/tinyexpr-rs)
[![Coverage Status](https://coveralls.io/repos/github/kondrak/tinyexpr-rs/badge.svg?branch=master)](https://coveralls.io/github/kondrak/tinyexpr-rs?branch=master)
![](https://img.shields.io/crates/l/json.svg)

Tiny recursive descent expression parser, compiler, and evaluation engine for math expressions.

This is a WIP port of [TinyExpr](https://github.com/codeplea/tinyexpr) to Rust. Current release only supports built-in system functions (trigonometry, algebraic operations, constants, etc.). See the `tests` module for more examples.

[Documentation](https://docs.rs/tinyexpr)

Usage
-----
```toml
# Cargo.toml
[dependencies]
tinyexpr = "0.0.1"
```

Example
-------
```rust
extern crate tinyexpr;

fn main()
{
    // parse the expression and fetch result
    let r = tinyexpr::interp("2+2*2").unwrap();

    // should print "6"
    println!("{:?}", r);
}
```

Build instructions
------------------

```
cargo build
cargo run --example example
```

## Todo
- support for custom user function
- support for functions taking more than 2 parameters

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)