use super::chunk_stream::ChunkStream;
use super::token::Token;
use super::utils::{is_digit, is_hex_digit, is_letter, is_whitespace};

struct Lexer {
    pub stream: ChunkStream,
}

impl Lexer {
    fn peek_token() -> Token {
        Token::eof_token()
    }

    fn next_token(&mut self) -> Token {
        self.skip_white_space();
        if self.eof() {
            return Token::eof_token();
        }

        let char = self.stream.peek();
        match char {
            ';' => {
                self.stream.next();
                Token::semi_token()
            }
            ',' => {
                self.stream.next();
                Token::comma_token()
            }
            '(' => {
                self.stream.next();
                Token::open_paren_token()
            }
            ')' => {
                self.stream.next();
                Token::close_paren_token()
            }
            ']' => {
                self.stream.next();
                Token::close_bracket_token()
            }
            '{' => {
                self.stream.next();
                Token::open_brace_token()
            }
            '}' => {
                self.stream.next();
                Token::close_brace_token()
            }
            '+' => {
                self.stream.next();
                Token::plus_token()
            }
            '-' => {
                self.stream.next();
                Token::minus_token()
            }
            '*' => {
                self.stream.next();
                Token::mul_token()
            }
            '^' => {
                self.stream.next();
                Token::pow_token()
            }
            '%' => {
                self.stream.next();
                Token::mod_token()
            }
            '&' => {
                self.stream.next();
                Token::band_token()
            }
            '|' => {
                self.stream.next();
                Token::bor_token()
            }
            '#' => {
                self.stream.next();
                Token::len_token()
            }
            ':' => {
                self.stream.next();
                let next_char = self.stream.peek();
                if next_char == ':' {
                    self.stream.next();
                    Token::label_token()
                } else {
                    Token::colon_token()
                }
            }
            '/' => {
                self.stream.next();
                let next_char = self.stream.peek();
                if next_char == '/' {
                    self.stream.next();
                    Token::idiv_token()
                } else {
                    Token::div_token()
                }
            }
            '~' => {
                self.stream.next();
                let next_char = self.stream.peek();
                if next_char == '=' {
                    self.stream.next();
                    Token::not_eqaul_token()
                } else {
                    Token::wave_token()
                }
            }
            '=' => {
                self.stream.next();
                let next_char = self.stream.peek();
                if next_char == '=' {
                    self.stream.next();
                    Token::equal_token()
                } else {
                    Token::assign_token()
                }
            }
            '<' => {
                self.stream.next();
                let next_char = self.stream.peek();
                if next_char == '=' {
                    self.stream.next();
                    Token::le_token()
                } else if next_char == '<' {
                    self.stream.next();
                    Token::shl_token()
                } else {
                    Token::lt_token()
                }
            }
            '>' => {
                self.stream.next();
                let next_char = self.stream.peek();
                if next_char == '=' {
                    self.stream.next();
                    Token::ge_token()
                } else if next_char == '>' {
                    self.stream.next();
                    Token::shr_token()
                } else {
                    Token::gt_token()
                }
            }
            '.' => {
                self.stream.next();
                let next_char = self.stream.peek();
                if next_char == '.' {
                    self.stream.next();
                    let third_char = self.stream.peek();
                    if third_char == '.' {
                        self.stream.next();
                        Token::vararg_token()
                    } else {
                        Token::concat_token()
                    }
                } else {
                    // TODO: next can't be digit
                    Token::dot_token()
                }
            }
            '[' => {
                // self.stream.next();
                let next_char = self.stream.peek2();
                if next_char == '[' || next_char == '=' {
                    // self.stream.next();
                    // Token::not_eqaul_token()
                    self.parse_long_string()
                } else {
                    Token::open_bracket_token()
                }
            }
            '\'' | '"' => {
                // self.stream.next();
                self.parse_short_string()
            }
            c if is_digit(c) => self.parse_number(),
            c if is_letter(c) => self.parse_identifier(),
            _ => todo!(),
        }
    }

