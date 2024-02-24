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

#[derive(Eq, PartialEq, Clone, Debug)]
enum Token {
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

fn print_tokens(tokens: &Vec<Token>) {
    for (i, token) in tokens.iter().enumerate() {
        println!("{}\t| {:?}", i, token);
    }
}


#[derive(Eq, PartialEq, Clone, Debug)]
enum NodeType {
    Prog(String),
    Fn(String),
    Stmt,
    Exp(i64),
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct ParseNode {
    entry: NodeType,
    children: Vec<ParseNode>,
}


// ===========================
// ===========================
// Copied (and edited) from https://github.com/onehr/crust/commit/a708fbc04bf395425197a11ca1f0fde0ed49d865#diff-4a04259da480a6b794a2e947e4cc03eff4d1aa9330836f5b91cac68c5398193f
// ===========================
// ===========================

impl ParseNode {
    fn new() -> ParseNode {
        ParseNode {
            children: Vec::new(),
            entry: NodeType::Prog("root".to_string()),
        }
    }

    fn print(&self, indent: usize) {
        println!("{}| {:?}", "  ".repeat(indent), self.entry);
        for child in self.children.iter() {
            child.print(indent + 1);
        }
    }
}

fn parse_function(
    toks: &Vec<Token>,
    pos: usize,
) -> Result<(ParseNode, usize), String> {
    let tok = &toks[pos];
    if *tok != Token::Int { // TODO: Chnage this to Token::Keyword::Type and then check if int
                            // float etc.
        return Err(format!("Expected `int`, found {:?} at {}", toks[pos], pos));
    }
    let mut pos = pos + 1;

    let tok = &toks[pos];
    if *tok != Token::Identifier("main".to_string()) {
        return Err(format!("Expected `main`, found {:?} at {}", toks[pos], pos));
    }
    pos = pos + 1;

    let tok = &toks[pos];
    if *tok != Token::LParenthesis {
        return Err(format!("Expected `(`, found {:?} at {}", toks[pos], pos));
    }
    pos = pos + 1;

    let tok = &toks[pos];
    if *tok != Token::RParenthesis {
        return Err(format!("Expected `)`, found {:?} at {}", toks[pos], pos));
    }
    pos = pos + 1;

    let tok = &toks[pos];
    if *tok != Token::LCurly {
        return Err(format!("Expected `{{`, found {:?} at {}", toks[pos], pos));
    }
    pos = pos + 1;

    let tmp = parse_statement(toks, pos);
    let mut stmt_node = ParseNode::new();
    match tmp {
        Ok((a,b)) => {stmt_node = a; pos = b;},
        Err(_) => {},
    }

    let tok = &toks[pos];
    if *tok != Token::RCurly{
        return Err(format!("Expected `}}`, found {:?} at {}", toks[pos], pos));
    }
    pos = pos + 1;

    let mut fn_node = ParseNode::new();
    fn_node.entry = NodeType::Fn("main".to_string());
    fn_node.children.push(stmt_node);

    Ok((fn_node, pos))
}

fn parse_statement(
    toks: &Vec<Token>,
    pos: usize,
) -> Result<(ParseNode, usize), String> {
    let tok = &toks[pos];
    if *tok != Token::Return {
        return Err(format!("Expected 'return', found {:?} at {}", toks[pos], pos));
    }
    let mut pos = pos + 1;

    let tmp = parse_expression(toks, pos);
    let mut exp_node = ParseNode::new();
    match tmp {
        Ok((a,b)) => {exp_node = a; pos = b;},
        Err(_) => {},
    }

    let tok = &toks[pos];
    if *tok != Token::SemiColon {
        return Err(format!("Expected ';', found {:?} at {}", toks[pos], pos));
    }
    pos = pos + 1;

    let mut stmt_node = ParseNode::new();
    stmt_node.entry = NodeType::Stmt;
    stmt_node.children.push(exp_node);

    Ok((stmt_node, pos))
}

fn parse_expression(
    toks: &Vec<Token>,
    pos: usize,
) -> Result<(ParseNode, usize), String> {
    let tok = &toks[pos];
    if *tok != Token::Integer(10) {
        panic!("Expected 'Integer(10)`, found {:?} at {}", toks[pos], pos);
    }
    let pos = pos + 1;

    let mut exp_node = ParseNode::new();
    exp_node.entry = NodeType::Exp(10);

    Ok((exp_node, pos))
}

fn parse_tokens(tokens: Vec<Token>) -> Result<ParseNode, String> {
    parse_function(&tokens, 0).and_then(|(n, i)| {
        if i == tokens.len() {
            Ok(n)
        } else {
            Err(format!(
                "Parsing Error: Expected end of input, found {:?} at {}",
                &tokens[i], i
            ))
        }
    })
}
//
// ===========================
// ===========================
// ===========================
// ===========================

fn main() {
    let args = Args::parse();
    let content = read_file_contents(args.file_path);

    println!("\nProgram:\n");
    println!("{}", content);

    let tokenized_content = tokenize(content);
    let tokens: Vec<Token>;

    // Probably better/cleaner/more idiomatic way to do this 
    match tokenized_content {
        Ok(lexed_tokens) => {
            tokens = lexed_tokens;
        }
        Err(reason) => {
            println!("Encountered error during the tokenizing step: {}", reason);
            return
        }
    }

    println!("\nTokens:");
    print_tokens(&tokens);

    let parser_result = parse_tokens(tokens);
    let ast: ParseNode;

    // Probably better/cleaner/more idiomatic way to do this 
    match parser_result {
        Ok(parsed_ast) => {
            ast = parsed_ast;
        }
        Err(reason) => {
            println!("Encountered error during the parser step: {}", reason);
            return
        }
    }

    println!("\nAST Tree:");
    ast.print(0);
}
