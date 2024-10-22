use core::fmt;

#[derive(Debug)]
pub enum Identifier {
    Simplify,
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mul,
}
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
