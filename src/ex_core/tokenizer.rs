use std::fmt;


pub fn tokenize(program: &String) -> Vec<Token> {
    let mut result : Vec<Token> = Vec::new();
    for line in program.lines().clone() {
        let mut word_string = line.to_string();
        while !word_string.is_empty() {
            if let Some(token) = read_token_from_char(&mut word_string) {
                result.push(token)
            } else if let Some(token) = read_token_from_string(&mut word_string) {
                result.push(token)    
            } else {
                if word_string.chars().next().unwrap() == ' ' {
                    word_string.remove(0);
                } else {
                    break;
                }
            }
        }
        result.push(Token::NewLine);
    }
    return result;
}

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Var,
    Dot,
    Comma,
    Assignment,
    Plus,
    Minus,
    Multi,
    Devide,
    OpenBrace,
    CloseBrace,
    OpenCurlyBrace,
    CloseCurlyBrace,
    StringLiteral(String),
    Name(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    Exec,
    Fn,
    Return,
    NewLine, 
    True,
    False,
    Eq,
    NotEq,
    More, 
    MoreEq,
    Less,
    LessEq,
 }

 impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Var => write!(f, "VAR TOKEN"),
            Token::Dot => write!(f, "DOT TOKEN"),
            Token::Comma => write!(f, "COMMA TOKEN"),
            Token::Assignment => write!(f, "ASSIGNMENT TOKEN"),
            Token::StringLiteral(s) => write!(f, "STRING LITERAL TOKEN WITH VALUE: {}", s),
            Token::IntLiteral(n) => write!(f, "INT LITERAL TOKEN WITH VALUE: {}", n),
            Token::FloatLiteral(n) => write!(f, "FLOAT LITERAL WITH VALUE: {}", n),
            Token::Exec => write!(f, "EXEC TOKEN"),
            Token::OpenBrace => write!(f, "OPEN BRACE TOKEN"),
            Token::CloseBrace => write!(f, "CLOSE BRACE TOKEN"),
            Token::OpenCurlyBrace => write!(f, "OPEN CURLY BRACE TOKEN"),
            Token::CloseCurlyBrace => write!(f, "CLOSE CURLY BRACE TOKEN"),
            Token::Plus => write!(f, "PLUS TOKEN"),
            Token::Minus => write!(f, "MINUS TOKEN"),
            Token::Multi => write!(f, "MULTI TOKEN"),
            Token::Devide => write!(f, "DEVIDE TOKEN"),
            Token::Name(s) => write!(f, "NAME TOKEN {s}"), 
            Token::NewLine => write!(f, "NEW LINE TOKEN"),
            Token::Fn => write!(f, "FN TOKEN"),
            Token::Return => write!(f, "RETURN TOKEN"),
            Token::True => write!(f, "TRUE TOKEN"),
            Token::False => write!(f, "FALSE TOKEN"),
            Token::Eq => write!(f, "EQ TOKEN"),
            Token::NotEq => write!(f, "NOT EQ TOKEN"),
            Token::More => write!(f, "MORE TOKEN"),
            Token::MoreEq => write!(f, "MORE EQ TOKEN"),
            Token::Less => write!(f, "LESS TOKEN"),
            Token::LessEq => write!(f, "LESS EQ TOKEN"),
        }
    }
}

fn char_to_simple_token(ch : char) -> Option<Token> {
    match ch {
        '.' =>      Some(Token::Dot),
        '(' =>      Some(Token::OpenBrace),
        ')' =>      Some(Token::CloseBrace),
        '{' =>      Some(Token::OpenCurlyBrace),
        '}' =>      Some(Token::CloseCurlyBrace),
        '+' =>      Some(Token::Plus),
        '-' =>      Some(Token::Minus),
        '/' =>      Some(Token::Devide),
        '*' =>      Some(Token::Multi),
        ',' =>      Some(Token::Comma),
        _ =>        None
    }
}

fn read_token_from_char(source: &mut String) -> Option<Token> {
    let mut chars = source.chars();
    if let Some(f_ch) = chars.next() {
        if let Some(token) = char_to_simple_token(f_ch) {
            source.replace_range(0..1, "");
            return Some(token)
        } else if f_ch == '=' {
            if let Some(s_ch) = chars.next() {
                if s_ch != '=' {
                    source.replace_range(0..1, "");
                    return Some(Token::Assignment);    
                }
            }
        } else if f_ch == '>' {
            if let Some(s_ch) = chars.next() {
                if s_ch != '=' {
                    source.replace_range(0..1, "");
                    return Some(Token::More);    
                }
            }
        } else if f_ch == '<' {
            if let Some(s_ch) = chars.next() {
                if s_ch != '=' {
                    source.replace_range(0..1, "");
                    return Some(Token::Less);    
                }
            }
        }
    }
    None
}

fn read_token_from_string(source: &mut String) -> Option<Token> {
    if source.is_empty() {
        return None
    }
    if let Some(token) = read_number_token(source) {
        return Some(token)
    } else if let Some(token) = read_string_literal_token(source) {
        return Some(token);
    } else if let Some(token) = read_reserved_token(source) {
        return Some(token);
    } 
    else if let Some(token) = read_name_token(source) {
        return Some(token);
    } 
    None
}

fn read_number_token(source: &mut String) -> Option<Token> {
    let mut result_string = String::new();
    for (i, ch) in source.char_indices() {
        if i == 0 {
            if ch != '-' && ch != '+' && !ch.is_numeric() {
                return None
            }
            result_string.push(ch);
        } else {
            if ch.is_numeric() || ch == '.' {
                result_string.push(ch);
            } else {
                break;
            }    
        }
    }
    if result_string.is_empty() {
        return None
    }
    source.replace_range(0..result_string.len(), "");
    if result_string.contains('.') {
        if let Ok(f) = result_string.parse::<f64>() {
            return Some(Token::FloatLiteral(f));
        } 
    } else if let Ok(i) = result_string.parse::<i64>() {
        return Some(Token::IntLiteral(i));
    }
    None
}



