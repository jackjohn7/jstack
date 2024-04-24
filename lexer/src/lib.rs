pub mod tokens;

use tokens::{lookup_keyword, Token, TokenKind, Value};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut result = Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        result.read_char();
        result
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input.as_bytes()[self.read_position]
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.ch {
            b'+' => tokens::Token {
                location: self.position,
                kind: TokenKind::Add,
            },
            b';' => tokens::Token {
                location: self.position,
                kind: TokenKind::Semicolon,
            },
            b'(' => tokens::Token {
                location: self.position,
                kind: TokenKind::LeftParen,
            },
            b')' => tokens::Token {
                location: self.position,
                kind: TokenKind::RightParen,
            },
            b'#' => tokens::Token {
                location: self.position,
                kind: TokenKind::Pound,
            },
            0 => Token {
                location: self.position,
                kind: TokenKind::EOF,
            },
            a => {
                if a.is_ascii_alphabetic() {
                    let ident = self.read_identifier();
                    match lookup_keyword(ident.clone()) {
                        Some(t) => Token {
                            location: self.position,
                            kind: t,
                        },
                        None => Token {
                            location: self.position,
                            kind: ident,
                        },
                    }
                } else if a.is_ascii_digit() {
                    // read number
                    match self.read_number() {
                        Ok(num) => Token {
                            location: self.position,
                            kind: num,
                        },
                        Err(msg) => {
                            panic!("Error parsing number: {}, line: {}", msg, self.position)
                        }
                    }
                } else {
                    Token {
                        location: self.position,
                        kind: TokenKind::ILLEGAL,
                    }
                }
            }
        }
    }

    fn read_identifier(&mut self) -> TokenKind {
        let position = self.position;
        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }
        TokenKind::Ident(self.input[position..self.position].to_owned())
    }

    fn read_number(&mut self) -> Result<TokenKind, &'static str> {
        let position = self.position;
        loop {
            match self.ch {
                b'0'..=b'9' => self.read_char(),
                b'.' => match self.read_float(position) {
                    Ok(f) => return Ok(f),
                    Err(msg) => return Err(msg),
                },
                _ => break,
            }
        }

        match i32::from_str_radix(&self.input[position..self.position].to_owned(), 10) {
            Ok(val) => Ok(TokenKind::Value(Value::Int(val))),
            Err(msg) => {
                println!("{}", msg.to_string());
                Err("Failed to parse integer")
            }
        }
    }

    fn read_float(&mut self, start_position: usize) -> Result<TokenKind, &'static str> {
        loop {
            match self.ch {
                b'0'..=b'9' => self.read_char(),
                _ => break,
            }
        }

        match (self.input[start_position..self.position].to_owned()).parse::<f32>() {
            Ok(f) => Ok(TokenKind::Value(Value::Float(f))),
            Err(msg) => {
                println!("{}", msg.to_string());
                Err("Failed to parse float")
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}