    /**
     * lua string literal
     * @see https://www.lua.org/manual/5.4/manual.html#3.1
     * @example
     * ```lua
     * a = [[alo
     *     123"]];
     * a = [==[
     * alo
     * 123"]==];
     * ```
     */
    fn parse_long_string(&mut self) -> Token {
        // skip string delimiter
        self.skip_specific_char('[');
        let c = self.stream.peek();
        if c == '[' {
            self.skip_specific_char('[');
            let mut long_string = String::new();
            while self.stream.peek() != ']'
                || (self.stream.peek() == '[' && self.stream.peek2() != ']')
            {
                long_string.push(self.stream.next());
            }
            Token::string_token(&long_string)
        } else if c == '=' {
            self.skip_specific_char('=');
            self.skip_specific_char('=');
            self.skip_specific_char('[');
            // TODO: implement
            Token::string_token("")
        } else {
            panic!(
                "Invalid long string delimiter near {}",
                self.stream.get_position()
            )
        }
    }

    fn parse_short_string(&mut self) -> Token {
        // quote is ' or "
        let quota = self.stream.next(); // eat ' or "
        let mut short_string = String::new();
        while self.stream.peek() != quota {
            if self.stream.peek() == '\n' {
                panic!(
                    "unfinished string near {} at {}",
                    short_string,
                    self.stream.get_position()
                );
            }
            short_string.push(self.stream.next());
        }
        self.stream.next(); // eat ' or "
        Token::string_token(&short_string)
    }

    /**
     * @example
     *  3   345   0xff   0xBEBADA
     * 3.0     3.1416     314.16e-2     0.31416E1     34e1
     * 0x0.1E  0xA23p-4   0X1.921FB54442D18P+1
     */
    fn parse_number(&mut self) -> Token {
        let mut number_string = String::new();

        let mut first_char = self.stream.next();
        number_string.push(first_char);
        let second_char = self.stream.peek();
        match second_char {
            c if is_digit(c) || c == '.' => {
                while is_digit(self.stream.peek()) {
                    number_string.push(self.stream.next());
                }
                if self.stream.peek() == '.' {
                    number_string.push(self.stream.next()); // eat .

                    while is_digit(self.stream.peek()) {
                        number_string.push(self.stream.next());
                    }
                    Token::number_token(&number_string)
                } else {
                    Token::number_token(&number_string)
                }
            }
            c if c == 'x' && first_char == '0' => {
                number_string.push(self.stream.next()); // eat x
                while is_hex_digit(self.stream.peek()) {
                    number_string.push(self.stream.next());
                }
                Token::number_token(&number_string)
            }
            _ => Token::number_token(&number_string),
        }
    }

    fn parse_identifier(&mut self) -> Token {
        let mut identifier_string = String::new();
        identifier_string.push(self.stream.next());
        let mut letter = self.stream.peek();
        while is_letter(letter) || letter == '-' {
            self.stream.next();
            identifier_string.push(letter);
            letter = self.stream.peek();
        }

        match &identifier_string[..] {
            "break" => Token::break_token(),
            "do" => Token::do_token(),
            "else" => Token::else_token(),
            "elseif" => Token::elseif_token(),
            "end" => Token::end_token(),
            "false" => Token::false_token(),
            "for" => Token::for_token(),
            "function" => Token::function_token(),
            "goto" => Token::goto_token(),
            "if" => Token::if_token(),
            "in" => Token::in_token(),
            "local" => Token::local_token(),
            "nil" => Token::nil_token(),
            "repeat" => Token::repeat_token(),
            "return" => Token::return_token(),
            "then" => Token::then_token(),
            "true" => Token::true_token(),
            "until" => Token::until_token(),
            "while" => Token::while_token(),
            _ => Token::identifier_token(&identifier_string),
        }
    }

    fn skip_specific_char(&mut self, target_char: char) -> () {
        let char_should_skip = self.stream.peek();
        if char_should_skip == target_char {
            self.stream.next();
        } else {
            panic!(
                "Expecting charactor {}, but got {} as {}",
                target_char,
                char_should_skip,
                self.stream.get_position()
            )
        }
    }

    fn skip_white_space(&mut self) -> () {
        loop {
            if self.eof() {
                break;
            }
            let char = self.stream.peek();
            if is_whitespace(char) {
                self.stream.next();
            } else {
                break;
            }
        }
    }

