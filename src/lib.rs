//! multi-print - Extension Trait for convenient string value output multiplication
//! 
//! This is a basic crate that provides a string multiplier Trait and implements it 
//! for Rust's `std::str::String`. 
//! 
//! If you wish to implement the `MultiPrint` trait for your types, you will need 
//! to ensure that those types support the `ToString` trait which is implemented 
//! by default when you implement `std::fmt::Display`.
//! 
//! An example of using the crate:
//! 
//! ```
//! use multiprint::MultiPrint;
//! 
//! fn main() {
//!     let s = String::from("Echo..");
//!     println!("{}", s.times(5, ' '));
//! 
//!     assert_eq!(s.times(5, ' '), "Echo.. Echo.. Echo.. Echo.. Echo..");
//!     assert_eq!("Hello!".to_string().times(2, ' '), "Hello! Hello!");
//! }
//! ```
use std::string::ToString;

/// This MultiPrint trait exposes a default implementation of the `times()` method
pub trait MultiPrint : ToString {
    // fn times(&self, n: usize, sep: char) -> String;
    fn times(&self, n: usize, sep: char) -> String {
        let mut str = String::new();
        for i in 0..n {
            if i < n && i > 0 {
                str.push(sep);
            }
            str.push_str(&self.to_string());
        }
        str
    }
}

impl MultiPrint for String {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_twice() {
        let s = String::from("Hello");

        assert_eq!(s.times(2, ' '), "Hello Hello");
    }

    #[test]
    fn do_it_5_times() {
        let s = String::from("Peter");

        assert_eq!(s.times(5, '-'), "Peter-Peter-Peter-Peter-Peter");
    }
}
