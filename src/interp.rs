use std::fmt::format;

use crate::parser::{Expression, ExpressionVisitor, ExpressionVisitResult};
use crate::tokenizer::Token;

#[derive(Clone, PartialEq, Debug)]
enum ValueVariant {
    String(String),
    Integer(i64),
    Float(f64)
}

pub struct Interpreter {
    values_stack: Vec<ValueVariant>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            values_stack : vec![]
        }
    } 
    pub fn eval(&mut self, expr : Box<dyn Expression>) -> f64 {
        expr.accept(self).unwrap();
        let result = self.values_stack.pop().unwrap();
        match result {
            ValueVariant::Float(f) => {
                return f;
            }
            _ => {
                panic!("result should be float");
            }
        }
    }
}

impl ExpressionVisitor for Interpreter {
    fn visit_float_expression(&mut self, expr: &crate::parser::FloatExpression) -> ExpressionVisitResult {
        self.values_stack.push(ValueVariant::Float(expr.f));
        Ok(())
    }

    fn visit_unary_expression(&mut self, un_expr: &crate::parser::UnaryExpression) -> ExpressionVisitResult {
        un_expr.expr.accept(self)?;
        let op = un_expr.op.clone();
        if let Some(val) = self.values_stack.pop() {
            match val {
                ValueVariant::Float(f) => {
                    match op {
                        Token::Plus => {
                            self.values_stack.push(ValueVariant::Float(f));  
                            Ok(())          
                        } 
                        Token::Minus => {
                            self.values_stack.push(ValueVariant::Float(-f));            
                            Ok(())
                        }
                        _ => {
                            Err(format!("unsupported unary op {}", op.to_string()))    
                        }
                    }
                },
                _ => {
                    Err("for now unary operation supported only with float types".to_string())    
                }
            }
        } else {
            Err("empty stack in unary expression".to_string())  
        }

    }

    fn visit_binary_expression(&mut self, expr: &crate::parser::BinaryExpression) -> ExpressionVisitResult {
        expr.left.accept(self)?;
        expr.right.accept(self)?;
        let op = expr.op.clone();
        if let (Some(r), Some(l)) = (self.values_stack.pop(), self.values_stack.pop()) {
            match (l, r) {
                (ValueVariant::Float(l_float), ValueVariant::Float(r_float)) => {
                    let mut res = 0.;
                    match op {
                        Token::Plus => res = l_float + r_float,
                        Token::Minus => res = l_float - r_float,
                        Token::Multi => res = l_float * r_float,
                        Token::Devide => res = l_float / r_float,
                        _ => {
                            return Err(format!("unsupported binary op {}", op.to_string()));
                        }
                    }
                    self.values_stack.push(ValueVariant::Float(res));
                    Ok(())
                }
                _ => {
                    return Err("for now binary operation supported only with float types".to_string());
                }
            }
        } else {
            return Err("empty stack in bynary expression".to_string());
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
        test_map.insert("10 / 5 + -3", -1.);
        for (prog, exp_res) in test_map.iter() {
            let prog = prog.to_string();
            let expr = Parser::new(&tokenize(&prog)).parse();
            let res = Interpreter::new().eval(expr);
            assert_eq!(res, *exp_res);
        }
    }
}