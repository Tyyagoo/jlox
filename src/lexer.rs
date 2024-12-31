#![allow(dead_code)]
#![allow(unused_imports)]

use std::{ops::Index, ptr::null};

use crate::error::ErrorReporter;
use crate::token::{
    Token,
    TokenType::{self, *},
};

pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
    err: ErrorReporter,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            err: ErrorReporter::new(),
        }
    }

    pub fn tokenize(mut self) -> Vec<Token<'a>> {
        while !self.is_end_of_file() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(EOF, "", self.line));
        self.tokens
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let t = if self.take('=') { BangEqual } else { Equal };
                self.add_token(t)
            }
            '=' => {
                let t = if self.take('=') { EqualEqual } else { Equal };
                self.add_token(t)
            }
            '<' => {
                let t = if self.take('=') { LessEqual } else { Less };
                self.add_token(t)
            }
            '>' => {
                let t = if self.take('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(t)
            }
            '/' => {
                if self.take('/') {
                    while self.peek() != '\n' {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            '"' => self.string(),
            char if char::is_numeric(char) => self.number(),
            char if char::is_alphabetic(char) => self.identifier(),
            '\n' => {
                self.line += 1;
            }
            ' ' | '\r' | '\t' => {}
            reason => {
                self.err
                    .error(self.line, &format!("unexpected {}.", reason));
            }
        }
    }

    fn advance(&mut self) -> char {
        match self.source.chars().nth(self.current) {
            Some(char) => {
                self.current += 1;
                char
            }
            None => '\0',
        }
    }

    fn take(&mut self, what: char) -> bool {
        match self.source.chars().nth(self.current) {
            Some(char) if char == what => {
                self.current += 1;
                true
            }

            _ => false,
        }
    }

    fn peek(&self) -> char {
        match self.source.chars().nth(self.current) {
            Some(char) => char,
            None => '\0',
        }
    }

    fn peek_next(&self) -> char {
        match self.source.chars().nth(self.current + 1) {
            Some(char) => char,
            None => '\0',
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_end_of_file() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end_of_file() {
            self.err.error(self.line, "Unterminated string.");
            return;
        }

        self.advance();

        let str = &self.source[(self.start + 1)..(self.current - 1)];
        self.add_token(Str(str));
    }

    fn number(&mut self) {
        while char::is_numeric(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && char::is_numeric(self.peek_next()) {
            self.advance();
            while char::is_numeric(self.peek()) {
                self.advance();
            }
        }

        let num: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token(Num(num));
    }

    fn identifier(&mut self) {
        // TODO: handle underline
        while char::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let lexeme = &self.source[self.start..self.current];

        let t = match lexeme {
            "and" => And,
            "class" => Class,
            "else" => Else,
            "false" => False,
            "for" => For,
            "fun" => Fun,
            "if" => If,
            "nil" => Nil,
            "or" => Or,
            "print" => Print,
            "return" => Return,
            "super" => Super,
            "this" => This,
            "true" => True,
            "var" => Var,
            "while" => While,
            _ => Id,
        };

        self.tokens.push(Token::new(t, lexeme, self.line));
    }

    fn add_token(&mut self, t: TokenType<'a>) {
        let lexeme = &self.source[self.start..self.current];
        self.tokens.push(Token::new(t, lexeme, self.line));
    }

    fn is_end_of_file(&self) -> bool {
        self.current >= self.source.len()
    }
}
