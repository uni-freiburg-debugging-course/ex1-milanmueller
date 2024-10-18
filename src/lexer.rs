#[derive(Debug)]
pub struct Number {
    pub base: u32,
    pub values: Vec<u32>,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum Token {
    Numeral(Number),
    Identifier(String),
    Operator(Operator),
    LParen,
    RParen,
}

pub struct Tokenizer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer { input, position: 0 }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input.chars().nth(self.position) {
            Some(c) => match c {
                '1'..='9' => {
                    let mut number = Number {
                        base: 10,
                        values: vec![c.to_digit(10).unwrap()],
                    };
                    self.position += 1;
                    while let Some(c) = self.input.chars().nth(self.position) {
                        if c.is_digit(10) {
                            number.values.push(c.to_digit(10).unwrap());
                            self.position += 1;
                        } else {
                            break;
                        }
                    }
                    return Some(Token::Numeral(number));
                }
                '+' => {
                    self.position += 1;
                    return Some(Token::Operator(Operator::Plus));
                }
                '-' => {
                    self.position += 1;
                    return Some(Token::Operator(Operator::Minus));
                }
                '*' => {
                    self.position += 1;
                    return Some(Token::Operator(Operator::Multiply));
                }
                '/' => {
                    self.position += 1;
                    return Some(Token::Operator(Operator::Divide));
                }
                '(' => {
                    self.position += 1;
                    return Some(Token::LParen);
                }
                ')' => {
                    self.position += 1;
                    return Some(Token::RParen);
                }
                ' ' => {
                    self.position += 1;
                    return self.next();
                }
                _ if c.is_alphabetic() => {
                    let identifier_start = self.position;
                    self.position += 1;
                    while let Some(c) = self.input.chars().nth(self.position) {
                        if c.is_alphanumeric() {
                            self.position += 1;
                        } else {
                            break;
                        }
                    }
                    let identifier_end = self.position;
                    return Some(Token::Identifier(
                        self.input[identifier_start..identifier_end].to_string(),
                    ));
                }
                _ => {
                    self.position += 1;
                    return None;
                }
            },
            None => {
                return None;
            }
        }
    }
}
