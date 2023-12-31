use core::fmt;

use crate::ex_std::{FunctionRepository, IOFunctionRepo, StdFuncMap, StdMethodsMap, IntMethods, StdMethodsRepository, StringMethods};
use std::collections::HashMap;
use std::mem::Discriminant;

use super::{expressions::{FunctionDefExpression, Expression, ExpressionVisitResult, ExpressionVisitor}, tokenizer::Token};
#[derive(Clone, PartialEq, Debug)]
pub enum ValueVariant {
    String(String),
    Integer(i64),
    Float(f64), 
    Bool(bool)
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
            ValueVariant::Bool(b) => {
                write!(f, "{}", b)
            }
        }
    }
}

type UserFuncMap = HashMap<String, FunctionDefExpression>;
type ValueScope = HashMap<String, ValueVariant>;

pub struct Interpreter {
    values_stack: Vec<ValueVariant>,
    var_scopes: Vec<ValueScope>,
    std_funcs: StdFuncMap,
    std_methods: HashMap<Discriminant<ValueVariant>, StdMethodsMap>,
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

        let mut std_methods : HashMap<Discriminant<ValueVariant>,StdMethodsMap> = HashMap::new();
        let mut std_methods_repos: Vec<Box<dyn StdMethodsRepository>> = Vec::new();
        std_methods_repos.push(Box::new(IntMethods::new()));
        std_methods_repos.push(Box::new(StringMethods::new()));
        
        for methods_repo in std_methods_repos.iter() {
            std_methods.insert(methods_repo.get_diterminant(), methods_repo.get_methods());
        }


