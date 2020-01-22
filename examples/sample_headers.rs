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
    println!("{}\n", "Header 1".to_string().underline('-'));

    // this will output:
    //
    //   --------
    //   Header 1
    //
    println!("{}\n", "Header 1".to_string().overline('-'));

    // this will output:
    //
    //   ________
    //   Header 1
    //   ========
    //
    println!("{}\n", "Header 1".to_string().outline('_', '='));
}