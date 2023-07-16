use crate::parser::{Expression, ExpressionVisitor};
use crate::tokenizer::Token;

pub struct Interpreter{
    expr : Box<dyn Expression>
}

impl Interpreter {
    pub fn new(expr : Box<dyn Expression>) -> Interpreter {
        Interpreter {
            expr : expr,
        }
    } 

    pub fn eval(&mut self) -> f64 {
        return self.expr.accept(self);
    }
}

impl ExpressionVisitor for Interpreter {
    fn visit_float_expression(&self, expr: &crate::parser::FloatExpression) -> f64 {
        return expr.f;
    }

    fn visit_binary_expression(&self, expr: &crate::parser::BinaryExpression) -> f64 {
        let l = expr.left.accept(self);
        let r = expr.right.accept(self);
        let op = expr.op.clone();
        match op {
            Token::Plus => return l + r,
            Token::Minus => return l - r,
            Token::Multi => return l * r,
            Token::Devide => return  l / r,
            _ => {
                panic!("unsupported op {}", op.to_string());
            }
        }
    }
}


mod tests {
    use super::*;
    use crate::parser::Parser;
    use crate::tokenizer::tokenize;
    #[test]
    fn parser_test() {
        let program1 = "(2 + 2) * 2".to_string();
        let program2 = "2 + 2 * 2 * 2".to_string();
        let expr1 = Parser::new(&tokenize(&program1)).parse();
        let expr2 = Parser::new(&tokenize(&program2)).parse();
        let res1 = Interpreter::new(expr1).eval();
        let res2 = Interpreter::new(expr2).eval();
        assert_eq!(res1, 8.);
        assert_eq!(res2, 10.);   
    }
}

