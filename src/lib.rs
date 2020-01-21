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
//! use multiprint::{MultiPrint, Decorate};
//! 
//! fn main() {
//!     let s = String::from("Echo..");
//!     println!("{}", s.times(5, ' '));
//! 
//!     assert_eq!(s.times(5, ' '), "Echo.. Echo.. Echo.. Echo.. Echo..");
//!     assert_eq!("Hello!".to_string().times(2, ' '), "Hello! Hello!");
//! 
//!     // this will output:
//!     //
//!     //   Header 1
//!     //   --------
//!     //
//!     println!("{}", "Header 1".underline('-'));
//! 
//!     // this will output:
//!     //
//!     //   --------
//!     //   Header 1
//!     //
//!     println!("{}", "Header 1".overline('-'));
//! 
//!     // this will output:
//!     //
//!     //   ________
//!     //   Header 1
//!     //   ========
//!     //
//!     println!("{}", "Header 1".outlne('_', '='));
//!     
//! }
//! ```
use std::string::ToString;

/// This MultiPrint trait exposes a default implementation of the `times()` method
/// 
/// By default, the `times` method is auto-implemented for the `String` type
pub trait MultiPrint : ToString {
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

/// The `Decorate` trait exposes methods for decorating text with underlines, overlines and outlines 
/// 
/// By default, the `Decorate` trait methods are automatically implemented for the `String` type
pub trait Decorate : ToString {
    /// Render the string with an underline on the next line using the specific character
    fn underline(&self, underline_character: char) -> String;

    /// Render the string with an overline on the previous line using the specific character
    fn overline(&self, overline_character: char) -> String;

    /// Render the string with both an over- and under-line on the previous and following lines using the specific characters
    fn outline(&self, overline_character: char, underline_character: char) -> String;
}

impl MultiPrint for String {}

impl Decorate for String {
    fn underline(&self, underline_character: char) -> String {
        format!("{}\n{}", self.to_string(), std::iter::repeat(underline_character).take(self.len()).collect::<String>())
    }

    fn overline(&self, overline_character: char) -> String {
        format!("{}\n{}", std::iter::repeat(overline_character).take(self.len()).collect::<String>(), self.to_string())
    }

    fn outline(&self, overline_character: char, underline_character: char) -> String {
        format!("{}\n{}\n{}", 
            std::iter::repeat(overline_character).take(self.len()).collect::<String>(), 
            self.to_string(),
            std::iter::repeat(underline_character).take(self.len()).collect::<String>()
        )
    }
}

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

    #[test]
    fn underline_test() {
        let s = String::from("This is a header");

        assert_eq!(s.underline('-'), "This is a header\n----------------");
    }

    #[test]
    fn overline_test() {
        let s = String::from("This is a header");

        assert_eq!(s.overline('-'), "----------------\nThis is a header");
    }

    #[test]
    fn outline() {
        let s = String::from("This is a header");

        assert_eq!(s.outline('-', '='), "----------------\nThis is a header\n================");
    }
}
