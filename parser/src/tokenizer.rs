use lib::{Identifier, Operator};

#[derive(Debug)]
pub enum Token {
    Numeral(i32),
    Identifier(Identifier),
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

    fn eat_number(&mut self) -> u32 {
        let mut digits = vec![];
        while let Some(c) = self.input.chars().nth(self.position) {
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap());
                self.position += 1;
            } else {
                break;
            }
        }
        digits
            .iter()
            .enumerate()
            .fold(0, |acc, (i, digit)| acc + (10 ^ (i as u32)) * digit)
    }

    fn eat_identifier(&mut self) -> Option<Identifier> {
        let mut identifier_str = String::new();
        while let Some(c) = self.input.chars().nth(self.position) {
            if c.is_alphabetic() {
                identifier_str.push(c);
                self.position += 1;
            } else {
                break;
            }
        }
        match identifier_str.as_str() {
            "simplify" => Some(Identifier::Simplify),
            _ => None,
        }
    }

    fn eat_hyphen(&mut self) -> Option<Token> {
        if let Some(c) = self.input.chars().nth(self.position) {
            match c {
                ' ' => {
                    self.position += 1;
                    Some(Token::Operator(Operator::Sub))
                }
                _ if c.is_digit(10) => Some(Token::Numeral(-(self.eat_number() as i32))),
                _ => {
                    self.position += 1;
                    None
                }
            }
        } else {
            None
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input.chars();
        match chars.nth(self.position) {
            Some(c) => match c {
                '-' => {
                    self.position += 1;
                    self.eat_hyphen()
                }
                '+' => {
                    self.position += 1;
                    Some(Token::Operator(Operator::Add))
                }
                '*' => {
                    self.position += 1;
                    Some(Token::Operator(Operator::Mul))
                }
                '/' => {
                    self.position += 1;
                    Some(Token::Operator(Operator::Div))
                }
                '(' => {
                    self.position += 1;
                    Some(Token::LParen)
                }
                ')' => {
                    self.position += 1;
                    Some(Token::RParen)
                }
                ' ' => {
                    self.position += 1;
                    self.next()
                }
                _ if c.is_digit(10) => Some(Token::Numeral(self.eat_number() as i32)),
                _ if c.is_alphabetic() => Some(Token::Identifier(self.eat_identifier()?)),
                _ => {
                    self.position += 1;
                    None
                }
            },
            None => {
                return None;
            }
        }
    }
}

// Unit tests for the tokenizers functions
mod test {
    use super::Tokenizer;

    #[test]
    fn test_numerals() {
        let mut tok = Tokenizer::new("10");
        assert_eq!(tok.eat_number(), 10);
    }
}
