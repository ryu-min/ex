use crate::tokenizer::Token;
use std::mem;

pub type ExpressionVisitResult = Result<(), String>;

pub trait ExpressionVisitor {
    fn visit_float_expression(&mut self, expr: &FloatExpression) -> ExpressionVisitResult;
    fn visit_unary_expression(&mut self, expr: &UnaryExpression) -> ExpressionVisitResult;
    fn visit_binary_expression(&mut self, expr: &BinaryExpression) -> ExpressionVisitResult;
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
        self.expr()
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