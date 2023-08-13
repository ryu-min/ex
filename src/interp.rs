use core::fmt;
use std::collections::HashMap;

use crate::parser::{Expression, ExpressionVisitor, ExpressionVisitResult};
use crate::tokenizer::Token;
use crate::ex_std::{FunctionRepository, IOFunctionRepo, StdFuncMap};
#[derive(Clone, PartialEq, Debug)]
pub enum ValueVariant {
    String(String),
    Integer(i64),
    Float(f64)
}
impl fmt::Display for ValueVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueVariant::String(s) => {
                write!(f, "{}", s)
            }
            ValueVariant::Integer(i) => {
                write!(f, "{}", i)
            }
            ValueVariant::Float(fl) => {
                write!(f, "{}", fl)
            }
        }
    }
}

type UserFuncMap = HashMap<String, Box<dyn Expression>>;

pub struct Interpreter {
    values_stack: Vec<ValueVariant>,
    var_maps: HashMap<String, ValueVariant>,
    std_funcs: StdFuncMap,
    user_funcs: UserFuncMap
}
type InterpResult = Result<(), String>;
impl Interpreter {
    pub fn new() -> Interpreter {
        let mut std_func_repos = Vec::new();
        std_func_repos.push(Box::new(IOFunctionRepo::new()));
        let mut std_fucs : StdFuncMap = StdFuncMap::new(); 
        for repo in std_func_repos.iter() {
            for (fname, f) in repo.get_functions() {
                std_fucs.insert(fname, f);
            }
        }
        return Interpreter {
            values_stack : vec![], 
            var_maps: HashMap::new(),
            std_funcs : std_fucs,
            user_funcs : HashMap::new()
        };
    } 
    pub fn interp_expr(&mut self, expr : Box<dyn Expression>) -> InterpResult {
        if let Err(err_msg) = expr.accept(self) {
            Err(err_msg)
        } else {
            Ok(())
        }
    }

    pub fn get_var_value(&mut self, name: String) -> Option<ValueVariant> {
        self.var_maps.get(&name).cloned()
    }
}

impl ExpressionVisitor for Interpreter {
    fn visit_float_literal_expression(&mut self, expr: &crate::parser::FloatLiteralExpression) -> ExpressionVisitResult {
        self.values_stack.push(ValueVariant::Float(expr.f));
        Ok(())
    }

    fn visit_string_literal_expression(&mut self, expr: &crate::parser::StringLiteralExpression) -> ExpressionVisitResult {
        self.values_stack.push(ValueVariant::String(expr.s.clone()));
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
                            return Err(format!("binary op {} not supported for float's", op.to_string()));
                        }
                    }
                    self.values_stack.push(ValueVariant::Float(res));
                }
                (ValueVariant::String(l_string), ValueVariant::String(r_string)) => {
                    match op {
                        Token::Plus => {
                            self.values_stack.push(ValueVariant::String(l_string + &r_string));
                        }
                        _ => {
                            return Err(format!("binary op {} not supported for strings", op.to_string()));
                        }
                    }
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

    fn visit_function_def_expression(&mut self, expr: &crate::parser::FunctionDefExpression) -> ExpressionVisitResult {
        self.user_funcs.insert(expr.name.clone(), Box::new(expr.clone()));
        return Ok(());
    }

    fn visit_function_call_expression(&mut self, expr: &crate::parser::FunctionCallExpression) -> ExpressionVisitResult {
        match self.std_funcs.get(&expr.name).cloned() {
            Some(f) => {
                let arg_count = expr.args.len();
                for arg_expr in expr.args.iter() {
                    arg_expr.accept(self)?;
                }
                let mut args : Vec<ValueVariant> = Vec::new();
                for _ in 0..arg_count {
                    if let Some(value) = self.values_stack.pop() {
                        args.insert(0, value);
                    } else {
                        return Err(String::from("exptected value in stack"));
                    }
                }
                let f_result = f(&args);
                match f_result {
                    Ok(f_return_value) => {
                        if let Some(val) = f_return_value {
                            self.values_stack.push(val);
                        }
                    }
                    Err(err_msg) => {
                        return Err(format!("Error with function {} : {}", &expr.name, err_msg));                
                    }
                }
                Ok(())
            }
            None => return Err(format!("Function {} not defined", &expr.name)),
        }
    }

    fn visit_statement_list_expression(&mut self, expr: &crate::parser::StatementListExpression) -> ExpressionVisitResult {
        for statement in expr.statement_list.iter() {
            statement.accept(self)?;
        }
        Ok(())
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
        test_map.insert("var a  = \"aa\" + \"bb\"", ValueVariant::String(String::from("aabb")));
        test_map.insert("var b  = \"aa\" \n\
                           var a = b + \"bb\" ", ValueVariant::String(String::from("aabb")));
        for (prog, exp_res) in test_map.iter() {
            let prog = prog.to_string();
            let expr = Parser::new(&tokenize(&prog)).parse().unwrap();
            let mut interp = Interpreter::new(); 
            interp.interp_expr(expr).unwrap();
            assert_eq!(interp.get_var_value("a".to_string()).unwrap(), *exp_res);
        }
    }

    #[test]
    fn function_call_test() {
        // let prog : String = "fn test(a, b) { \n\
        //                         write(a) \n\
        //                         write(b) \n\
        //                         }\n".to_string();

        let prog : String = "fn test(a) { \n\
                                write(a) \n\
                                return a\n\
                            }\n".to_string();
    
    
        let tokens = tokenize(&prog);
        for token in tokens.iter() {
            println!("{}", token.to_string());
        }

        let expr = Parser::new(&tokens).parse().unwrap();

        let mut interp = Interpreter::new(); 
        interp.interp_expr(expr).unwrap();
        assert_eq!(true, true);
    }
    
    
}
