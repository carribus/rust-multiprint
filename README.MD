[![Build Status](https://travis-ci.org/carribus/rust-multiprint.svg?branch=master)](https://travis-ci.org/carribus/rust-multiprint)

# MultiPrint - A convenient string value multiplier trait for Rust

This is a basic crate that provides a string multiplier Trait and implements it 
for Rust's `std::str::String`. 

If you wish to implement the `MultiPrint` trait for your types, you will need 
to ensure that those types support the `ToString` trait which is implemented 
by default when you implement `std::fmt::Display`.

An example of using the crate:

```
use multiprint::MultiPrint;

fn main() {
    let s = String::from("Echo..");
    println!("{}", s.times(5, ' '));

    assert_eq!(s.times(5, ' '), "Echo.. Echo.. Echo.. Echo.. Echo..");
    assert_eq!("Hello!".to_string().times(2, ' '), "Hello! Hello!");
}
```