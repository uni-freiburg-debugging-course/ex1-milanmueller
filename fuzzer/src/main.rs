use clap::Parser;
use lib::{ASTNode, Operator};
use rand::{seq::SliceRandom, Rng};

// For now, we use arrays to index our enums to randomly select an option
// TODO: find a better way to do this...
const OPERATOR_VARIANTS: &[Operator] = &[Operator::Add, Operator::Sub, Operator::Mul];

struct ExpressionGenerator {
    rng: rand::rngs::ThreadRng,
}

impl ExpressionGenerator {
    fn new() -> Self {
        ExpressionGenerator {
            rng: rand::thread_rng(),
        }
    }

    // fn generate_numeral(&mut self, min: i32, max: i32) -> ASTNode {
    //     ASTNode::Numeral(self.rng.gen_range(min..max + 1))
    // }

    fn generate_operator_expr(&mut self) -> ASTNode {
        let op = OPERATOR_VARIANTS.choose(&mut self.rng);
        // let x1 = self.generate_numeral(i32::MIN, i32::MAX);
        let x1: i32 = self.rng.gen_range(i32::MIN..i32::MAX);
        // to avoid overflows (and with that differences in result from z3),
        // the value of the second operand must be chosen such that the result
        // of the arithmetic operaton does not exceed the bounds of i32 range.
        let x2 = match op {
            Some(Operator::Add) => self
                .rng
                .gen_range(i32::MIN.saturating_sub(x1)..i32::MAX.saturating_sub(x1)),
            Some(Operator::Sub) => self
                .rng
                .gen_range(i32::MIN.saturating_add(x1)..i32::MAX.saturating_add(x1)),
            Some(Operator::Mul) => {
                if x1 >= 0 {
                    self.rng
                        .gen_range(i32::MIN.saturating_div(x1)..i32::MAX.saturating_div(x1))
                } else {
                    self.rng
                        .gen_range(i32::MAX.saturating_div(x1)..i32::MIN.saturating_div(x1))
                }
            }
            None => panic!("Choosing random operator failed."),
        };
        match op {
            Some(Operator::Add) => {
                assert!(x1 as i64 + x2 as i64 <= i32::MAX as i64);
                ASTNode::Operator(
                    Operator::Add,
                    Box::new(ASTNode::Numeral(x1)),
                    Box::new(ASTNode::Numeral(x2)),
                )
            }
            Some(Operator::Sub) => {
                assert!(x1 as i64 - x2 as i64 >= i32::MIN as i64);
                ASTNode::Operator(
                    Operator::Sub,
                    Box::new(ASTNode::Numeral(x1)),
                    Box::new(ASTNode::Numeral(x2)),
                )
            }
            Some(Operator::Mul) => {
                assert!(x1 as i64 * x2 as i64 >= i32::MIN as i64);
                assert!(x1 as i64 * x2 as i64 <= i32::MAX as i64);
                ASTNode::Operator(
                    Operator::Mul,
                    Box::new(ASTNode::Numeral(x1)),
                    Box::new(ASTNode::Numeral(x2)),
                )
            }
            None => panic!("Choosing random operator failed."),
        }
    }
}

// Command line arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, default_value_t = 1)]
    number: u32,
}

fn main() {
    let args = Arguments::parse();
    let mut expression_generator = ExpressionGenerator::new();
    for _ in 0..args.number {
        println!(
            "{}",
            ASTNode::Simplify(Box::new(expression_generator.generate_operator_expr()))
        )
    }
}
