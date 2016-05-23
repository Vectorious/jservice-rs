# jservice-rs

[![Build Status](https://travis-ci.org/Vectorious/jservice-rs.svg?branch=master)](https://travis-ci.org/Vectorious/jservice-rs)

A [jService](http://jservice.io) API wrapper for Rust.

# Documentation

https://vectorious.github.io/jservice-rs/

# Usage Example

```rust
extern crate jservice;
use jservice::Clue;

fn main() {
    // get 20 random clues
    let clues: Vec<Clue> = jservice::get_random(Some(20)).unwrap();

    for clue in clues {
        println!("{}", clue.question);
    }
}
```
