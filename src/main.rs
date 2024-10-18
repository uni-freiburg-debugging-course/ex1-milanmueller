// Disclaimer: I am not an experienced rust programer,
// I will attempt to learn the language in this course.
// Time will tell if this will turn out to be a good idea ^^

mod lexer;

fn main() {
    let my_input = "(simplify (+ 123 12))";
    let mut tokenizer = lexer::Tokenizer::new(my_input);
    loop {
        match tokenizer.next() {
            Some(token) => {
                println!("{:?}", token);
            }
            None => {
                break;
            }
        }
    }
}
