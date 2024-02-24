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
    SemiColon,
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

pub fn print_tokens(tokens: &Vec<Token>) {
    for (i, token) in tokens.iter().enumerate() {
        println!("{}\t| {:?}", i, token);
    }
}
