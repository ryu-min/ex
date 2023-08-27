use std::mem;

use super::{Expression, Token, StatementListExpression, AssignmentExpression, FunctionCallExpression, FunctionDefExpression, BinaryExpression, IntLiteralExpression, FloatLiteralExpression, StringLiteralExpression, UnaryExpression, NameExpression, BoolLiteralExpression};

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
    /// {assignment_statement} | {function call} | {function def} | {return_expr} | {expr}
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
                return self.expression();    
            } else {
                return self.expression();
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
        let f_args = self.read_func_call_args()?;
        return Ok(Box::new(FunctionCallExpression::new(f_name, f_args)));
    }

    fn function_def_statement(&mut self) -> ParseResult {
        self.eat(Token::Fn)?;
        let f_name = self.read_name()?;
        let f_args = self.read_func_def_args()?;
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

    fn expression(&mut self) -> ParseResult {
        return self.equality();
    }

    fn equality(&mut self) -> ParseResult {
        let mut result = self.term()?;
        loop {
            if let Some(token) = self.peek_current_token() {
                match token {
                    Token::Eq | Token::NotEq => {
                        self.advance();
                        let expr = self.term()?;
                        result = Box::new(BinaryExpression::new(token,result,expr));
                    }
                    _ => {
                        break;
                    }
                }
            }
        }
        return Ok(result);
    }

    /// 'expr' function match next syntax pattern:
    /// {term} [[PLUS|MINUS] {term}]*
    fn term(&mut self) -> ParseResult {
        let mut result = self.factor()?;
        loop {
            if let Some(token) = self.peek_current_token() {
                match token {
                    Token::Plus | Token::Minus => {
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

    /// 'term' function match next syntax pattern:
    /// {factor} [[MUL|DIV] {factor}]*
    fn factor(&mut self) -> ParseResult {
        let mut result = self.unary()?;
        loop {
            if let Some(token) = self.peek_current_token() {
                match token {
                    Token::Multi | Token::Devide => {
                        self.advance();
                        let expr = self.unary()?;
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

    fn unary(&mut self) -> ParseResult {
        if let Some(token) = self.peek_current_token() {
            match token {
                Token::Plus|Token::Minus => {
                    self.advance();
                    let expr = self.unary()?;
                    return Ok(Box::new(UnaryExpression::new(token, expr)));
                }
                _ => {}
            }
        }
        return self.primary();
    }
    
    fn primary(&mut self) -> ParseResult {
        let current_token = self.peek_current_token().unwrap();
        match current_token {
            Token::IntLiteral(i) => {
                self.advance();
                return Ok(Box::new(IntLiteralExpression::new(i)));
            }
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
                let result = self.expression()?;
                self.eat(Token::CloseBrace)?;
                return Ok(result); 
            }
            Token::Name(n) => {
                self.advance();
                return Ok(Box::new(NameExpression::new(n)));
            }
            Token::True => {
                self.advance();
                return Ok(Box::new(BoolLiteralExpression::new(true)));
            }
            Token::False => {
                self.advance();
                return Ok(Box::new(BoolLiteralExpression::new(false)));
            }
            Token::NewLine => {
                self.advance();
                return self.primary();
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

    fn read_func_def_args(&mut self) -> Result<Vec<String>, String> {
        self.eat(Token::OpenBrace)?;
        let mut f_args : Vec<String> = Vec::new();
        loop {
            if let Some(current_token) = self.peek_current_token() {
                match current_token {
                    Token::CloseBrace => {
                        break;
                    }
                    Token::Comma => {
                        self.advance();
                    }
                    Token::Name(n) => {
                        f_args.push(n);
                        self.advance();
                    }
                    _ => {
                        return Err(format!("Token {} not supported in function def args", current_token.to_string()));
                    }
                }
            }
        }
        self.eat(Token::CloseBrace)?;
        Ok(f_args)
    } 

    fn read_func_call_args(&mut self) -> Result<Vec<Box<dyn Expression>>, String> {
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
                        let arg_expression = self.statement()?;
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