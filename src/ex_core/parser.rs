use std::mem;

use super::{Expression, Token, StatementListExpression, AssignmentExpression, FunctionCallExpression, FunctionDefExpression,   BinaryExpression, IntLiteralExpression, FloatLiteralExpression, StringLiteralExpression, UnaryExpression, NameExpression, BoolLiteralExpression, IfExpression, WhileExpression, ForExpression};

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
            if self.current_token_is(Token::Name("".to_string())) && 
               self.nth_token_is(1, Token::Assignment) {
                return self.assignment_statement();
            } else if self.current_token_is(Token::Name("".to_string())) && 
                      self.nth_token_is(1, Token::OpenBracket) {
                    return self.function_call_statement();        
            } else if self.current_token_is(Token::Fn) {
                return self.function_def_statement();
            } else if self.current_token_is(Token::While) {
                return self.while_statement();
            } else if self.current_token_is(Token::For) {
                return self.for_statement();
            } else if self.current_token_is(Token::If) {
                return self.if_statement();
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

    fn parse_statements_in_curly_braces(&mut self) -> Result<Vec<Box<dyn Expression>>,String> {
        let mut result : Vec<Box<dyn Expression>> = Vec::new();
        self.eat(Token::OpenCurlyBracket)?;
        self.skip_new_lines();
        while !self.current_token_is(Token::CloseCurlyBraket) {
            result.push(self.statement()?); 
            self.skip_new_lines();
        }
        self.skip_new_lines();
        self.eat(Token::CloseCurlyBraket)?;
        Ok(result)
    }

    fn while_statement(&mut self) -> ParseResult {
        self.eat(Token::While)?;
        let while_expr = self.expression()?;
        let true_exprs = self.parse_statements_in_curly_braces()?;
        return Ok(Box::new(WhileExpression::new(while_expr, true_exprs)));
    }
    
    fn for_statement(&mut self) -> ParseResult {
        self.eat(Token::For)?;
        let var_name = self.parse_name()?;
        self.eat(Token::In)?;
        self.eat(Token::OpenSquareBracket)?;
        let left_b = self.expression()?;
        self.eat(Token::Comma)?;
        let right_b = self.expression()?;
        self.eat(Token::CloseSquareBracket)?;
        let body = self.parse_statements_in_curly_braces()?;
        return Ok(Box::new(ForExpression::new(var_name, left_b, right_b, body)));
    }

    fn if_statement(&mut self) -> ParseResult {
        self.eat(Token::If)?;
        let if_expr = self.expression()?;
        let true_expressions = self.parse_statements_in_curly_braces()?;
        let mut false_expressions: Vec<Box<dyn Expression>> = Vec::new();
        if self.current_token_is(Token::Else) {
            self.eat(Token::Else)?;
            false_expressions = self.parse_statements_in_curly_braces()?;
        }
        return Ok(Box::new(IfExpression::new(if_expr, true_expressions, false_expressions)));
    }

    /// 'assignment_statement' function match next syntax pattern:
    /// let NAME = {expr}
    fn assignment_statement(&mut self) -> ParseResult {
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
        let f_name = self.parse_name()?;
        let f_args = self.parse_func_call_args()?;
        return Ok(Box::new(FunctionCallExpression::new(f_name, f_args)));
    }

    fn function_def_statement(&mut self) -> ParseResult {
        self.eat(Token::Fn)?;
        let f_name = self.parse_name()?;
        let f_args = self.parse_func_def_args()?;
        let f_body = self.parse_statements_in_curly_braces()?;
        return Ok(Box::new(FunctionDefExpression::new(f_name, f_args, f_body)));
    }

    fn expression(&mut self) -> ParseResult {
        return self.equality();
    }
    
    fn equality(&mut self) -> ParseResult {
        let mut result = self.compression()?;
        loop {
            if let Some(token) = self.peek_current_token() {
                match token {
                    Token::Eq | Token::NotEq => {
                        self.advance();
                        let expr = self.compression()?;
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

    fn compression(&mut self) -> ParseResult {
        let mut result = self.term()?;
        loop {
            if let Some(token) = self.peek_current_token() {
                match token {
                    Token::More | Token::MoreEq | Token::Less | Token::LessEq => {
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
            Token::OpenBracket => {
                self.advance();
                let result = self.expression()?;
                self.eat(Token::CloseBracket)?;
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

    fn parse_name(&mut self) -> Result<String, String> {
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

    fn parse_func_def_args(&mut self) -> Result<Vec<String>, String> {
        self.eat(Token::OpenBracket)?;
        let mut f_args : Vec<String> = Vec::new();
        loop {
            if let Some(current_token) = self.peek_current_token() {
                match current_token {
                    Token::CloseBracket => {
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
        self.eat(Token::CloseBracket)?;
        Ok(f_args)
    } 

    fn parse_func_call_args(&mut self) -> Result<Vec<Box<dyn Expression>>, String> {
        self.eat(Token::OpenBracket)?;
        let mut f_args : Vec<Box<dyn Expression>> = Vec::new();
        loop {
            if let Some(current_token) = self.peek_current_token() {
                match current_token {
                    Token::CloseBracket => {
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
        self.eat(Token::CloseBracket)?;
        Ok(f_args)
    }

    fn skip_new_lines(&mut self) {
        while self.current_token_is(Token::NewLine) {
            self.advance();
        }
    }

}