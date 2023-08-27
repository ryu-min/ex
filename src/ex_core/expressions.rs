
use dyn_clone::DynClone;
use super::tokenizer::Token;

pub type ExpressionVisitResult = Result<(), String>;

pub trait ExpressionVisitor {
    fn visit_float_literal_expression(&mut self, expr: &FloatLiteralExpression) -> ExpressionVisitResult;
    fn visit_int_literal_expression(&mut self, expr: &IntLiteralExpression) -> ExpressionVisitResult;
    fn visit_string_literal_expression(&mut self, expr: &StringLiteralExpression) -> ExpressionVisitResult;
    fn visit_bool_literal_expression(&mut self, expr: &BoolLiteralExpression) -> ExpressionVisitResult;
    fn visit_name_expression(&mut self, expr: &NameExpression) -> ExpressionVisitResult;
    fn visit_unary_expression(&mut self, expr: &UnaryExpression) -> ExpressionVisitResult;
    fn visit_binary_expression(&mut self, expr: &BinaryExpression) -> ExpressionVisitResult;
    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression) -> ExpressionVisitResult;
    fn visit_function_def_expression(&mut self, expr: &FunctionDefExpression) -> ExpressionVisitResult;
    fn visit_function_call_expression(&mut self, expr: &FunctionCallExpression) -> ExpressionVisitResult;
    fn visit_return_expression(&mut self, expr: &ReturnExpression) -> ExpressionVisitResult;
    fn visit_statement_list_expression(&mut self, expr: &StatementListExpression) -> ExpressionVisitResult;
}

pub trait Expression : DynClone  {
    fn accept(&self, visitor : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult;
}

impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(&**self)
    }
}

#[derive(Clone)]
pub struct IntLiteralExpression {
    pub i : i64
}
impl IntLiteralExpression {
    pub fn new(i: i64) -> Self { Self { i : i} }
}
impl Expression for IntLiteralExpression {
    fn accept(&self, visitor : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult {
        visitor.visit_int_literal_expression(self)
    }
}


#[derive(Clone)]
pub struct FloatLiteralExpression {
    pub f : f64
} 
impl FloatLiteralExpression {
    pub fn new(f: f64) -> Self {
        FloatLiteralExpression {
            f : f
        }
    }
}
impl Expression for FloatLiteralExpression {
    fn accept(&self, visitor : & mut dyn ExpressionVisitor) -> ExpressionVisitResult {
        visitor.visit_float_literal_expression(self)
    }
}

#[derive(Clone)]
pub struct StringLiteralExpression {
    pub s : String
}
impl StringLiteralExpression {
    pub fn new(s: String) -> Self {
        Self { s : s }
    }
}
impl Expression for StringLiteralExpression {
    fn accept(&self, visitor : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult {
        visitor.visit_string_literal_expression(self)
    }
}

#[derive(Clone)]
pub struct BoolLiteralExpression {
    pub b : bool
}
impl BoolLiteralExpression {
    pub fn new(b: bool) -> Self { Self{ b: b} }
}
impl Expression for BoolLiteralExpression {
    fn accept(&self, visitor : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult {
        visitor.visit_bool_literal_expression(self)
    }
}

#[derive(Clone)]
pub struct UnaryExpression {
    pub op: Token,
    pub expr : Box<dyn Expression>
}
impl UnaryExpression {
    pub fn new(op: Token, expr: Box<dyn Expression>) -> Self {
        UnaryExpression {
            op : op,
            expr : expr
        }
    }
}
impl Expression for UnaryExpression {
    fn accept(&self, visitor : & mut  dyn ExpressionVisitor) -> ExpressionVisitResult {
        visitor.visit_unary_expression(self)
    }
}

#[derive(Clone)]
pub struct BinaryExpression {
    pub op : Token, 
    pub left : Box<dyn Expression>,
    pub right : Box<dyn Expression>
}
impl BinaryExpression {
    pub fn new(op: Token, left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        BinaryExpression { op: op, left: left, right: right }
    }
}
impl Expression for BinaryExpression {
    fn accept(&self,visitor : & mut dyn ExpressionVisitor) -> ExpressionVisitResult {
        visitor.visit_binary_expression(self)
    }
}

#[derive(Clone)]
pub struct AssignmentExpression {
    pub name: String,
    pub value: Box<dyn Expression>
}
impl AssignmentExpression {
    pub fn new(name: String, value: Box<dyn Expression>) -> Self {
        AssignmentExpression { name: name, value: value }
    } 
}
impl Expression for AssignmentExpression {
    fn accept(&self, visitor : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult {
        visitor.visit_assignment_expression(self)
    }
}

#[derive(Clone)]
pub struct StatementListExpression {
    pub statement_list: Vec<Box<dyn Expression>>,
}
impl StatementListExpression {
    pub fn new(statement_list: Vec<Box<dyn Expression>>) -> Self {
        StatementListExpression {
            statement_list : statement_list
        }
    }
}
impl Expression for StatementListExpression {
    fn accept(&self, visitor : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult {
        visitor.visit_statement_list_expression(self)
    }
}

#[derive(Clone)]
pub struct NameExpression {
    pub name : String,
}
impl NameExpression {
    pub fn new(name: String) -> Self {
        NameExpression { name: name }
    }
}
impl Expression for NameExpression {
    fn accept(&self, visitor : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult {
        visitor.visit_name_expression(self)
    }
}

#[derive(Clone)]
pub struct FunctionCallExpression {
    pub name: String, 
    pub args: Vec<Box<dyn Expression>> 
}
impl FunctionCallExpression {
    pub fn new(name: String, args : Vec<Box<dyn Expression>> ) -> Self {
        FunctionCallExpression { name: name, args: args }
    }
}
impl Expression for FunctionCallExpression {
    fn accept(&self, visitor : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult {
        visitor.visit_function_call_expression(self)
    }
}

#[derive(Clone)]
pub struct FunctionDefExpression {
    pub name: String,
    pub args: Vec<String>,
    pub body: Vec<Box<dyn Expression>>
}
impl FunctionDefExpression {
    pub fn new(name: String, args: Vec<String>, body: Vec<Box<dyn Expression>>) -> Self {
        FunctionDefExpression {
            name : name,
            args : args,
            body : body
        }
    }
}
impl Expression for FunctionDefExpression {
    fn accept(&self, visitor : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult {
        visitor.visit_function_def_expression(self)
    }
}

#[derive(Clone)]
pub struct ReturnExpression {
    pub expr: Box<dyn Expression>
}
impl ReturnExpression {
    pub fn new(expr: Box<dyn Expression>) -> Self {
        ReturnExpression { expr: expr }
    }
}
impl Expression for ReturnExpression {
    fn accept(&self, visitor : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult {
        visitor.visit_return_expression(self)
    }
}
