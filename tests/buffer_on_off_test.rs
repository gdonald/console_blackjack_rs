use console_blackjack_rs::{buffer_off, buffer_on, MockTermiosWrapper};

#[test]
fn test_buffer_off() {
    let mut termios = MockTermiosWrapper;
    buffer_off(&mut termios);
}

#[test]
fn test_buffer_on() {
    let mut termios = MockTermiosWrapper;
    buffer_on(&mut termios);
}
