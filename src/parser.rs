use crate::tokenizer::{Token, tokenize};

trait Expression {
    fn eval(&self) -> Option<f64>;
}

struct FloatExpression {
    f : f64
}

impl FloatExpression {
    pub fn new(f: f64) -> Self {
        FloatExpression {
            f : f
        }
    }
}
impl Expression for FloatExpression {
    fn eval(&self) -> Option<f64> {
        return Some(self.f);
    }
}

struct BinaryExpression {
    op : char, 
    left : Box<dyn Expression>,
    right : Box<dyn Expression>
}
impl BinaryExpression {
    pub fn new(op: char, left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        BinaryExpression { op: op, left: left, right: right }
    }
}

impl Expression for BinaryExpression {
    fn eval(&self) -> Option<f64> {

        if let (Some(l), Some(r) ) = (self.left.eval(), self.right.eval() ) {
            match self.op {
                '+' => return Some(l + r),
                '-' => return Some(l - r),
                '*' => return  Some(l * r),
                '/' => return Some(l / r),
                _ => return None
            }
        } else {
            return None
        }
    }
}

struct Parser {
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

    pub fn parse(&mut self) -> Option<f64> {
        self.expr().eval()
    } 

    fn expr(&mut self) -> Box<dyn Expression> {
        self.aditive()
    }

    fn aditive(&mut self) -> Box<dyn Expression> {
        let mut result = self.multi();
        loop {
            if let Some(next_token) = self.peek_current_token() {
                match next_token {
                    Token::Plus => { 
                        self.advance();
                        result = Box::new(BinaryExpression::new('+', result, self.multi()));
                    }
                    Token::Minus => {
                        self.aditive();
                         result = Box::new(BinaryExpression::new('-', result, self.multi()));
                    }
                    _ => break
                }
            } else {
                break;
            }
        }
        return result;
    }

    fn multi(&mut self) -> Box<dyn Expression> {
        let mut result = self.unary();
        loop {
            if let Some(next_token) = self.peek_current_token() {
                match next_token {
                    Token::Multi => {
                        self.advance();
                        result = Box::new(BinaryExpression::new('*', result, self.unary())); 
                    }
                    Token::Devide => { 
                        self.advance();
                        result = Box::new(BinaryExpression::new('/', result, self.unary()));
                    }
                    _ => break
                }
            } else {
                break;
            }
        }
        return result;
    }
 
    fn unary(&mut self) -> Box<dyn Expression> {
        return self.primary()
    }

    fn primary(&mut self) -> Box<dyn Expression> {
        if let Some(token) = self.peek_current_token() {
            match token {
                Token::FloatLiteral(f) => {
                    self.advance();
                    return Box::new(FloatExpression::new(f));
                }
                _ => {
                    self.advance();
                    return Box::new(FloatExpression::new(69.));    
                }    
            }
        } else {
            return Box::new(FloatExpression::new(69.));     
        }
    }

    fn peek_current_token(&self) -> Option<Token> {
        if self.pos >= self.tokens.len() {
            return None
        }
        return Some(self.tokens[self.pos].clone());

    }

    fn peek_next_token(&mut self) -> Option<Token> {
        if self.pos + 1 >= self.tokens.len() {
            return None
        }
        return Some(self.tokens[self.pos + 1].clone());
    }

    fn advance(&mut self) {
        self.pos += 1;
    }




    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parser_test() {
        let program = "2 + 2 + 100 * 2".to_string();
        let tokens = tokenize(&program);
        let mut p = Parser::new(&tokens);
        assert_eq!(p.parse().unwrap(), 204.);
    }
}
