pub mod tokens;

use tokens::{Token, TokenKind};

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
                    match lookup_keyword(&ident) {}
                } else if a.is_ascii_digit() {
                    // read number
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

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}
