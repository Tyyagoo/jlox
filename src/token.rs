#![allow(dead_code)]
#![allow(unused_imports)]

use std::fmt::{self, Display};

use TokenType::*;

#[derive(Debug)]
pub enum TokenType<'a> {
    // Single-char tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // +1 char tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals
    Id, Str(&'a str), Num(f64),

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    EOF
}

#[derive(Debug)]
pub struct Token<'a> {
    t: TokenType<'a>,
    lexeme: &'a str,
    line: usize
}

impl<'a> Token<'a> {
    pub fn new(t: TokenType<'a>, lexeme: &'a str, line: usize) -> Self {
        Self { t, lexeme, line }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?} {}", self.t, self.lexeme)
    }
}