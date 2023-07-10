use std::{io, fmt};
use std::collections::HashSet;

#[derive(Clone)]
 enum Token {
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
            Token::OpenCurlyBrace => write!(f, "OPEN CURLY BRACE TOKEN"),
            Token::CloseCurlyBrace => write!(f, "CLOSE CURLY BRACE TOKEN"),
            Token::Plus => write!(f, "PLUS TOKEN"),
            Token::Minus => write!(f, "MINUS TOKEN"),
            Token::Multi => write!(f, "MULTI TOKEN"),
            Token::Devide => write!(f, "DEVIDE TOKEN"), 
        }
    }
}

// fn tokenize(program: &str) -> Vec<Token> {
//     let mut result: Vec<Token> = Vec::new();
//     for line in program.lines().clone() {
//         let splited_word = line.split(" ");
//         for word in splited_word {
//             if !word.is_empty() {
//                 if let Some(tokens) = str_to_token(word) {
//                     for token in tokens {
//                         result.push(token);
//                     }
//                 } else {
//                     println!("can't parse string {} as token", word)
//                 }
//             }
//         }
//     }
//     return result;
// }


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
        '*' =>      Some(Token::Minus),
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
    }  else if let Some(token) = read_string_literal_token(source) {
        return Some(token);
    } 
    println!("return none from read token");
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
        println!("return {f}");
        return Some(Token::FloatLiteral(f));
    } else if let Ok(i) = result_string.parse::<i64>() {
        println!("return {i}");
        return Some(Token::IntLiteral(i));
    }
    None
}


fn read_reserved_token(source: &mut String) -> Option<Token> { 
    // @todo write source.clear somewhere once
    if source == "exec" {
        source.clear();
        return Some(Token::Exec);
    } else if source == "var" {
        source.clear();
        return Some(Token::Var);
    } 
    None
}

fn read_string_literal_token(source: &mut String) -> Option<Token> {
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
    return Some(Token::StringLiteral(result));
}


//     for line in program.lines().clone() {
//         let splited_word = line.split(" ");
//         for word in splited_word {
//             if !word.is_empty() {
//                 if let Some(tokens) = str_to_token(word) {
//                     for token in tokens {
//                         result.push(token);
//                     }
//                 } else {
//                     println!("can't parse string {} as token", word)
//                 }
//             }
//         }
//     }
//     return result;



fn tokenize(program: &String) -> Vec<Token> {
    let mut program_local = program.clone();
    let mut result : Vec<Token> = Vec::new();
    for line in program.lines().clone() {
        println!("new line {line}");
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
    }

    // while !program_local.is_empty() {
    //     println!("program local {program_local}");
    //     if let Some(token) = read_token_from_char(&mut program_local) {
    //         result.push(token)
    //     } else if let Some(token) = read_token_from_string(&mut program_local) {
    //         result.push(token)    
    //     } else {
    //         break;
    //     }
    // }
    return result;
}

fn get_reserved_chars() -> HashSet<char> {
    let mut reserved : HashSet<char> = HashSet::new();
    reserved.insert(';');
    reserved.insert('(');
    reserved.insert(')');
    reserved.insert('{');
    reserved.insert('}');
    reserved.insert('=');
    reserved.insert('+');
    reserved.insert('-');
    reserved.insert('*');
    reserved.insert('/');
    return reserved;
}


fn main() -> io::Result<()>{
    let mut program = "(4.44+a+33-22.3 sdfsdf)){})\n 1 + 2";
    //program = "3 + 2 + {}";
    let tokens = tokenize(&program.to_string());
    for token in tokens {
        println!("token is {}", token.to_string());
    }
    Ok(())    
}
