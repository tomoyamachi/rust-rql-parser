use crate::token;
use crate::token::Token;
use std::iter::Peekable;
use std::mem;
use std::str::Chars;

pub struct Lexer {
    input: String,
    // Current position in input (points to current char)
    position: usize,
    // current char under examination
    ch: char,
    // Use `Chars` to support UTF-8.
    chars: Peekable<Chars<'static>>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let chars = unsafe { mem::transmute(input.chars().peekable()) };
        let mut lexer = Lexer {
            input,
            position: 0,
            ch: '\u{0}',
            chars,
        };
        lexer.read_char();
        lexer
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok: Token;
        match self.ch {
            '(' => {
                tok = Token::Lparen;
            }
            ')' => {
                tok = Token::Rparen;
            }
            ',' => {
                tok = Token::Comma;
            }
            '+' => {
                tok = Token::Plus;
            }
            '-' => {
                tok = Token::Minus;
            }
            '"' => {
                tok = Token::Str(self.read_string().to_string());
            }
            '\u{0}' => {
                tok = Token::Eof;
            }
            _ => {
                if is_letter(self.ch) {
                    let ident = self.read_identifier();
                    return token::lookup_ident(ident);
                } else if is_digit(self.ch) {
                    let integer_part = self.read_number().to_string();
                    if self.ch == '.' && is_digit(self.peek_char()) {
                        self.read_char();
                        let fractional_part = self.read_number();
                        return Token::Float(format!("{}.{}", integer_part, fractional_part));
                    } else {
                        return Token::Int(integer_part);
                    }
                } else {
                    tok = Token::Illegal
                }
            }
        }

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        // The first character needs to be a letter.
        if is_letter(self.ch) {
            self.read_char();
        }
        // The second character and after can be a letter or a digit.
        while is_letter(self.ch) || is_digit(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn read_string(&mut self) -> &str {
        let position = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\u{0}' {
                break;
            }
        }
        &self.input[position..self.position]
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) {
            self.read_char();
        }
    }

    // -- Low-level methods that touches the `Chars`.

    fn read_char(&mut self) {
        self.position += if self.ch == '\u{0}' {
            0
        } else {
            self.ch.len_utf8()
        };
        self.ch = self.chars.next().unwrap_or('\u{0}');
    }

    fn peek_char(&mut self) -> char {
        self.chars.peek().cloned().unwrap_or('\u{0}')
    }
}

fn is_letter(ch: char) -> bool {
    ch == '_'
        // propertyにperiodも入るため、文字列判別
        || ch == '.'
        || ch == '$'
        // 漢字も含まれる
        || ch.is_alphabetic()
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r'
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::token::Token;

    #[test]
    fn next_token() {
        let input = r#"and(eq(foo,"test"),or(gt(bar.baz,100),ge(test,60.0))"#;
        let tests = [
            Token::And,
            Token::Lparen,
            Token::Eq,
            Token::Lparen,
            Token::Ident("foo".to_string()),
            Token::Comma,
            Token::Str("test".to_string()),
            Token::Rparen,
            Token::Comma,
            Token::Or,
            Token::Lparen,
            Token::Gt,
            Token::Lparen,
            Token::Ident("bar.baz".to_string()),
            Token::Comma,
            Token::Int("100".to_string()),
            Token::Rparen,
            Token::Comma,
            Token::Ge,
            Token::Lparen,
            Token::Ident("test".to_string()),
            Token::Comma,
            Token::Float("60.0".to_string()),
            Token::Rparen,
            Token::Rparen,
        ];

        let mut lexer = Lexer::new(input.to_owned());

        for (i, expected_token) in tests.iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(&token, expected_token, "tests[{}]", i);
        }
    }
}
