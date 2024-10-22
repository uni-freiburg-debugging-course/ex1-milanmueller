// Disclaimer: I am not an experienced rust programer,
// I will attempt to learn the language in this course.
// Time will tell if this will turn out to be a good idea ^^
mod parser;
mod tokenizer;

use parser::Parser;
use tokenizer::Tokenizer;

fn main() {
    let my_input = "(simplify (- 123 -12))";
    let mut tokenizer = Tokenizer::new(my_input);
    // loop {
    //     match tokenizer.next() {
    //         Some(t) => println!("{:?}", t),
    //         None => break,
    //     }
    // }
    let mut parser = Parser::new(tokenizer);
    match parser.parse_expr() {
        Ok(node) => println!("{:?}", node),
        Err(e) => panic!("{:?}", e),
    }
}
