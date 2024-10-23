// Disclaimer: I am not an experienced rust programer,
// I will attempt to learn the language in this course.
// Time will tell if this will turn out to be a good idea ^^
mod evaluator;
mod parser;
mod tokenizer;

use clap::Parser;
use evaluator::evaluate;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};
use tokenizer::Tokenizer;

// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    // Input file (positional argument)
    #[arg(value_name = "FILE")]
    file: Option<String>,

    // Read from standard input
    #[arg(short = 'i', long)]
    stdin: bool,
}

fn main() {
    let args = Arguments::parse();

    // Determine the input source. If the `stdin` flag is set, we read from stdin.
    // Otherwise we read from a file that needs to be given
    let input: Box<dyn BufRead> = if args.stdin {
        Box::new(BufReader::new(io::stdin()))
    } else {
        match args.file {
            Some(filepath) => {
                let file = File::open(filepath).expect("Failed to open file");
                Box::new(BufReader::new(file))
            }
            None => {
                eprintln!("Error: Please provide input file, or use the `stdin` flag to parse standard input");
                std::process::exit(1);
            }
        }
    };

    // Read from standard input
    for line in input.lines() {
        match line {
            Ok(line) => {
                let tokenizer = Tokenizer::new(&line);
                let mut parser = parser::Parser::new(tokenizer);
                match parser.parse_expr() {
                    Ok(node) => {
                        match evaluate(node) {
                            Ok(res) => println!("{}", res),
                            Err(e) => eprintln!("{:?}", e),
                        };
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
            Err(e) => eprintln!("Error reading line: {}", e),
        }
    }
}
