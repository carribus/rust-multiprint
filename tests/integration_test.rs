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
