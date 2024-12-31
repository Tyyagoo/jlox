mod error;
mod lexer;
mod token;

use clap::Parser;
use clio::{Input, Result};
use lexer::Lexer;
use std::io::{self, BufRead, Read};

#[derive(Parser, Debug)]
#[command(name = "lox")]
struct Args {
    #[clap(value_parser, default_value = "-")]
    input: Input,
}

fn main() -> Result<()> {
    let mut args = Args::parse();

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
