use std::{io, fmt};
use std::collections::HashSet;

#[derive(Clone)]
 enum Token {
    Var,
    Dot,
    Assignment,
    OpenBrace,
    CloseBrace,
    OpenCurlyBrace,
    CloseCurlyBrace,
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    Exec
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
            Token::OpenCurlyBrace => write!(f, "OPEN CURLY BRACE"),
            Token::CloseCurlyBrace => write!(f, "CLOSE CURLY BRACE"), 
        }
    }
}

fn tokenize(program: &str) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    for line in program.lines().clone() {
        let splited_word = line.split(" ");
        for word in splited_word {
            if !word.is_empty() {
                if let Some(tokens) = str_to_token(word) {
                    for token in tokens {
                        result.push(token);
                    }
                } else {
                    println!("can't parse string {} as token", word)
                }
            }
        }
    }
    return result;
}

fn str_to_token(str_value: &str) -> Option<Vec<Token>>{
    let mut source = str_value.to_string();
    if str_value.is_empty() {
        return Some(Vec::new());
    }
    match str_value {
        "var" =>    Some(vec![Token::Var]),
        "." =>      Some(vec![Token::Dot]),
        "=" =>      Some(vec![Token::Assignment]),
        "exec" =>   Some(vec![Token::Exec]),
        "(" =>      Some(vec![Token::OpenBrace]),
        ")" =>      Some(vec![Token::CloseBrace]),
        "{" =>      Some(vec![Token::OpenCurlyBrace]),
        "}" =>      Some(vec![Token::CloseCurlyBrace]),
        _ => {
            if let Ok(i) = str_value.parse::<i64>() {
                return Some(vec![Token::IntLiteral(i)]);
            }
            if let Ok(f) = str_value.parse::<f64>() {
                return Some(vec![Token::FloatLiteral(f)]);
            } 
            if let Some(first_char) = str_value.chars().next() {
                if first_char == ')' || first_char == '(' {
                    if let Some(mut first_tokens) = str_to_token(first_char.to_string().as_str()) {
                        if let Some(second_tokens) = str_to_token(&str_value[1..]) {
                            for token in second_tokens {
                                first_tokens.push(token)
                            }
                        }
                        return Some(first_tokens);
                    }
                    else {
                        assert!(true, "unreachable code");
                        return None
                    }
                }
                else {
                    if let Some(str_literal) = read_string_literal(&mut source) {
                        let mut result = Vec::new();
                        result.push(str_literal);
                        if let Some(other_tokens) = str_to_token(&mut source) {
                            for token in other_tokens {
                                result.push(token);
                            }
                        }
                        return Some(result);
                     } else {
                        return Some(Vec::new());
                     }     
                }
            }
            None
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
        _ =>        None
    }
}

fn string_to_token(source: &str) -> Option<Token>{
    match source {
        "var" =>    Some(Token::Var),
        "exec" =>   Some(Token::Exec),
        _      =>   {
            if let Ok(i) = source.parse::<i64>() {
                return Some(Token::IntLiteral(i));
            }
            else if let Ok(f) = source.parse::<f64>() {
                return Some(Token::FloatLiteral(f));
            }
            else {
                return Some(Token::StringLiteral(source.to_string()))
            }
        }
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
    let mut word = String::new();
    for ch in source.chars() {
        if ch.is_alphanumeric() && !get_reserved_chars().contains(&ch) {
            word.push(ch)
        }
    }
    println!("word is {word}");
    if word.is_empty() {
        return None
    }
    if let Some(token) = string_to_token(&word) {
        source.replace_range(0..word.len(), "");
        return Some(token);
    }
    None
}

/// @todo
/// in tokenize method
/// untill programm is not empty
/// 1. try to read char
/// 2. try to read string
/// collect all tokens in progress





fn read_string_literal(source: &mut String) -> Option<Token> {
    let mut result = String::new();
    for ch in source.chars() {
        if ch.is_alphanumeric() {
            result.push(ch);
        } else {
            break;
        }
    }
    if !result.is_empty() {
        source.replace_range(0..result.len(), "");
        if let Some( tokens ) = str_to_token(&result) {
            if tokens.len() == 1 {
                return Some(tokens[0].clone());
            } 
        }
    }
    None
}




fn get_reserved_chars() -> HashSet<char> {
    let mut reserved : HashSet<char> = HashSet::new();
    reserved.insert(';');
    reserved.insert('(');
    reserved.insert(')');
    reserved.insert('{');
    reserved.insert('}');
    reserved.insert('=');
    return reserved;
}


fn main() -> io::Result<()>{




    let program = "4.44)";
    let tokens = tokenize(program);
    for token in tokens {
        println!("token is {}", token.to_string());
    }
    Ok(())    
}
