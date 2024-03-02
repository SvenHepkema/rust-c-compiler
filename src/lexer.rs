// Heavily inspired/copied from https://github.com/onehr/crust/blob/toy/src/lexer.rs

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Token {
    Return,
    Int, // keyword int
    Identifier(String),
    Integer(i32),
    LParenthesis,
    RParenthesis,
    LCurly,
    RCurly,
    Plus,
    Minus,
    Multiplication,
    Assignment,
    SemiColon,
}

impl Token {
    pub fn is_identifier(&self) -> bool {
        match self {
            Token::Identifier(_) => true,
            _ => false
        }
    }
    pub fn is_operation(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus | Token::Multiplication)
    }
    pub fn get_string_value(&self) -> &String {
        match self {
            Token::Identifier(x) => x,
            _ => panic!("Could not get string value from non identifier token.")
        }
    }
}

pub fn tokenize(program_text: String) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut it = program_text.chars().peekable();

    while let Some(&c) = it.peek() {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                it.next();
                let mut s = String::new();
                s.push(c);
                while let Some(&tmp) = it.peek() {
                    match tmp {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            s.push(tmp);
                            it.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                match s.as_ref() {
                    "return" => tokens.push(Token::Return),
                    "int" => tokens.push(Token::Int),
                    _ => tokens.push(Token::Identifier(s)),
                }
            }

            '0'..='9' => {
                it.next();
                let mut s = String::new();
                s.push(c);
                while let Some(&tmp) = it.peek() {
                    match tmp {
                        '0'..='9' => {
                            // Only works on integers for now
                            s.push(tmp);
                            it.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                tokens.push(Token::Integer(s.parse::<i32>().unwrap()));
            }
            '+' => {
                it.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                it.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                it.next();
                tokens.push(Token::Multiplication);
            }
            '=' => {
                it.next();
                tokens.push(Token::Assignment);
            }
            '(' => {
                it.next();
                tokens.push(Token::LParenthesis);
            }
            ')' => {
                it.next();
                tokens.push(Token::RParenthesis);
            }
            '{' => {
                it.next();
                tokens.push(Token::LCurly);
            }
            '}' => {
                it.next();
                tokens.push(Token::RCurly);
            }
            ';' => {
                it.next();
                tokens.push(Token::SemiColon);
            }
            _ => {
                it.next();
            }
        }
    }

    Ok(tokens)
}

pub fn print_tokens(tokens: &[Token]) {
    for (i, token) in tokens.iter().enumerate() {
        println!("{}\t| {:?}", i, token);
    }
}
