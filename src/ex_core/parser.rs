use std::mem;
use dyn_clone::DynClone;
use super::tokenizer::Token;

pub type ExpressionVisitResult = Result<(), String>;

pub trait ExpressionVisitor {
    fn visit_float_literal_expression(&mut self, expr: &FloatLiteralExpression) -> ExpressionVisitResult;
    fn visit_string_literal_expression(&mut self, expr: &StringLiteralExpression) -> ExpressionVisitResult;
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
    pub args: Vec<Box<dyn Expression>>,
    pub body: Vec<Box<dyn Expression>>
}
impl FunctionDefExpression {
    pub fn new(name: String, args: Vec<Box<dyn Expression>>, body: Vec<Box<dyn Expression>>) -> Self {
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


pub type ParseResult = Result<Box<dyn Expression>, String>;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize
}

impl Parser {
    pub fn new(tokens: &Vec<Token>) -> Self {
        Parser {
            tokens: tokens.clone(),
            pos: 0
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        self.program()
    } 


    fn program(&mut self) -> ParseResult {
        self.statement_list()
    }

    /// 'statement_list' function match next syntax pattern:
    /// {statement}*
    fn statement_list(&mut self) -> ParseResult {
        let mut statements  = Vec::new();
        loop {
            if let Some(current_token) = self.peek_current_token() {
                match current_token {
                    Token::NewLine => {
                        self.advance();
                        continue;
                    }
                    _ => {
                        let statement = self.statement()?;
                        statements.push(statement);
                    }
                }
            } else {
                break;
            }
        }
        Ok(Box::new(StatementListExpression::new(statements)))
    }

    /// 'statement' function match next syntax pattern:
    /// {assignment_statement} | {function call}
    fn statement(&mut self) -> ParseResult {
        if let Some(_) = self.peek_current_token() {
            if self.current_token_is(Token::Var) {
                return self.assignment_statement();
            } else if self.current_token_is(Token::Name("".to_string())) && 
                      self.nth_token_is(1, Token::OpenBrace) {
                    return self.function_call_statement();        
            } else if self.current_token_is(Token::Fn) {
                return self.function_def_statement();
            } else if self.current_token_is(Token::NewLine) {
                self.advance();
                return self.statement();
            } else if self.current_token_is(Token::Return) {
                self.advance();
                return self.expr();    
            } else {
                return self.expr();
            }
        } else {
            return Err(String::from("no token for statement"));
        }
    }

    /// 'assignment_statement' function match next syntax pattern:
    /// let NAME = {expr}
    fn assignment_statement(&mut self) -> ParseResult {
        self.eat(Token::Var)?;
        if let Some(name_token) = self.peek_current_token() {
            match name_token {
                Token::Name(name) => {
                    self.advance();
                    let eat_result = self.eat(Token::Assignment);
                    if let Ok(()) = eat_result {
                        let value = self.statement()?;
                        return Ok(Box::new(AssignmentExpression::new(name, value)));
                    } else if let Err(error_message) = eat_result {
                        return Err(error_message);                        
                    }  
                }
                _ => {
                    return Err(String::from("expected 'name' token in assignment statement"));
                }
            }
        }
        return Err(String::from("no token in assignment statement"));
    }


    /// 'function_call_statement' function match next syntax pattern:
    /// f_name ([expt,]*)
    fn function_call_statement(&mut self) -> ParseResult {

        let f_name = self.read_name()?;
        let f_args = self.read_func_args()?;
        return Ok(Box::new(FunctionCallExpression::new(f_name, f_args)));
    }

    fn function_def_statement(&mut self) -> ParseResult {
        self.eat(Token::Fn)?;
        let f_name = self.read_name()?;
        let f_args = self.read_func_args()?;
        let mut f_body : Vec<Box<dyn Expression>> = Vec::new();
        self.eat(Token::OpenCurlyBrace)?;
                    self.skip_new_lines();
        while !self.current_token_is(Token::CloseCurlyBrace) {
            f_body.push(self.statement()?); 
            self.skip_new_lines();
        }
        self.skip_new_lines();
        self.eat(Token::CloseCurlyBrace)?;
        return Ok(Box::new(FunctionDefExpression::new(f_name, f_args, f_body)));
    }


    /// 'expr' function match next syntax pattern:
    /// {term} [[PLUS|MINUS] {term}]*
    fn expr(&mut self) -> ParseResult {
        let mut result = self.temr()?;
        loop {
            if let Some(token) = self.peek_current_token() {
                match token {
                    Token::Plus | Token::Minus => {
                        self.advance();
                        let expr = self.temr()?;
                        result =  Box::new(BinaryExpression::new(token, result, expr));
                    }
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        return Ok(result);
    }

    /// 'term' function match next syntax pattern:
    /// {factor} [[MUL|DIV] {factor}]*
    fn temr(&mut self) -> ParseResult {
        let mut result = self.factor()?;
        loop {
            if let Some(token) = self.peek_current_token() {
                match token {
                    Token::Multi | Token::Devide => {
                        self.advance();
                        let expr = self.factor()?;
                        result =  Box::new(BinaryExpression::new(token, result, expr));
                    }
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        return Ok(result);
    }
    
    /// 'factor' function match next syntax pattern:
    /// FLOAT_LITERAL | STRING_LITERAL | NAME | {expr}
    fn factor(&mut self) -> ParseResult {
        let current_token = self.peek_current_token().unwrap();
        match current_token {
            Token::FloatLiteral(f) => {
                self.advance();
                return Ok(Box::new(FloatLiteralExpression::new(f)));
            }
            Token::StringLiteral(s) => {
                self.advance();
                return Ok(Box::new(StringLiteralExpression::new(s)));
            }
            Token::OpenBrace => {
                self.advance();
                let result = self.expr()?;
                self.eat(Token::CloseBrace)?;
                return Ok(result); 
            }
            Token::Plus|Token::Minus => {
                self.advance();
                let expr = self.factor()?;
                return Ok(Box::new(UnaryExpression::new(current_token, expr)));
            }
            Token::Name(n) => {
                self.advance();
                return Ok(Box::new(NameExpression::new(n)));
            }
            _ => {
                return Err(format!("Not valid factor {}", current_token.to_string()));
            }
        }
    }

    fn advance(&mut self) {
        self.pos += 1;
    }
    fn peek_current_token(&self) -> Option<Token> {
        if self.pos >= self.tokens.len() {
            return None
        }
        return Some(self.tokens[self.pos].clone());
    }

    fn peek_nth_token(&self, n: usize) -> Option<Token> {
        if self.pos + n >= self.tokens.len() {
            return None;
        }
        return Some(self.tokens[self.pos + n].clone());
    }

    fn current_token_is(&self, exp_token: Token) -> bool {
        return self.nth_token_is(0, exp_token);
    }

    fn nth_token_is(&self, n: usize, exp_token: Token) -> bool {
        if let Some(current_token) = self.peek_nth_token(n) {
            let exp_disc = mem::discriminant(&exp_token);
            return mem::discriminant(&current_token).eq( &exp_disc );
        } else {
            return false;
        }
    }

    fn eat(&mut self, token: Token) -> Result<(), String> {
        if let Some(current) = self.peek_current_token() {
            if  mem::discriminant(&current) == mem::discriminant(&token) {
                self.advance();
                Ok(())
            } else {
                return Err(format!("exprected token {}, find token {}", token.to_string(), current.to_string()));
            }
        } else {
            return Err(format!("exptect token {}, found no token", token.to_string()));
        }
    }

    fn read_name(&mut self) -> Result<String, String> {
        if let Some(name_token) = self.peek_current_token() {
            match name_token {
                Token::Name(n) => {
                    self.advance();
                    Ok(n)
                } 
                _ => {
                    return Err(String::from("expected name token"));
                }       
            }
        } else {
            return Err(String::from("expected name token"));
        }
    }

    fn read_func_args(&mut self ) -> Result<Vec<Box<dyn Expression>>, String> {
        self.eat(Token::OpenBrace)?;
        let mut f_args : Vec<Box<dyn Expression>> = Vec::new();
        loop {
            if let Some(current_token) = self.peek_current_token() {
                match current_token {
                    Token::CloseBrace => {
                        break;
                    }
                    Token::Comma => {
                        self.advance();
                    }
                    _ => {
                        let arg_expression = self.expr()?;
                        f_args.push(arg_expression);
                    }
                }
            }
        }
        self.eat(Token::CloseBrace)?;
        Ok(f_args)
    }

    fn skip_new_lines(&mut self) {
        while self.current_token_is(Token::NewLine) {
            self.advance();
        }
    }

}