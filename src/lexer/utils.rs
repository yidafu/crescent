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