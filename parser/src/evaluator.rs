use lib::{ASTNode, Operator};

#[derive(Debug)]
pub enum EvaluationError {
    ExpectedNumeral,
}

pub fn evaluate(node: ASTNode) -> Result<ASTNode, EvaluationError> {
    match node {
        ASTNode::Numeral(_) => Ok(node),
        ASTNode::Operator(_, _, _) => Ok(node),
        ASTNode::Simplify(inner) => Ok(simplify(*inner)?),
    }
}

fn simplify(node: ASTNode) -> Result<ASTNode, EvaluationError> {
    match node {
        ASTNode::Operator(op, a, b) => match op {
            Operator::Add => Ok(add(evaluate(*a)?, evaluate(*b)?)?),
            Operator::Sub => Ok(sub(evaluate(*a)?, evaluate(*b)?)?),
            Operator::Mul => Ok(mul(evaluate(*a)?, evaluate(*b)?)?),
        },
        ASTNode::Numeral(_) => Ok(node),
        ASTNode::Simplify(node) => Ok(simplify(*node)?),
    }
}

fn add(a: ASTNode, b: ASTNode) -> Result<ASTNode, EvaluationError> {
    Ok(ASTNode::Numeral(evaluate_number(a)? + evaluate_number(b)?))
}

fn sub(a: ASTNode, b: ASTNode) -> Result<ASTNode, EvaluationError> {
    Ok(ASTNode::Numeral(evaluate_number(a)? - evaluate_number(b)?))
}

fn mul(a: ASTNode, b: ASTNode) -> Result<ASTNode, EvaluationError> {
    Ok(ASTNode::Numeral(evaluate_number(a)? * evaluate_number(b)?))
}

fn evaluate_number(node: ASTNode) -> Result<i32, EvaluationError> {
    match node {
        ASTNode::Numeral(num) => Ok(num),
        _ => Err(EvaluationError::ExpectedNumeral),
    }
}
