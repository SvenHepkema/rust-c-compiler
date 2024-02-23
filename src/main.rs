use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_path: String,
}

fn read_file_contents(file_path: String) -> String {
    let result = fs::read_to_string(file_path);
    match result {
        Ok(content) => return content,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}

#[derive(Debug)]
enum Token {
    Return,
    Int, // keyword int
    Identifier(String),
    Integer(i32),
    OpeningBracket,
    ClosingBracket,
    OpeningCurly,
    ClosingCurly,
    SemiColon,
}

fn tokenize(program_text: String) -> Result<Vec<Token>, String> {
    // Heavily inspired/copied from https://github.com/onehr/crust/blob/toy/src/lexer.rs
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
                        '0'..='9' => { // Only works on integers for now
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
                tokens.push(Token::OpeningBracket);
            }
            ')' => {
                it.next();
                tokens.push(Token::ClosingBracket);
            }
            '{' => {
                it.next();
                tokens.push(Token::OpeningCurly);
            }
            '}' => {
                it.next();
                tokens.push(Token::ClosingCurly);
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

fn main() {
    let args = Args::parse();
    let content = read_file_contents(args.file_path);

    println!("{}", content);

    let tokenized_content = tokenize(content);

    match tokenized_content {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(reason) => {
            println!("Encontered error during the tokenizing step: {}", reason);
        }
    }
}
