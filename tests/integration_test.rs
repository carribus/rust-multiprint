use multiprint::*;

#[test]
fn two_times() {
    assert_eq!("Hello Hello", "Hello".to_string().times(2, ' '));
}

#[test]
fn three_times() {
    assert_eq!("!?!?!", "!".to_string().times(3, '?'));
}

#[test]
fn ten_times() {
    assert_eq!("...................", ".".to_string().times(10, '.'));
}

#[test]
fn underline_word() {
    assert_eq!("Header\n______", "Header".to_string().underline('_'));
}

#[test]
fn overline_word() {
    assert_eq!("______\nHeader", "Header".to_string().overline('_'));
}

#[test]
fn outline_word() {
    assert_eq!("******\nHeader\n******", "Header".to_string().outline('*', '*'));
}

#[test]
fn border_word() {
    assert_eq!("Header".to_string().border('-', '|'), "----------\n| Header |\n----------");
}