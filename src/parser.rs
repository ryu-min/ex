use crate::tokenizer::Token;
use std::mem;

pub type ExpressionVisitResult = Result<(), String>;

pub trait ExpressionVisitor {
    fn visit_float_expression(&mut self, expr: &FloatExpression) -> ExpressionVisitResult;
    fn visit_unary_expression(&mut self, expr: &UnaryExpression) -> ExpressionVisitResult;
    fn visit_binary_expression(&mut self, expr: &BinaryExpression) -> ExpressionVisitResult;
    fn visit_assignment_expression(&mut self, expr: &AssignmentExpression) -> ExpressionVisitResult;
    fn visit_statement_list_expression(&mut self, extr: &StatementListExpression) -> ExpressionVisitResult;
}

pub trait Expression {
    fn accept(&self, expr : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult;
}

pub struct FloatExpression {
    pub f : f64
} 
impl FloatExpression {
    pub fn new(f: f64) -> Self {
        FloatExpression {
            f : f
        }
    }
}
impl Expression for FloatExpression {
    fn accept(&self, expr : & mut dyn ExpressionVisitor) -> ExpressionVisitResult {
        expr.visit_float_expression(self)
    }
}

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
    fn accept(&self, expr : & mut  dyn ExpressionVisitor) -> ExpressionVisitResult {
        expr.visit_unary_expression(self)
    }
}


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
    fn accept(&self,expr : & mut dyn ExpressionVisitor) -> ExpressionVisitResult {
        expr.visit_binary_expression(self)
    }
}

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
    fn accept(&self, expr : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult {
        expr.visit_assignment_expression(self)
    }
}

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
    fn accept(&self, expr : & mut dyn ExpressionVisitor) ->  ExpressionVisitResult {
        expr.visit_statement_list_expression(self)
    }
}

// pub struct EmptyExpression;
// impl EmptyExpression {
//     pub fn 
// }


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

    fn statement(&mut self) -> ParseResult {
        if let Some(token) = self.peek_current_token() {
            match token {
                Token::Name(_) => {
                    return self.assignment_statement();
                }
                _ => {
                    return Err(format!("unsupported statement token {}", token.to_string()));
                }
            }
        } else {
            return Err(String::from("no token for statement"));
        }
    }

    fn assignment_statement(&mut self) -> ParseResult {
        if let Some(name_token) = self.peek_current_token() {
            match name_token {
                Token::Name(name) => {
                    let eat_result = self.eat(Token::Assignment);
                    if let Ok(()) = eat_result {
                        let value = self.expr()?;
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
    
    /// 'expr' function match next syntax pattern:
    /// term [[PLUS|MINUS] term]*
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
    /// factor [[MUL|DIV] factor]*
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
    /// FLOAT | ( expr )
    fn factor(&mut self) -> ParseResult {
        let current_token = self.peek_current_token().unwrap();
        match current_token {
            Token::FloatLiteral(f) => {
                self.advance();
                return Ok(Box::new(FloatExpression::new(f)));
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
            _ => {
                println!("error in factor with token {}", current_token.to_string());
                return Err("unreachable".to_string());
            }
        }
    }

    fn peek_current_token(&self) -> Option<Token> {
        if self.pos >= self.tokens.len() {
            return None
        }
        return Some(self.tokens[self.pos].clone());

    }

    // fn peek_next_token(&mut self) -> Option<Token> {
    //     if self.pos + 1 >= self.tokens.len() {
    //         return None
    //     }
    //     return Some(self.tokens[self.pos + 1].clone());
    // }

    fn advance(&mut self) {
        self.pos += 1;
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
}