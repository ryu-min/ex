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
                panic!("unsupported binary op {}", op.to_string());
            }
        }
    }

    fn visit_unary_expression(&self, un_expr: &crate::parser::UnaryExpression) -> f64 {
        let op = un_expr.op.clone();
        let expr_result = un_expr.expr.accept(self);
        match op {
            Token::Plus => return expr_result,
            Token::Minus => return -expr_result,
            _ => {
                panic!("unsupported unary op {}", op.to_string());    
            }
        }
    }
}


mod tests {
    use std::collections::HashMap;
    use crate::{parser::Parser, tokenizer::tokenize, interp::Interpreter};
    #[test]
    fn iterp_test() {
        let mut test_map = HashMap::new();
        test_map.insert("(2 + 2) * 2", 8.);
        test_map.insert("2 + 2 * 2 * 2", 10.);
        test_map.insert("-2 + 2", 0.);
        test_map.insert("-(2 + 2 * 2)", -6.);
        for (prog, exp_res) in test_map.iter() {
            let prog = prog.to_string();
            let expr = Parser::new(&tokenize(&prog)).parse();
            let res = Interpreter::new(expr).eval();
            assert_eq!(res, *exp_res);
        }
    }
}