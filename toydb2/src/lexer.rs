use crate::token;

pub struct Lexer<'a> {
    input: &'a str,
    len: usize,
    position: usize,
    peek_position: usize,
    ch: Option<char>,
    line: usize,
    token: token::Token,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            len: input.len(),
            position: 0,
            peek_position: 0,
            ch: None,
            line: 1,
            token: token::Token::new(token::TokenKind::EOF, 0),
        };
        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        if self.len <= self.peek_position {
            self.ch = None;
        } else {
            self.ch = self.input.chars().nth(self.peek_position);
        }
        self.position = self.peek_position;
        self.peek_position += 1;
    }

    fn char(&self) -> Option<char> {
        self.ch
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.peek_position)
    }

    fn skip_space(&mut self) {
        while self.ch == Some(' ')
            || self.ch == Some('\n')
            || self.ch == Some('\r')
            || self.ch == Some('\t')
        {
            if self.ch == Some('\n') {
                self.line += 1;
            }
            self.read_char();
        }
    }

    // After remove pub
    pub fn next_token(&mut self) -> &token::Token {
        self.skip_space();

        let tok = match self.ch {
            Some(';') => token::Token::new(token::TokenKind::Symbol(token::SEMICOLON), self.line),
            Some('*') => token::Token::new(token::TokenKind::Symbol(token::ASTERISK), self.line),
            Some(',') => token::Token::new(token::TokenKind::Symbol(token::COMMA), self.line),
            Some('(') => token::Token::new(token::TokenKind::Symbol(token::LPAREN), self.line),
            Some(')') => token::Token::new(token::TokenKind::Symbol(token::RPAREN), self.line),
            None => token::Token::new(token::TokenKind::EOF, self.line),
            Some(_) => {
                if is_digit(self.char(), self.peek_char()) {
                    let mut number = if char_is(self.char(), Some('-')) {
                        self.read_char();
                        -char2num(self.char())
                    } else {
                        char2num(self.char())
                    };
                    while is_digit(self.peek_char(), None) {
                        number = number * 10 + char2num(self.peek_char());
                        self.read_char();
                    }
                    token::Token::new(token::TokenKind::Number(number), self.line)
                } else if self.ch == Some('\'') {
                    // String
                    let mut s = String::new();
                    self.read_char();
                    if char_is(self.char(), Some('\'')) {
                        token::Token::new(token::TokenKind::String("".to_string()), self.line)
                    } else {
                        while !char_is(self.peek_char(), Some('\''))
                            && !char_is(self.peek_char(), None)
                        {
                            s += &self.char().unwrap().to_string();
                            self.read_char();
                        }
                        s += &self.char().unwrap().to_string();
                        self.read_char();
                        token::Token::new(token::TokenKind::String(s), self.line)
                    }
                } else {
                    // Keyword
                    let mut s = {
                        if !self.char().unwrap().is_alphabetic() {
                            let invalid_chars = {
                                let mut s = self.char().unwrap().to_string();
                                while !char_is(self.peek_char(), None)
                                    && !self.peek_char().unwrap().is_alphabetic()
                                    && self.peek_char() != Some(' ')
                                {
                                    s += &self.peek_char().unwrap().to_string();
                                    self.read_char();
                                }
                                s
                            };
                            let emsg = format!(r#"error: Invalid character: "{}""#, invalid_chars);
                            self.read_char();
                            self.token =
                                token::Token::new(token::TokenKind::Illegal(emsg), self.line);
                            return &self.token;
                        }
                        self.char().unwrap().to_string()
                    };
                    while !char_is(self.peek_char(), None)
                        && self.peek_char().unwrap().is_alphabetic()
                    {
                        s += &self.peek_char().unwrap().to_string();
                        self.read_char();
                    }
                    if let Some(keyword) = token::is_keyword(&s) {
                        token::Token::new(token::TokenKind::Keyword(keyword), self.line)
                    } else {
                        token::Token::new(token::TokenKind::Identifier(s), self.line)
                    }
                }
            }
        };

        self.read_char();
        self.token = tok;
        &self.token
    }

    pub fn token(&self) -> &token::Token {
        &self.token
    }
}

fn is_digit(ch: Option<char>, next_ch: Option<char>) -> bool {
    match ch {
        Some('0'..='9') => true,
        Some('-') => matches!(next_ch, Some('0'..='9')),
        _ => false,
    }
}

fn char_is(ch: Option<char>, is: Option<char>) -> bool {
    ch == is
}

