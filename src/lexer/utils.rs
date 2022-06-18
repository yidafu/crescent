pub fn is_whitespace(c: char) -> bool {
    c == '\n' || c == '\r' || c == ' '
}

pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn is_hex_digit(c: char) -> bool {
    is_digit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
}

pub fn is_letter(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')
}

#[test]
fn test_is_whitespace() {
    assert!(is_whitespace('\t'));
    assert!(is_whitespace('\n'));
    assert!(is_whitespace(' '));
    assert!(!is_whitespace('a'));
}

#[test]
fn test_is_digit() {
    assert!(is_digit('0'));
    assert!(is_digit('9'));
    assert!(is_digit('5'));
    assert!(!is_digit('a'));
}

#[test]
fn test_is_hex_digit() {
    assert!(is_hex_digit('0'));
    assert!(is_hex_digit('a'));
    assert!(is_hex_digit('f'));
    assert!(!is_hex_digit('g'));
}

#[test]
fn test_is_letter() {
    assert!(is_letter('a'));
    assert!(is_letter('x'));
    assert!(is_letter('A'));
    assert!(is_letter('Z'));
    assert!(!is_letter('0'));
}
