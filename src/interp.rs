use std::collections::HashMap;

use crate::parser::{Expression, ExpressionVisitor, ExpressionVisitResult};
use crate::tokenizer::Token;

#[derive(Clone, PartialEq, Debug)]
pub enum ValueVariant {
    String(String),
    Integer(i64),
    Float(f64)
}

pub struct Interpreter {
    values_stack: Vec<ValueVariant>,
    var_maps: HashMap<String, ValueVariant>
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            values_stack : vec![], 
            var_maps: HashMap::new()
        }
    } 
    pub fn parse(&mut self, expr : Box<dyn Expression>) {
        expr.accept(self).unwrap();
    }

    pub fn get_var_value(&mut self, name: String) -> Option<ValueVariant> {
        self.var_maps.get(&name).cloned()
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
                        } 
                        Token::Minus => {
                            self.values_stack.push(ValueVariant::Float(-f));            
                        }
                        _ => {
                            return Err(format!("unsupported unary op {}", op.to_string()));
                        }
                    }
                },
                _ => {
                    return Err("for now unary operation supported only with float types".to_string());    
                }
            }
        } else {
            return Err("empty stack in unary expression".to_string()); 
        }
        Ok(())          

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
                }
                _ => {
                    return Err("for now binary operation supported only with float types".to_string());
                }
            }
        } else {
            return Err("empty stack in bynary expression".to_string());
        }
        Ok(())
    }

    fn visit_assignment_expression(&mut self, expr: &crate::parser::AssignmentExpression) -> ExpressionVisitResult {
        expr.value.accept(self)?;
        if let Some(value) = self.values_stack.pop() {
            self.var_maps.insert(expr.name.clone(), value);
            Ok(())
        } else {
            Err(String::from("no value for assgignment expression"))
        }
    }

    fn visit_statement_list_expression(&mut self, expr: &crate::parser::StatementListExpression) -> ExpressionVisitResult {
        for statement in expr.statement_list.iter() {
            statement.accept(self)?;
        }
        Ok(())
    }

    fn visit_name_expression(&mut self, expr: &crate::parser::NameExpression) -> ExpressionVisitResult {
        if let Some(value) = self.var_maps.get(&expr.name) {
            self.values_stack.push(value.clone());
            return Ok(());
        } else {
            return Err(format!("unknown name '{}'", &expr.name));
        }
    }

}


mod tests {
    use std::collections::HashMap;
    use crate::{parser::Parser, tokenizer::tokenize, interp::{Interpreter, ValueVariant}};
    #[test]
    fn iterp_test() {
        let mut test_map = HashMap::new();

        
        test_map.insert("var a = 2 + 2", ValueVariant::Float(4.0));
        test_map.insert("var a  =   (2 + 2) * 2", ValueVariant::Float(8.));
        test_map.insert("var a  = 2 + 2 * 2 * 2", ValueVariant::Float((10.)));
        test_map.insert("var b  = 0 - 3 \n\
                           var a = b - 1", ValueVariant::Float((-4.)));
        // test_map.insert("-2 + 2", 0.);
        // test_map.insert("-(2 + 2 * 2)", -6.);
        // test_map.insert("10 / 5 + -3 ", -1.);
        // test_map.insert("\n\
        //                    10 / 5 + -3 ", -1.);
        // test_map.insert("10 / 5 + \n\
        //                    -3 ", -1.);

        for (prog, exp_res) in test_map.iter() {
            let prog = prog.to_string();
            let expr = Parser::new(&tokenize(&prog)).parse().unwrap();
            let mut interp = Interpreter::new(); 
            interp.parse(expr);
            assert_eq!(interp.get_var_value("a".to_string()).unwrap(), *exp_res);
        }
    }
}