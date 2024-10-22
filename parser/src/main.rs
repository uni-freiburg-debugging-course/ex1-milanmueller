// Disclaimer: I am not an experienced rust programer,
// I will attempt to learn the language in this course.
// Time will tell if this will turn out to be a good idea ^^
mod evaluator;
mod parser;
mod tokenizer;

use evaluator::evaluate;
use parser::Parser;
use tokenizer::Tokenizer;

fn main() {
    let my_input = "(simplify (+ 1 1))";
    // let tokenizer = Tokenizer::new(my_input);
    // for token in tokenizer {
    //     println!("{:?}", token);
    // }
    let tokenizer = Tokenizer::new(my_input);
    let mut parser = Parser::new(tokenizer);
    match parser.parse_expr() {
        Ok(node) => {
            match evaluate(node) {
                Ok(res) => println!("{}", res),
                Err(e) => panic!("{:?}", e),
            };
        }
        Err(e) => panic!("{}", e),
    }
}
