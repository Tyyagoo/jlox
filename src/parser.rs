use std::marker::PhantomData;

use crate::token::{Token, TokenType::*};
use Expr::*;
pub enum Expr<'a> {
    Literal(Token<'a>),
    Unary(Token<'a>, Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, Token<'a>, Box<Expr<'a>>),
    Grouping(Box<Expr<'a>>),
}

pub struct AstPrinter<'a> {
    phantom: PhantomData<&'a str>,
}

impl<'a> AstPrinter<'a> {
    pub fn print(expr: &Expr<'a>) -> String {
        match expr {
            Literal(Token { lexeme, .. }) => lexeme.to_string(),
            Unary(Token { lexeme, .. }, inner) => {
                format!("({} {})", lexeme, Self::print(inner))
            }
            Binary(left, Token { lexeme, .. }, right) => {
                format!("({} {} {})", lexeme, Self::print(left), Self::print(right))
            }
            Grouping(expr) => {
                format!("(group {})", Self::print(expr))
            }
        }
    }
}