fn read_reserved_token(source: &mut String) -> Option<Token> { 
    if try_read_reserved_word("var", source) {
        return Some(Token::Var);
    } else if try_read_reserved_word("fn", source) {
        return Some(Token::Fn);
    } else if try_read_reserved_word("return", source) {
        return Some(Token::Return);
    } else if try_read_reserved_word("true", source) {
        return Some(Token::True);
    } else if try_read_reserved_word("false", source) {
        return Some(Token::False);
    } else if try_read_reserved_word("==", source) {
        return Some(Token::Eq);
    } else if try_read_reserved_word("!=", source) {
        return Some(Token::NotEq);
    } else if try_read_reserved_word(">=", source) {
        return Some(Token::MoreEq);
    } else if try_read_reserved_word("<=", source) {
        return Some(Token::LessEq);
    }
    None
}

fn try_read_reserved_word(reserved: &str, source: &mut String) -> bool {
    let reserved_with_space = reserved.to_owned() + " ";
    if source == reserved || source.starts_with(&reserved_with_space) {
        source.replace_range(..reserved.len(), "");
        return true;
    }
    return false;
} 


fn read_name_token(source: &mut String) -> Option<Token> {
    let mut result = String::new();
    for ch in source.chars() {
        if ch.is_alphanumeric() {
            result.push(ch);
        } else {
            break;
        }
    }
    if result.is_empty() {
        return None;
    }
    source.replace_range(0..result.len(), "");
    return Some(Token::Name(result));
}

fn read_string_literal_token(source: &mut String) -> Option<Token> {
    if !source.starts_with("\"") {
        return None;
    }
    let quote_indexes : Vec<_> = source.match_indices("\"").collect();
    if quote_indexes.len() < 2 {
        return None;
    }
    let end_literal = quote_indexes[1].0;
    let string_literal = source[1..end_literal].to_string();
    source.replace_range(..end_literal + 1, "");
    return Some(Token::StringLiteral(string_literal));
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn bug_test() {

        let program = "write(\"input: \", a)".to_string();
        let tokens = tokenize(&program);
        for token in tokens.iter() {
            println!("token is {}", token);
        }
        assert!(true);
    }


    #[test]
    fn tokenizer_test() {
        let program: String = String::from("var x = 10 \n\
                                            var y = 20.5 \n\
                                            var s = \"str\" \\
                                            x = x * ( x + y ) \\
                                            var a = true \n\
                                            var c = a == true\n\
                                            var bu = c != false \n\
                                            var boo1 = 1 >= 2 \n\
                                            var boo2 = 1 > 2 \n\
                                            var boo3 = 1 <= 2 \n\
                                            var boo4 = 1 < 2 \n");
        let expected_tokens = vec![
            Token::Var,
            Token::Name(String::from("x")),
            Token::Assignment,
            Token::IntLiteral(10),
            Token::NewLine,

            Token::Var,
            Token::Name(String::from("y")),
            Token::Assignment,
            Token::FloatLiteral(20.5),
            Token::NewLine,

            Token::Var,
            Token::Name(String::from("s")),
            Token::Assignment,
            Token::StringLiteral(String::from("str")),
            Token::NewLine,

            Token::Name(String::from("x")),
            Token::Assignment,
            Token::Name(String::from("x")),
            Token::Multi,
            Token::OpenBrace,
            Token::Name(String::from("x")),
            Token::Plus,
            Token::Name(String::from("y")),
            Token::CloseBrace,
            Token::NewLine,

            Token::Var,
            Token::Name(String::from("a")),
            Token::Assignment,
            Token::True,
            Token::NewLine,

            Token::Var,
            Token::Name(String::from("c")),
            Token::Assignment,
            Token::Name(String::from("a")),
            Token::Eq,
            Token::True,
            Token::NewLine,

            Token::Var, 
            Token::Name(String::from("bu")),
            Token::Assignment,
            Token::Name(String::from("c")),
            Token::NotEq,
            Token::False,
            Token::NewLine,

            Token::Var,
            Token::Name(String::from("boo1")),
            Token::Assignment,
            Token::IntLiteral(1),
            Token::MoreEq,
            Token::IntLiteral(2),
            Token::NewLine,

            Token::Var,
            Token::Name(String::from("boo2")),
            Token::Assignment,
            Token::IntLiteral(1),
            Token::More,
            Token::IntLiteral(2),
            Token::NewLine,

            Token::Var,
            Token::Name(String::from("boo3")),
            Token::Assignment,
            Token::IntLiteral(1),
            Token::LessEq,
            Token::IntLiteral(2),
            Token::NewLine,

            Token::Var,
            Token::Name(String::from("boo4")),
            Token::Assignment,
            Token::IntLiteral(1),
            Token::Less,
            Token::IntLiteral(2),
            Token::NewLine,        
        ];
        let tokens = tokenize(&program);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn number_tests() {
        let program: String = String::from("5 + 5.5 = 11 - 0.5");
        let expected_tokens = vec![
            Token::IntLiteral(5),
            Token::Plus,
            Token::FloatLiteral(5.5),
            Token::Assignment,
            Token::IntLiteral(11),
            Token::Minus,
            Token::FloatLiteral(0.5),
            Token::NewLine
        ];
        let tokens = tokenize(&program);
        assert_eq!(tokens, expected_tokens);
    }

}