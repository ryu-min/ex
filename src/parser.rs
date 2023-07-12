use crate::tokenizer::Token;

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
    left : f64,
    right : f64
}
impl BinaryExpression {
    pub fn new(op: char, left: f64, right: f64) -> Self {
        BinaryExpression { op: op, left: left, right: right }
    }
}

///! @todo Bynary storee expression (left and right)

impl Expression for BinaryExpression {
    fn eval(&self) -> Option<f64> {
        match  self.op {
            '+' => Some(self.left + self.right),
            '-' => Some(self.left - self.right),
            '*' => Some(self.left * self.right),
            '/' => Some(self.left / self.right),
            _ => None
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

    pub fn parse(&self) -> f64 {
        2.
    } 

    fn expr(&mut self) -> Box<dyn Expression> {
        self.binary()
    }

    fn binary(&mut self) -> Box<dyn Expression> {
        let expr = self.unary();
        loop {
            if let Some(next_token) = self.next_token() {
                match next_token {
                    Token::Multi => {},
                    Token::Devide => {},
                    _ => {}
                }
            } else {
                break;
            }
        }
    }
 
    fn unary(&mut self) -> Box<dyn Expression> {
        return self.primary()
    }

    fn primary(&mut self) -> Box<dyn Expression> {
        let token = self.current_token();
        match token {
            Token::FloatLiteral(f) => {
                Box::new(FloatExpression::new(f))
            }
            _ => {
                Box::new(FloatExpression::new(69.))    
            }
            
        }
    }

    fn current_token(&self) -> Token {
        return self.tokens[self.pos].clone();
    }

    fn next_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }


    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parser_test() {
        let a : Box<dyn Expression> = Box::new( FloatExpression::new(2.) );
        assert_eq!(a.eval(), 2.); 
    }
}
