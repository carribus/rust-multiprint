[![Build Status](https://travis-ci.org/carribus/rust-multiprint.svg?branch=master)](https://travis-ci.org/carribus/rust-multiprint)
[![cratesio](https://img.shields.io/crates/v/multiprint.svg)](https://crates.io/crates/multiprint)
[![docsrs](https://docs.rs/multiprint/badge.svg)](https://docs.rs/multiprint)

# MultiPrint - A convenient string value multiplier trait for Rust

This is a basic crate that provides a string multiplier Trait and implements it 
for Rust's `std::str::String`. 

If you wish to implement the `MultiPrint` trait for your types, you will need 
to ensure that those types support the `ToString` trait which is implemented 
by default when you implement `std::fmt::Display`.

An example of using the crate:

```
use multiprint::{MultiPrint, Decorate};

fn main() {
    let s = String::from("Echo..");
    println!("{}", s.times(5, ' '));

    assert_eq!(s.times(5, ' '), "Echo.. Echo.. Echo.. Echo.. Echo..");
    assert_eq!("Hello!".to_string().times(2, ' '), "Hello! Hello!");

    // this will output:
    //
    //   Header 1
    //   --------
    //
    println!("{}", "Header 1".to_string().underline('-'));

    // this will output:
    //
    //   --------
    //   Header 1
    //
    println!("{}", "Header 1".to_string().overline('-'));

    // this will output:
    //
    //   ________
    //   Header 1
    //   ========
    //
    println!("{}", "Header 1".to_string().outline('_', '='));

    // this will output:
    //
    //   ------------
    //   | Header 1 |
    //   ------------
    //
    println!("{}", "Header 1".to_string().border('-', '|'));
}
```


## Example(s)

At the moment, there is only one example since the crate is quite simple. If the complexity grows over time,
I will add more targetted examples. In the meanwhile, to run the example, just type:

```
$ cargo run --example sample_headers
```
