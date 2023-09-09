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
    Dot,
    Comma,
    Assignment,
    Plus,
    Minus,
    Multi,
    Devide,
    OpenBracket,
    CloseBracket,
    OpenCurlyBracket,
    CloseCurlyBraket,
    OpenSquareBracket,
    CloseSquareBracket,
    StringLiteral(String),
    Name(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    Exec,
    Fn,
    Return,
    While,
    For,
    In,
    If,
    Else,
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
            Token::Dot => write!(f, "DOT TOKEN"),
            Token::Comma => write!(f, "COMMA TOKEN"),
            Token::Assignment => write!(f, "ASSIGNMENT TOKEN"),
            Token::StringLiteral(s) => write!(f, "STRING LITERAL TOKEN WITH VALUE: {}", s),
            Token::IntLiteral(n) => write!(f, "INT LITERAL TOKEN WITH VALUE: {}", n),
            Token::FloatLiteral(n) => write!(f, "FLOAT LITERAL WITH VALUE: {}", n),
            Token::Exec => write!(f, "EXEC TOKEN"),
            Token::OpenBracket => write!(f, "OPEN BRACE TOKEN"),
            Token::CloseBracket => write!(f, "CLOSE BRACE TOKEN"),
            Token::OpenCurlyBracket => write!(f, "OPEN CURLY BRACE TOKEN"),
            Token::CloseCurlyBraket => write!(f, "CLOSE CURLY BRACE TOKEN"),
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
            Token::If => write!(f, "IF TOKEN"),
            Token::Else => write!(f, "ELSE TOKEN"),
            Token::While => write!(f, "WHILE TOKEN"),
            Token::For => write!(f, "FOR TOKEN"),
            Token::OpenSquareBracket => write!(f, "OPEN SQUARE BRACKET TOKEN"),
            Token::CloseSquareBracket => write!(f, "CLOSE SQUARE BRACKET TOKEN"),
            Token::In => write!(f, "IN TOKEN"),
        }
    }
}

