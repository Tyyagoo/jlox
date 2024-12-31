mod error;
mod lexer;
mod parser;
mod token;

use clap::Parser;
use clio::{Input, Result};
use lexer::Lexer;
use std::io::{self, BufRead, Read};

use parser::{AstPrinter, Expr::*};
use token::{Token, TokenType};

#[derive(Parser, Debug)]
#[command(name = "lox")]
struct Args {
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

fn main() -> Result<()> {
    let mut args = Args::parse();

    let expr = Binary(
        Box::new(Unary(
            Token::new(TokenType::Minus, "-", 1),
            Box::new(Literal(Token::new(TokenType::Num(123.0), "123", 1))),
        )),
        Token::new(TokenType::Star, "*", 1),
        Box::new(Grouping(Box::new(Literal(Token::new(
            TokenType::Num(45.67),
            "45.67",
            1,
        ))))),
    );

    println!("{}", AstPrinter::print(&expr));

    if args.input.is_std() {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        loop {
            match lines.next() {
                Some(line) => {
                    let source = line?;
                    let lexer = Lexer::new(source.as_str());
                    let tokens = lexer.tokenize();

                    for token in tokens {
                        print!("{:?} ", token);
                    }

                    println!();
                }

                None => {}
            }
        }
    } else {
        let mut source = String::new();
        args.input.read_to_string(&mut source)?;

        let lexer = Lexer::new(source.as_str());
        let tokens = lexer.tokenize();

        for token in tokens {
            print!("{:?} ", token);
        }

        println!();
    }

    Ok(())
}