    fn eof(&self) -> bool {
        return self.stream.eof();
    }
}

#[test]
fn test_parse_long_string() {
    let mut lexer = Lexer {
        stream: ChunkStream {
            chunk_name: String::from("test.lua"),
            chunk: String::from("[[line 1\nline 2]]").chars().collect(),
            line: 1,
            column: 0,
            index: 0,
        },
    };

    assert_eq!(lexer.next_token(), Token::string_token("line 1\nline 2"));
}

#[test]
fn test_parse_short_string() {
    let mut lexer = Lexer {
        stream: ChunkStream {
            chunk_name: String::from("test.lua"),
            chunk: String::from("'short string'\n\"long string\"")
                .chars()
                .collect(),
            line: 1,
            column: 0,
            index: 0,
        },
    };

    assert_eq!(lexer.next_token(), Token::string_token("short string"));
    assert_eq!(lexer.next_token(), Token::string_token("long string"));
}

#[test]
fn test_parse_oparetor() {
    let mut lexer = Lexer {
        stream: ChunkStream {
            chunk_name: String::from("test.lua"),
            chunk: String::from("+-*/^%&|#~~=>=>>><=<<<").chars().collect(),
            line: 1,
            column: 0,
            index: 0,
        },
    };

    assert_eq!(lexer.next_token(), Token::plus_token());
    assert_eq!(lexer.next_token(), Token::minus_token());
    assert_eq!(lexer.next_token(), Token::mul_token());
    assert_eq!(lexer.next_token(), Token::div_token());
    assert_eq!(lexer.next_token(), Token::pow_token());
    assert_eq!(lexer.next_token(), Token::mod_token());
    assert_eq!(lexer.next_token(), Token::band_token());
    assert_eq!(lexer.next_token(), Token::bor_token());
    assert_eq!(lexer.next_token(), Token::len_token());
    assert_eq!(lexer.next_token(), Token::wave_token());
    assert_eq!(lexer.next_token(), Token::not_eqaul_token());
    assert_eq!(lexer.next_token(), Token::ge_token());
    assert_eq!(lexer.next_token(), Token::shr_token());
    assert_eq!(lexer.next_token(), Token::gt_token());
    assert_eq!(lexer.next_token(), Token::le_token());
    assert_eq!(lexer.next_token(), Token::shl_token());
    assert_eq!(lexer.next_token(), Token::lt_token());
}

#[test]
fn test_parse_digit() {
    let mut lexer = Lexer {
        stream: ChunkStream {
            chunk_name: String::from("test.lua"),
            chunk: String::from("0 3 345 0xff 0xBEBADA 3.0 3.1416")
                .chars()
                .collect(),
            line: 1,
            column: 0,
            index: 0,
        },
    };

    assert_eq!(lexer.next_token(), Token::number_token("0"));
    assert_eq!(lexer.next_token(), Token::number_token("3"));
    assert_eq!(lexer.next_token(), Token::number_token("345"));
    assert_eq!(lexer.next_token(), Token::number_token("0xff"));
    assert_eq!(lexer.next_token(), Token::number_token("0xBEBADA"));
    assert_eq!(lexer.next_token(), Token::number_token("3.0"));
    assert_eq!(lexer.next_token(), Token::number_token("3.1416"));
}

#[test]
fn test_parse_identifier() {
    let mut lexer = Lexer {
        stream: ChunkStream {
            chunk_name: String::from("test.lua"),
            chunk: String::from("if true then else end function() end")
                .chars()
                .collect(),
            line: 1,
            column: 0,
            index: 0,
        },
    };

    assert_eq!(lexer.next_token(), Token::if_token());
    assert_eq!(lexer.next_token(), Token::true_token());
    assert_eq!(lexer.next_token(), Token::then_token());
    assert_eq!(lexer.next_token(), Token::else_token());
    assert_eq!(lexer.next_token(), Token::end_token());
    assert_eq!(lexer.next_token(), Token::function_token());
    assert_eq!(lexer.next_token(), Token::open_paren_token());
    assert_eq!(lexer.next_token(), Token::close_paren_token());
    assert_eq!(lexer.next_token(), Token::end_token());
}
