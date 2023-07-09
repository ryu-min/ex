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




fn tokenize(program: &str) {
    for line in program.lines().clone() {
        let splited_word = line.split(" ");
        for word in splited_word {
            if !word.is_empty() {
                if let Some(token) = str_to_token(word) {
                    println!{"find token {}", token.to_string()}
                } else {
                    println!("can't parse string {} as token", word)
                }
            }
        }
    }
}

fn str_to_token(str_value: &str) -> Option<Token>{
    match str_value {
        "var" =>    Some(Token::Var),
        "." =>      Some(Token::Dot),
        "=" =>      Some(Token::Assignment),
        "exec" =>   Some(Token::Exec),
        "(" =>      Some(Token::OpenBrace),
        ")" =>      Some(Token::CloseBrace),
        "{" =>      Some(Token::OpenCurlyBrace),
        "}" =>      Some(Token::CloseCurlyBrace),
        _ => {
            let int_convert_result = str_value.parse::<i32>();
            if int_convert_result.is_ok() {
                return Some(Token::IntLiteral(int_convert_result.unwrap()))
            }
            return Some(Token::StringLiteral(str_value.to_string()))
        }
    }
}


fn main() -> io::Result<()>{
    let program = "some = (exec) ( ) 32 32.exe { } 44.22 .";
    tokenize(program);
    Ok(())    
}
