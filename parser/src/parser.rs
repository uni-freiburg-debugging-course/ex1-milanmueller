/*
Implement the following grammar
<numeral> ::= 0 | [1-9][0-9]*
<operator> ::= + | - | * | /
<expr> ::= ( <expr> ) | (operator <expr> <expr> ) | ( simplify <expr> )
*/
use crate::tokenizer::{Token, Tokenizer};
use core::fmt;
use lib::{Identifier, Operator};
use std::iter::Peekable;

// Define Enums and how to print them
#[derive(Debug)]
pub enum ASTNode {
    Numeral(i32),
    Operator(Operator, Box<ASTNode>, Box<ASTNode>),
    Simplify(Box<ASTNode>),
}

impl ASTNode {
    fn print(&self) -> String {
        match self {
            Self::Numeral(number) => format!("{}", number),
            Self::Operator(op, a, b) => {
                let op_str = match op {
                    Operator::Add => "+",
                    Operator::Sub => "-",
                    Operator::Mul => "*",
                    Operator::Div => "/",
                };
                format!("{} {} {}", op_str, (*a).print(), (*b).print())
            }
            Self::Simplify(inner) => format!("simplify ({})", (*inner).print()),
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.print())
    }
}

pub enum ParseError {
    ExpectedNumeral(Option<Token>),
    ExpectedIdentifier(Option<Token>),
    ExpectedOperator(Option<Token>),
    ExpectedTerm(Option<Token>),
    UnexpectedToken(Option<Token>),
}

// Custom Error Messages
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ExpectedNumeral(e) => match e {
                Some(t) => write!(f, "Expected numeral but found {:?}", t),
                None => write!(f, "Expected numeral but did not find token"),
            },
            ParseError::ExpectedIdentifier(e) => match e {
                Some(t) => write!(f, "Expected identifier but found {:?}", t),
                None => write!(f, "Expected identifier but did not find token"),
            },
            ParseError::ExpectedOperator(e) => match e {
                Some(t) => write!(f, "Expected operator but found {:?}", t),
                None => write!(f, "Expected operator but did not find token"),
            },
            ParseError::ExpectedTerm(e) => match e {
                Some(t) => write!(f, "Expected term but found {:?}", t),
                None => write!(f, "Expected term but did not find token"),
            },
            ParseError::UnexpectedToken(e) => match e {
                Some(t) => write!(f, "Unexpected token {:?}", t),
                None => write!(f, "Expected token"),
            },
        }
    }
}

// Actual implementation of the parser
pub struct Parser<'a> {
    tokenizer: Peekable<Tokenizer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokenizer: Tokenizer<'a>) -> Self {
        Parser {
            tokenizer: tokenizer.peekable(),
        }
    }

    fn parse_numeral(&mut self) -> Result<ASTNode, ParseError> {
        match self.tokenizer.next() {
            Some(Token::Numeral(numeral)) => Ok(ASTNode::Numeral(numeral)),
            Some(t) => Err(ParseError::ExpectedNumeral(Some(t))),
            None => Err(ParseError::ExpectedNumeral(None)),
        }
    }

    fn parse_identifier(&mut self) -> Result<ASTNode, ParseError> {
        match self.tokenizer.next() {
            Some(Token::Identifier(id)) => match id {
                Identifier::Simplify => Ok(ASTNode::Simplify(Box::new(self.parse_expr()?))),
            },
            Some(t) => Err(ParseError::ExpectedIdentifier(Some(t))),
            None => Err(ParseError::ExpectedIdentifier(None)),
        }
    }

    fn parse_operator(&mut self) -> Result<ASTNode, ParseError> {
        match self.tokenizer.next() {
            Some(Token::Operator(op)) => match op {
                Operator::Add => Ok(ASTNode::Operator(
                    Operator::Add,
                    Box::new(self.parse_expr()?),
                    Box::new(self.parse_expr()?),
                )),
                Operator::Sub => Ok(ASTNode::Operator(
                    Operator::Sub,
                    Box::new(self.parse_expr()?),
                    Box::new(self.parse_expr()?),
                )),
                Operator::Mul => Ok(ASTNode::Operator(
                    Operator::Mul,
                    Box::new(self.parse_expr()?),
                    Box::new(self.parse_expr()?),
                )),
                Operator::Div => Ok(ASTNode::Operator(
                    Operator::Div,
                    Box::new(self.parse_expr()?),
                    Box::new(self.parse_expr()?),
                )),
            },
            Some(t) => Err(ParseError::ExpectedOperator(Some(t))),
            None => Err(ParseError::ExpectedOperator(None)),
        }
    }

    pub fn parse_expr(&mut self) -> Result<ASTNode, ParseError> {
        match self.tokenizer.peek() {
            Some(Token::Numeral(_)) => self.parse_numeral(),
            Some(Token::Identifier(_)) => self.parse_identifier(),
            Some(Token::Operator(_)) => self.parse_operator(),
            Some(Token::RParen) => Err(ParseError::UnexpectedToken(Some(Token::RParen))),
            Some(Token::LParen) => {
                // "Eat" the left parenthesis
                self.tokenizer.next();
                match self.parse_expr() {
                    // The expression is only valid if we find closing parenthesis
                    Ok(node) => match self.tokenizer.next() {
                        Some(Token::RParen) => Ok(node),
                        Some(t) => Err(ParseError::UnexpectedToken(Some(t))),
                        None => Err(ParseError::ExpectedTerm(None)),
                    },
                    Err(e) => Err(e),
                }
            }
            None => Err(ParseError::ExpectedTerm(None)),
        }
    }
}