        let mut var_scopes : Vec<ValueScope> = Vec::new();
        // global value scope
        var_scopes.push(ValueScope::new());
        return Interpreter {
            values_stack : vec![], 
            var_scopes: var_scopes,
            std_funcs : std_fucs,
            std_methods : std_methods,
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

    pub fn _get_var_value(&mut self, name: &String) -> Option<ValueVariant> {
        assert!(self.var_scopes.len() >= 1);
        self.var_scopes.last().unwrap().get(name).cloned()
    }

    fn _add_var_scope(&mut self) {
        assert!(self.var_scopes.len() >= 1);
        self.var_scopes.push(ValueScope::new());
    }

    fn _drop_var_scope(&mut self) {
        assert!(self.var_scopes.len() >= 2);
        self.var_scopes.pop();
    }

    fn _get_var(&mut self, name: &String) -> Option<&mut ValueVariant> {
        assert!(self.var_scopes.len() >= 1);
        for scope in self.var_scopes.iter_mut().rev() {
            if let Some(res) = scope.get_mut(name) {
                return Some(res);
            }
        }
        None
    }

    fn add_var(&mut self, name: &String, var: &ValueVariant) {
        assert!(self.var_scopes.len() >= 1);
        let n = self.var_scopes.len();
        self.var_scopes[n - 1].insert(name.clone(), var.clone());
    }

    fn get_current_stack_value(&mut self) -> Result<ValueVariant, String> {
        if let Some(value) = self.values_stack.pop() {
            return Ok(value);
        }
        Err("Expected value on stack".to_string())
    }

    fn value_variant_to_int(&mut self, v : &ValueVariant) -> Result<i64,String> {
        match v {
            ValueVariant::Integer(i) => {
                Ok(*i)
            }
            _ => {
                Err("expected int".to_string())
            }
        }
    }

    fn call_std_func(&mut self, expr: &crate::ex_core::expressions::FunctionCallExpression) -> ExpressionVisitResult {
            
            let f = self.std_funcs.get(&expr.name).unwrap().clone();
            for arg_expr in expr.args.iter() {
                arg_expr.accept(self)?;
            }
            let mut args : Vec<ValueVariant> = Vec::new();
            let arg_count = &expr.args.len();
            for _ in 0..*arg_count {
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
        
    fn call_user_func(&mut self, expr: &crate::ex_core::expressions::FunctionCallExpression) -> ExpressionVisitResult {
        assert!(self.var_scopes.len() >= 1);
        self.var_scopes.push(ValueScope::new()); 
        let user_f = self.user_funcs.get(&expr.name).unwrap().clone();
        for arg in expr.args.iter() {
            arg.accept(self)?;
        }
        let mut user_args = user_f.args;
        
        user_args.reverse();
        for arg_name in user_args.iter() {
            if let Some(arg_value) = self.values_stack.pop() {
                let n = self.var_scopes.len();
                self.var_scopes[n - 1].insert(arg_name.clone(), arg_value);
            }
        }
        for expr in user_f.body.iter() {
            expr.accept(self)?;
        }
        self.var_scopes.pop();
        Ok(())
    }


    fn call_method(&mut self, this_value : &ValueVariant, method_name: &String, args: &Vec<Box<dyn Expression>>) -> ExpressionVisitResult {
        let discr = std::mem::discriminant(this_value);
        if !self.std_methods.contains_key(&discr) {
            return Err(format!("method for {} not supported", this_value.to_string()));
        }
        let methods_map = self.std_methods.get(&discr).unwrap().clone();
        if let Some(f) = methods_map.get(method_name) {
            for arg_expr in args.iter() {
                arg_expr.accept(self)?;
            }
            let mut parsed_args : Vec<ValueVariant> = Vec::new();
            let arg_count = &args.len();
            for _ in 0..*arg_count {
                if let Some(value) = self.values_stack.pop() {
                    parsed_args.insert(0, value);
                } else {
                    return Err(String::from("exptected value in stack"));
                }
            }
            let f_result = f(&this_value, &parsed_args);
            match f_result {
                Ok(f_return_value) => {
                    if let Some(val) = f_return_value {
                        self.values_stack.push(val);
                    }
                }
                Err(err_msg) => {
                    return Err(format!("Error with method {} : {}", &method_name, err_msg));                
                }
            }
        } else {
            return Err(format!("unknows method {} for {}", method_name, this_value.to_string()));
        }
        Ok(())
    }

}



impl ExpressionVisitor for Interpreter {
    fn visit_float_literal_expression(&mut self, expr: &crate::ex_core::expressions::FloatLiteralExpression) -> ExpressionVisitResult {
        self.values_stack.push(ValueVariant::Float(expr.f));
        Ok(())
    }

    fn visit_int_literal_expression(&mut self, expr: &super::IntLiteralExpression) -> ExpressionVisitResult {
        self.values_stack.push(ValueVariant::Integer(expr.i));
        Ok(())
    }

    fn visit_string_literal_expression(&mut self, expr: &crate::ex_core::expressions::StringLiteralExpression) -> ExpressionVisitResult {
        self.values_stack.push(ValueVariant::String(expr.s.clone()));
        Ok(())
    }

    fn visit_bool_literal_expression(&mut self, expr: &super::BoolLiteralExpression) -> ExpressionVisitResult {
        self.values_stack.push(ValueVariant::Bool(expr.b));
        Ok(())
    }

    fn visit_name_expression(&mut self, expr: &crate::ex_core::expressions::NameExpression) -> ExpressionVisitResult {
        assert!(self.var_scopes.len() >= 1);
        if let Some(value) = self.var_scopes.last().unwrap().get(&expr.name) {
            self.values_stack.push(value.clone());
            return Ok(());
        } else {
            return Err(format!("unknown name '{}'", &expr.name));
        }
    }
    fn visit_unary_expression(&mut self, un_expr: &crate::ex_core::expressions::UnaryExpression) -> ExpressionVisitResult {
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
                }
                ValueVariant::Integer(i) => {
                    match op {
                        Token::Plus => {
                            self.values_stack.push(ValueVariant::Integer(i));  
                        } 
                        Token::Minus => {
                            self.values_stack.push(ValueVariant::Integer(-i));            
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

    fn visit_binary_expression(&mut self, expr: &crate::ex_core::expressions::BinaryExpression) -> ExpressionVisitResult {
        expr.left.accept(self)?;
        expr.right.accept(self)?;
        let op = expr.op.clone();
        if let (Some(r), Some(l)) = (self.values_stack.pop(), self.values_stack.pop()) {
            match (l, r) {
                (ValueVariant::Float(l_float), ValueVariant::Float(r_float)) => {
                    match op {
                        Token::Plus => {
                            self.values_stack.push(ValueVariant::Float(l_float + r_float));
                        }
                        Token::Minus => {
                            self.values_stack.push(ValueVariant::Float(l_float - r_float));
                        }
                        Token::Multi => {
                            self.values_stack.push(ValueVariant::Float(l_float * r_float));
                        }
                        Token::Devide => {
                            self.values_stack.push(ValueVariant::Float(l_float / r_float));
                        }
                        Token::Eq => {
                            self.values_stack.push(ValueVariant::Bool(l_float == r_float));
                        }
                        Token::NotEq => {
                            self.values_stack.push(ValueVariant::Bool(l_float != r_float));
                        }
                        Token::More => {
                            self.values_stack.push(ValueVariant::Bool(l_float > r_float));
                        }
                        Token::MoreEq => {
                            self.values_stack.push(ValueVariant::Bool(l_float >= r_float));
                        }
                        Token::Less => {
                            self.values_stack.push(ValueVariant::Bool(l_float < r_float));
                        }
                        Token::LessEq => {
                            self.values_stack.push(ValueVariant::Bool(l_float <= r_float));
                        }
                        _ => {
                            return Err(format!("binary op {} not supported for float's", op.to_string()));
                        }
                    }
                }
                (ValueVariant::Integer(l_int), ValueVariant::Integer(r_int)) => {
                    match op {
                        Token::Plus => {
                            self.values_stack.push(ValueVariant::Integer(l_int + r_int));
                        }
                        Token::Minus => {
                            self.values_stack.push(ValueVariant::Integer(l_int - r_int));
                        }
                        Token::Multi => {
                            self.values_stack.push(ValueVariant::Integer(l_int * r_int));
                        }
                        Token::Devide => {
                            self.values_stack.push(ValueVariant::Float(l_int as f64 / r_int as f64));
                        }
                        Token::Eq => {
                            self.values_stack.push(ValueVariant::Bool(l_int == r_int));
                        }
                        Token::NotEq => {
                            self.values_stack.push(ValueVariant::Bool(l_int != r_int));
                        }
                        Token::More => {
                            self.values_stack.push(ValueVariant::Bool(l_int > r_int));
                        }
                        Token::MoreEq => {
                            self.values_stack.push(ValueVariant::Bool(l_int >= r_int));
                        }
                        Token::Less => {
                            self.values_stack.push(ValueVariant::Bool(l_int < r_int));
                        }
                        Token::LessEq => {
                            self.values_stack.push(ValueVariant::Bool(l_int <= r_int));
                        }
                        _ => {
                            return Err(format!("binary op {} not supported for float's", op.to_string()));
                        }
                    }
                }
                (ValueVariant::String(l_string), ValueVariant::String(r_string)) => {
                    match op {
                        Token::Plus => {
                            self.values_stack.push(ValueVariant::String(l_string + &r_string));
                        }
                        Token::Eq => {
                            self.values_stack.push(ValueVariant::Bool(l_string == r_string));
                        }
                        Token::NotEq => {
                            self.values_stack.push(ValueVariant::Bool(l_string != r_string));
                        }
                        _ => {
                            return Err(format!("binary op {} not supported for strings", op.to_string()));
                        }
                    }
                }
                (ValueVariant::Bool(lb), ValueVariant::Bool(rb)) => {
                    match op {
                        Token::Eq => {
                            self.values_stack.push(ValueVariant::Bool(lb == rb));
                        }
                        Token::NotEq => {
                            self.values_stack.push(ValueVariant::Bool(lb != rb));
                        }
                        _ => {
                            return Err(format!("binary op {} not supported for bool", op.to_string()));
                        }
                    }
                }

                
                _ => {
                    return Err(format!("for now binary operation {} for this args", op.to_string()));
                }
            }
        } else {
            return Err("empty stack in bynary expression".to_string());
        }
        Ok(())
    }

    fn visit_assignment_expression(&mut self, expr: &crate::ex_core::expressions::AssignmentExpression) -> ExpressionVisitResult {
        assert!(self.var_scopes.len() >= 1);
        expr.value.accept(self)?;
        if let Some(value) = self.values_stack.pop() {
            let len = self.var_scopes.len(); 
            self.var_scopes[len - 1].insert(expr.name.clone(), value);
            Ok(())
        } else {
            Err(String::from("no value for assgignment expression"))
        }
    }

    fn visit_if_expression(&mut self, expr: &super::IfExpression) ->ExpressionVisitResult {
        expr.if_expr.accept(self)?;
        if let Some(value) = self.values_stack.pop() {
            match value {
                ValueVariant::Bool(b) => {
                    if b {
                        for expr in expr.true_expression.iter() {
                            expr.accept(self)?;
                        } 
                    } else {
                        for expr in expr.false_expression.iter() {
                            expr.accept(self)?;
                        }   
                    }
                }
                _ => {
                    return Err("not expected result in if".to_string());
                }
            }
        }
        Ok(())
    }

    
    fn visit_while_expression(&mut self, expr: &super::WhileExpression) -> ExpressionVisitResult {
        loop {
            expr.while_expr.accept(self)?;
            if let Some(value) = self.values_stack.pop() {
                match value {
                    ValueVariant::Bool(b) => {
                        if b {
                            for expr in expr.body_exprs.iter() {
                                expr.accept(self)?;
                            } 
                        } else {
                            break;
                        }
                    }
                    _ => {
                        return Err("not expected result in while".to_string());
                    }
                }
            }
        }
        Ok(())
    }

    fn visit_for_expression(&mut self, expr: &super::ForExpression) -> ExpressionVisitResult {
        expr.l_bound.accept(self)?;
        expr.r_bound.accept(self)?;
        let r_bound = self.get_current_stack_value()?;
        let l_bound = self.get_current_stack_value()?;
        let mut l_bound_i = self.value_variant_to_int(&l_bound)?;
        let r_bound_i = self.value_variant_to_int(&r_bound)?;
        self.add_var(&expr.var_name, &l_bound);
        while l_bound_i < r_bound_i {
            for e in expr.body_exprs.iter() {
                e.accept(self)?;
            }
            self.add_var(&expr.var_name, &ValueVariant::Integer(l_bound_i + 1));
            let i_value = self._get_var_value(&expr.var_name).unwrap();
            l_bound_i = self.value_variant_to_int(&i_value)?;
        }
        Ok(())
    }

    fn visit_function_def_expression(&mut self, expr: &crate::ex_core::expressions::FunctionDefExpression) -> ExpressionVisitResult {
        self.user_funcs.insert(expr.name.clone(), expr.clone());
        return Ok(());
    }

    fn visit_function_call_expression(&mut self,  expr: &crate::ex_core::expressions::FunctionCallExpression) -> ExpressionVisitResult {
        if self.std_funcs.contains_key(&expr.name) {
            return self.call_std_func(expr);
        } else if self.user_funcs.contains_key(&expr.name) {
            return self.call_user_func(expr);
        } else {
            return Err(format!("function {} not defined", &expr.name));
        }
    }

    fn visit_method_call_expression(&mut self, expr: &super::MethodCallExpression) -> ExpressionVisitResult {
        if let Some(this_value) = self._get_var(&expr.self_name).cloned() {
            return self.call_method(&this_value, &expr.method_name, &expr.args);
        } else {
            return Err(format!("call method with unknown object '{}'", expr.self_name));
        }
    }

    fn visit_anonymous_method_call_expression(&mut self, expr: &super::AnonymousMethodExpression) -> ExpressionVisitResult {
        expr.self_expr.accept(self)?;
        if let Some(this_value) = self.values_stack.pop() {
            return self.call_method(&this_value, &expr.method_name, &expr.args);
        } else {
            return Err(format!("no value to call method {}", expr.method_name));
        }
    }

    fn visit_return_expression(&mut self, expr: &crate::ex_core::expressions::ReturnExpression) -> ExpressionVisitResult {
        return expr.expr.accept(self);
    }

    fn visit_statement_list_expression(&mut self, expr: &crate::ex_core::expressions::StatementListExpression) -> ExpressionVisitResult {
        for statement in expr.statement_list.iter() {
            statement.accept(self)?;
        }
        Ok(())
    }

}


mod tests {

    #[test]
    fn iterp_test() {
        let mut test_map: std::collections::HashMap<&str, _> = std::collections::HashMap::new();
        test_map.insert("a = 2 + 2", super::ValueVariant::Integer(4));
        test_map.insert("a  =   (2 + 2) * 2", super::ValueVariant::Integer(8));
        test_map.insert("a  = 2 + 2 * 2 * 2", super::ValueVariant::Integer(10));
        test_map.insert("b  = 0 - 3 \n\
                           a = b - 1", super::ValueVariant::Integer(-4));
        test_map.insert("a  = \"aa\" + \"bb\"", super::ValueVariant::String(String::from("aabb")));
        test_map.insert("b  = \"aa\" \n\
                           a = b + \"bb\" ", super::ValueVariant::String(String::from("aabb")));
        for (prog, exp_res) in test_map.iter() {
            let prog = prog.to_string();
            let expr = crate::ex_core::parser::Parser::new(&crate::ex_core::tokenize(&prog)).parse().unwrap();
            let mut interp = crate::ex_core::interp::Interpreter::new(); 
            interp.interp_expr(expr).unwrap();
            assert_eq!(interp._get_var_value(&"a".to_string()).unwrap(), *exp_res);
        }
    }

    #[test]
    fn function_call_test() {
        let prog : String = "fn test() { \n\
                                writeln(\"call test\") \n\
                                return \"afs\" \n\
                            }\n\
                            test()".to_string();    
        let tokens = crate::ex_core::tokenize(&prog);
        let expr = crate::ex_core::parser::Parser::new(&tokens).parse().unwrap();
        let mut interp = crate::ex_core::interp::Interpreter::new(); 
        interp.interp_expr(expr).unwrap();
        assert_eq!(true, true);
    }

    #[test]
    fn function_call_with_args() {
        let prog : String = "fn test(a) { \n\
                                writeln(a) \n\
                            }\n\
                            test(123)".to_string();    

                            
        let tokens = crate::ex_core::tokenize(&prog);
        let expr = crate::ex_core::parser::Parser::new(&tokens).parse().unwrap();
        let mut interp = crate::ex_core::interp::Interpreter::new(); 
        interp.interp_expr(expr).unwrap();
        assert_eq!(true, true);
    }



    #[test]
    fn if_test() {
        let prog : String = "if (true) { \n\
                                writeln(\"bububu\") \n\
                            }\n".to_string();                                
        let tokens = crate::ex_core::tokenize(&prog);
        let expr = crate::ex_core::parser::Parser::new(&tokens).parse().unwrap();
        let mut interp = crate::ex_core::interp::Interpreter::new(); 
        interp.interp_expr(expr).unwrap();
        assert_eq!(true, true);
    }
    
    #[test]
    fn for_test() {
        let prog : String = "for i in [0, 10] { \n\
            writeln(\"fuuu\") \n\
        }\n".to_string();                                
        let tokens = crate::ex_core::tokenize(&prog);
        let expr = crate::ex_core::parser::Parser::new(&tokens).parse().unwrap();
        let mut interp = crate::ex_core::interp::Interpreter::new(); 
        interp.interp_expr(expr).unwrap();
        assert_eq!(true, true);
    }
        
}