fn char2num(ch: Option<char>) -> i64 {
    ch.unwrap() as i64 - 48
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_peek_char() {
        let input = "'OR 1=1";
        let mut l = Lexer::new(&input);

        assert_eq!(l.char(), Some('\''));
        assert_eq!(l.peek_char(), Some('O'));
        while l.char().is_some() {
            l.read_char();
        }
        assert_eq!(l.char(), None);
    }

    #[test]
    fn test_line() {
        let input = "1\n2\n\n3\n";
        let mut l = Lexer::new(&input);

        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Number(1), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Number(2), 2)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Number(3), 4)
        );
        assert_eq!(l.next_token(), &token::Token::new(token::TokenKind::EOF, 5));
    }

    #[test]
    fn test_is_digit() {
        assert_eq!(is_digit(Some('/'), None), false);
        assert_eq!(is_digit(Some('0'), None), true);
        assert_eq!(is_digit(Some('1'), None), true);
        assert_eq!(is_digit(Some('8'), None), true);
        assert_eq!(is_digit(Some('9'), None), true);
        assert_eq!(is_digit(Some(':'), None), false);
        assert_eq!(is_digit(Some('-'), Some('1')), true);
        assert_eq!(is_digit(Some('-'), Some('9')), true);
        assert_eq!(is_digit(Some('-'), Some('0')), true);
        assert_eq!(is_digit(Some('-'), Some('-')), false);
    }

    #[test]
    fn test_symbol_token() {
        let input = ";*,()";
        let mut l = Lexer::new(&input);

        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::SEMICOLON), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::ASTERISK), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::COMMA), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::LPAREN), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::RPAREN), 1)
        );
        assert_eq!(l.next_token(), &token::Token::new(token::TokenKind::EOF, 1));
    }

    #[test]
    fn test_number_token() {
        let input = "-1 -0 0 1 42 -100";
        let mut l = Lexer::new(&input);

        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Number(-1), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Number(-0), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Number(0), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Number(1), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Number(42), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Number(-100), 1)
        );
        assert_eq!(l.next_token(), &token::Token::new(token::TokenKind::EOF, 1));
    }

    #[test]
    fn test_string_token() {
        let input = "'string' '123' '''\"'";
        let mut l = Lexer::new(&input);

        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::String("string".to_string()), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::String("123".to_string()), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::String("".to_string()), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::String("\"".to_string()), 1)
        );
        assert_eq!(l.next_token(), &token::Token::new(token::TokenKind::EOF, 1));
    }

    #[test]
    fn test_keyword_token() {
        let input = "select from as table create insert into values int text none";
        let mut l = Lexer::new(&input);

        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::SELECT), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::FROM), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::AS), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::TABLE), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::CREATE), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::INSERT), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::INTO), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::VALUES), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::INT), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::TEXT), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::NONE), 1)
        );
        assert_eq!(l.next_token(), &token::Token::new(token::TokenKind::EOF, 1));
    }

    #[test]
    fn test_identifier_token() {
        let input = "id name user";
        let mut l = Lexer::new(&input);

        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Identifier("id".to_string()), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Identifier("name".to_string()), 1)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Identifier("user".to_string()), 1)
        );
        assert_eq!(l.next_token(), &token::Token::new(token::TokenKind::EOF, 1));
    }

    #[test]
    fn test_all_tokenize() {
        let input = r#"
            CREATE TABLE users (id INT, name TEXT);
            INSERT INTO users VALUES (1, 'Alice');
            SELECT id, name FROM users;"#;
        let mut l = Lexer::new(&input);

        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::CREATE), 2)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::TABLE), 2)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Identifier("users".to_string()), 2)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::LPAREN), 2)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Identifier("id".to_string()), 2)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::INT), 2)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::COMMA), 2)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Identifier("name".to_string()), 2)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::TEXT), 2)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::RPAREN), 2)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::SEMICOLON), 2)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::INSERT), 3)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::INTO), 3)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Identifier("users".to_string()), 3)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::VALUES), 3)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::LPAREN), 3)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Number(1), 3)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::COMMA), 3)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::String("Alice".to_string()), 3)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::RPAREN), 3)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::SEMICOLON), 3)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::SELECT), 4)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Identifier("id".to_string()), 4)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::COMMA), 4)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Identifier("name".to_string()), 4)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Keyword(token::FROM), 4)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Identifier("users".to_string()), 4)
        );
        assert_eq!(
            l.next_token(),
            &token::Token::new(token::TokenKind::Symbol(token::SEMICOLON), 4)
        );
        assert_eq!(l.next_token(), &token::Token::new(token::TokenKind::EOF, 4));
    }
}
