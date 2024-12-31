#![allow(dead_code)]
#![allow(unused_imports)]

use std::marker::PhantomData;

use crate::token::{
    Token,
    TokenType::{self, *},
};

pub enum Literal<'a> {
    Num(f64),
    Str(&'a str),
    Bool(bool),
    Nil,
}

pub enum Expr<'a> {
    Literal(Literal<'a>),
    Unary(&'a Token<'a>, Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, &'a Token<'a>, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
}

pub struct AstPrinter<'a> {
    phantom: PhantomData<&'a str>,
}

impl<'a> AstPrinter<'a> {
    pub fn print(expr: &Expr<'a>) -> String {
        match expr {
            Expr::Literal(literal) => match literal {
                Literal::Num(num) => num.to_string(),
                Literal::Str(str) => str.to_string(),
                Literal::Bool(false) => String::from("false"),
                Literal::Bool(true) => String::from("true"),
                Literal::Nil => String::from("nil"),
            },
            Expr::Unary(Token { lexeme, .. }, inner) => {
                format!("({} {})", lexeme, Self::print(inner))
            }
            Expr::Binary(left, Token { lexeme, .. }, right) => {
                format!("({} {} {})", lexeme, Self::print(left), Self::print(right))
            }
            Expr::Grouping(expr) => {
                format!("(group {})", Self::print(expr))
            }
        }
    }
}

pub struct Parser<'a> {
    tokens: Vec<&'a Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<&'a Token<'a>>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Expr<'a>> {
        let mut expressions = Vec::new();

        while !self.is_eof() {
            expressions.push(self.expr());
        }

        expressions
    }

    fn expr(&mut self) -> Expr<'a> {
        self.eq()
    }

    fn eq(&mut self) -> Expr<'a> {
        let mut expr = self.cmp();

        while self.matches(&[BangEqual, EqualEqual]) {
            let op = self.previous();
            let right = self.cmp();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn cmp(&mut self) -> Expr<'a> {
        let mut expr = self.term();

        while self.matches(&[Less, LessEqual, Greater, GreaterEqual]) {
            let op = self.previous();
            let right = self.term();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn term(&mut self) -> Expr<'a> {
        let mut expr = self.factor();

        while self.matches(&[Plus, Minus]) {
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn factor(&mut self) -> Expr<'a> {
        let mut expr = self.unary();

        while self.matches(&[Star, Slash]) {
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr<'a> {
        if self.matches(&[Bang, Minus]) {
            let op = self.previous();
            let right = self.unary();
            Expr::Unary(op, Box::new(right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expr<'a> {
        let expr = match self.peek().t {
            TokenType::False => Expr::Literal(Literal::Bool(false)),
            TokenType::True => Expr::Literal(Literal::Bool(true)),
            TokenType::Nil => Expr::Literal(Literal::Nil),
            TokenType::Str(str) => Expr::Literal(Literal::Str(str)),
            TokenType::Num(num) => Expr::Literal(Literal::Num(num)),
            TokenType::LeftParen => {
                self.current += 1;
                let expr = self.expr();
                // FIXME: obviously don't panic on this shit.
                assert!(
                    self.matches(&[RightParen]),
                    "Expected ')' after expression."
                );
                self.current -= 1; // FIXME: dirty workaround
                Expr::Grouping(Box::new(expr))
            }
            _ => unimplemented!(),
        };

        self.current += 1;
        expr
    }

    fn matches(&mut self, types: &[TokenType]) -> bool {
        for t_ in types {
            if self.peek().t == *t_ {
                self.advance();
                return true;
            }
        }
        false
    }

    fn advance(&mut self) -> &'a Token<'a> {
        if !self.is_eof() {
            self.current += 1;
        }

        self.previous()
    }

    fn peek(&self) -> &'a Token<'a> {
        self.tokens.get(self.current).unwrap()
    }

    fn next(&self) -> &'a Token<'a> {
        self.tokens.get(self.current + 1).unwrap()
    }

    fn previous(&self) -> &'a Token<'a> {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn is_eof(&self) -> bool {
        self.peek().t == TokenType::EOF
    }
}
