use std::fmt;


pub fn tokenize(program: &String) -> Vec<Token> {
    let mut result : Vec<Token> = Vec::new();
    for line in program.lines().clone() {
        let splited_word = line.split(" ");
        for word in splited_word { 
            let mut word_string = word.to_string();
            while !word_string.is_empty() {
                if let Some(token) = read_token_from_char(&mut word_string) {
                    result.push(token)
                } else if let Some(token) = read_token_from_string(&mut word_string) {
                    result.push(token)    
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
    NewLine
 }

 impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Var => write!(f, "VAR TOKEN"),
            Token::Dot => write!(f, "DOT TOKEN"),
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
            Token::NewLine => write!(f, "NEW LINE TOKEN")
        }
    }
}

fn char_to_token(ch : char) -> Option<Token> {
    match ch {
        '.' =>      Some(Token::Dot),
        '=' =>      Some(Token::Assignment),
        '(' =>      Some(Token::OpenBrace),
        ')' =>      Some(Token::CloseBrace),
        '{' =>      Some(Token::OpenCurlyBrace),
        '}' =>      Some(Token::CloseCurlyBrace),
        '+' =>      Some(Token::Plus),
        '-' =>      Some(Token::Minus),
        '/' =>      Some(Token::Devide),
        '*' =>      Some(Token::Multi),
        _ =>        None
    }
}

fn read_token_from_char(source: &mut String) -> Option<Token> {
    if let Some(ch) = source.chars().next() {
        if let Some(token) = char_to_token(ch) {
            source.replace_range(0..1, "");
            return Some(token)
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
    } else if let Some(token) = read_reserved_token(source) {
        return Some(token);
    } else if let Some(token) = read_string_literal_token(source) {
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
    if let Ok(f) = result_string.parse::<f64>() {
        return Some(Token::FloatLiteral(f));
    } else if let Ok(i) = result_string.parse::<i64>() {
        return Some(Token::IntLiteral(i));
    }
    None
}


fn read_reserved_token(source: &mut String) -> Option<Token> { 
    if source == "exec" {
        source.clear();
        return Some(Token::Exec);
    } else if source == "var" {
        source.clear();
        return Some(Token::Var);
    } 
    None
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
    source.replace_range(..end_literal, "");
    return Some(Token::StringLiteral(string_literal));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tokenizer_test() {
        
        let program = String::from("var x = 10 \n\
                                            var y = 20.5 \n\
                                            var s = \"str\" \\
                                            x = x * ( x + y )");
        let expected_tokens = vec![
            Token::Var,
            Token::Name(String::from("x")),
            Token::Assignment,
            Token::FloatLiteral(10.),
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
            Token::NewLine
        ];
        let tokens = tokenize(&program);
        assert_eq!(tokens, expected_tokens);

    }
}