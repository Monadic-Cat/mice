[![Build Status](https://travis-ci.org/Monadic-Cat/mice.svg?branch=master)](https://travis-ci.org/Monadic-Cat/mice)
[![docs.rs](https://docs.rs/mice/badge.svg)](https://docs.rs/mice/)
![License](https://img.shields.io/crates/l/mice)

# mice, messing with dice

This is a simple crate for evaluating dice expressions.
It will receive features as I desire, or as are contributed
if I happen to be so lucky.

Panicking from this crate is considered a serious bug.
Please  submit an Issue on this project's GitHub repo
if it manages to do that, and please describe the usage
that it panicked from.

Without further ado, here's some usage:

```rust
use mice::roll;

println!("{}", roll("2d6 + 3")?);

println!("{}", roll("2d6 + 3")?.total());

let result = roll("2d6 + 3")?;
println!("{}\n{}", result, result.total());
```

The parser accepts an arbitrary number of terms in a dice expression.
```rust
use mice::roll;
println!("{}", roll("9d8 + 4d2 - 5 - 8d7")?);
```