fn char_to_simple_token(ch : char) -> Option<Token> {
    match ch {
        '.' =>      Some(Token::Dot),
        '(' =>      Some(Token::OpenBracket),
        ')' =>      Some(Token::CloseBracket),
        '{' =>      Some(Token::OpenCurlyBracket),
        '}' =>      Some(Token::CloseCurlyBraket),
        '[' =>      Some(Token::OpenSquareBracket),
        ']' =>      Some(Token::CloseSquareBracket),
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
    if let Some(dot_pos) = result_string.find('.') {
        if dot_pos == result_string.len() - 1 {
            if let Ok(i) = result_string[..dot_pos].parse::<i64>() {
                source.replace_range(0..result_string.len() - 1, "");
                return Some(Token::IntLiteral(i));
            }
        }
        if let Ok(f) = result_string.parse::<f64>() {
            source.replace_range(0..result_string.len(), "");
            return Some(Token::FloatLiteral(f));
        }
    } else if let Ok(i) = result_string.parse::<i64>() {
        source.replace_range(0..result_string.len(), "");
        return Some(Token::IntLiteral(i));
    }
    None
}



fn read_reserved_token(source: &mut String) -> Option<Token> { 
    if try_read_reserved_word("fn", source) {
        return Some(Token::Fn);
    } else if try_read_reserved_word("while", source) {
        return Some(Token::While);
    } else if try_read_reserved_word("for", source) {
        return Some(Token::For);
    }  else if try_read_reserved_word("in", source) {
        return Some(Token::In);
    } else if try_read_reserved_word("if", source) {
        return Some(Token::If);
    }  else if try_read_reserved_word("else", source) {
        return Some(Token::Else);
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
    
    let reserved_with_open_brace = reserved.to_owned() + "(";
    let reserved_with_close_brace = reserved.to_owned() + ")";

    let reserved_with_open_square_brace = reserved.to_owned() + "[";
    let reserved_with_close_square_brace = reserved.to_owned() + "]";

    if source == reserved 
    || source.starts_with(&reserved_with_space) 
    || source.starts_with(&reserved_with_open_brace) 
    || source.starts_with(&reserved_with_close_brace) 
    || source.starts_with(&reserved_with_open_square_brace) 
    || source.starts_with(&reserved_with_close_square_brace) {
        source.replace_range(..reserved.len(), "");
        return true;
    }
    return false;
} 

// TODO: think about reading everything untill escape symbol??
fn read_name_token(source: &mut String) -> Option<Token> {
    let mut result = String::new();
    for ch in source.chars() {
        if ch.is_alphanumeric() || ch == '_' {
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
        let program: String = String::from("x = 10 \n\
                                            y = 20.5 \n\
                                            s = \"str\" \\
                                            x = x * ( x + y ) \\
                                            a = true \n\
                                            c = a == true\n\
                                            bu = c != false \n\
                                            boo1 = 1 >= 2 \n\
                                            boo2 = 1 > 2 \n\
                                            boo3 = 1 <= 2 \n\
                                            boo4 = 1 < 2 \n\
                                            if (boo4 == true) doSomething()\n\
                                            else doNothing()\n\
                                            while(true) tododo() \n\
                                            for i in [2, 44] topudo() \n\
                                            a = b.to_int()\n");
        let expected_tokens = vec![
            Token::Name(String::from("x")),
            Token::Assignment,
            Token::IntLiteral(10),
            Token::NewLine,

            Token::Name(String::from("y")),
            Token::Assignment,
            Token::FloatLiteral(20.5),
            Token::NewLine,

            Token::Name(String::from("s")),
            Token::Assignment,
            Token::StringLiteral(String::from("str")),
            Token::NewLine,

            Token::Name(String::from("x")),
            Token::Assignment,
            Token::Name(String::from("x")),
            Token::Multi,
            Token::OpenBracket,
            Token::Name(String::from("x")),
            Token::Plus,
            Token::Name(String::from("y")),
            Token::CloseBracket,
            Token::NewLine,

            Token::Name(String::from("a")),
            Token::Assignment,
            Token::True,
            Token::NewLine,

            Token::Name(String::from("c")),
            Token::Assignment,
            Token::Name(String::from("a")),
            Token::Eq,
            Token::True,
            Token::NewLine,

            Token::Name(String::from("bu")),
            Token::Assignment,
            Token::Name(String::from("c")),
            Token::NotEq,
            Token::False,
            Token::NewLine,

            Token::Name(String::from("boo1")),
            Token::Assignment,
            Token::IntLiteral(1),
            Token::MoreEq,
            Token::IntLiteral(2),
            Token::NewLine,

            Token::Name(String::from("boo2")),
            Token::Assignment,
            Token::IntLiteral(1),
            Token::More,
            Token::IntLiteral(2),
            Token::NewLine,

            Token::Name(String::from("boo3")),
            Token::Assignment,
            Token::IntLiteral(1),
            Token::LessEq,
            Token::IntLiteral(2),
            Token::NewLine,

            Token::Name(String::from("boo4")),
            Token::Assignment,
            Token::IntLiteral(1),
            Token::Less,
            Token::IntLiteral(2),
            Token::NewLine,   

            Token::If,
            Token::OpenBracket,
            Token::Name(String::from("boo4")),
            Token::Eq,
            Token::True,
            Token::CloseBracket,
            Token::Name(String::from("doSomething")),
            Token::OpenBracket,
            Token::CloseBracket,
            Token::NewLine,
 
            Token::Else,
            Token::Name(String::from("doNothing")),
            Token::OpenBracket,
            Token::CloseBracket,
            Token::NewLine,

            Token::While,
            Token::OpenBracket,
            Token::True,
            Token::CloseBracket,
            Token::Name(String::from("tododo")),
            Token::OpenBracket,
            Token::CloseBracket,
            Token::NewLine,

            Token::For,
            Token::Name(String::from("i")),
            Token::In,
            Token::OpenSquareBracket,
            Token::IntLiteral(2),
            Token::Comma,
            Token::IntLiteral(44),
            Token::CloseSquareBracket,
            Token::Name(String::from("topudo")),
            Token::OpenBracket,
            Token::CloseBracket,
            Token::NewLine,

            Token::Name(String::from("a")),
            Token::Assignment,
            Token::Name(String::from("b")),
            Token::Dot,
            Token::Name(String::from("to_int")),
            Token::OpenBracket,
            Token::CloseBracket,
            Token::NewLine
        ];
        let tokens = tokenize(&program);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn number_tests() {
        let program1: String = String::from("5 + 5.5 = 11 - 0.5");
        let expected_tokens1 = vec![
            Token::IntLiteral(5),
            Token::Plus,
            Token::FloatLiteral(5.5),
            Token::Assignment,
            Token::IntLiteral(11),
            Token::Minus,
            Token::FloatLiteral(0.5),
            Token::NewLine
        ];
        let tokens1 = tokenize(&program1);
        assert_eq!(tokens1, expected_tokens1);


        let program2 = String::from("2.pow(2)");
        let expected_tokens2 = vec![
            Token::IntLiteral(2),
            Token::Dot,
            Token::Name(String::from("pow")),
            Token::OpenBracket,
            Token::IntLiteral(2),
            Token::CloseBracket,
            Token::NewLine
        ];
        let tokens2 = tokenize(&program2);
        assert_eq!(tokens2, expected_tokens2);
    }

    #[test]
    fn var_name_test() {
        let program: String = String::from("to_guess = read(\"Enter your number to guess:\")");
        let expected_tokens = vec![
            Token::Name(String::from("to_guess")),
            Token::Assignment,
            Token::Name(String::from("read")),
            Token::OpenBracket,
            Token::StringLiteral(String::from("Enter your number to guess:")),
            Token::CloseBracket,
            Token::NewLine,
        ];
        let tokens = tokenize(&program);
        assert_eq!(tokens, expected_tokens);  
    }


}