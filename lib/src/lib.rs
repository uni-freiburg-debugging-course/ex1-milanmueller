use core::fmt;

#[derive(Debug)]
pub enum Identifier {
    Simplify,
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
}
#[derive(Debug)]
pub enum ASTNode {
    Numeral(i32),
    Operator(Operator, Box<ASTNode>, Box<ASTNode>),
    Simplify(Box<ASTNode>),
}

impl ASTNode {
    fn print_rec(&self) -> String {
        match self {
            Self::Numeral(number) => format!("{}", number),
            Self::Operator(op, a, b) => {
                let op_str = match op {
                    Operator::Add => "+",
                    Operator::Sub => "-",
                    Operator::Mul => "*",
                };
                format!("({} {} {})", op_str, a.print_rec(), b.print_rec())
            }
            Self::Simplify(inner) => format!("(simplify {})", inner),
        }
    }
}

impl fmt::Display for ASTNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Numeral(number) => {
                if *number < 0 {
                    write!(f, "(- {})", -number)
                } else {
                    write!(f, "{}", self.print_rec())
                }
            }
            _ => write!(f, "{}", self.print_rec()),
        }
    }
}
