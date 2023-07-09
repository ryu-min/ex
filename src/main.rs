use std::{io, fmt};


 enum Token {
    Var,
    Dot,
    Assignment,
    OpenBrace,
    CloseBrace,
    OpenCurlyBrace,
    CloseCurlyBrace,
    StringLiteral(String),
    IntLiteral(i32),
    Exec
 }
 impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Var => write!(f, "VAR TOKEN"),
            Token::Dot => write!(f, "DOT TOKEN"),
            Token::Assignment => write!(f, "ASSIGNMENT TOKEN"),
            Token::StringLiteral(s) => write!(f, "STRING LITERAL TOKEN WITH VALUE: {}", s),
            Token::IntLiteral(i) => write!(f, "INT LITERAL TOKEN WITH VALUE: {}", i),
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
                if let Some(token) = str_to_token(word) {
                    result.append(token);
                } else {
                    println!("can't parse string {} as token", word)
                }
            }
        }
    }

    return result;
}

fn str_to_token(str_value: &str) -> Option<Vec<Token>>{
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
            let int_convert_result = str_value.parse::<i32>();
            if int_convert_result.is_ok() {
                return Some(vec![Token::IntLiteral(int_convert_result.unwrap())])
            }
            return Some(vec![Token::StringLiteral(str_value.to_string())])
        }
    }
}


fn main() -> io::Result<()>{
    let program = "some = (exec) ( ) 32 32.exe { } 44.22 .";
    let mut tokens: Vec<Token> = Vec::new();
    tokens.push(Token::Assignment);
    tokens.push(Token::OpenBrace);
    tokens.push(Token::CloseBrace);
    for token in tokens {
        println!("token is vec {}", token.to_string());
    }
    tokenize(program);
    Ok(())    
}
