use std::fs;

use rust_c_compiler::generator::generate;
use rust_c_compiler::lexer::{print_tokens, tokenize, Token};
use rust_c_compiler::parser::{parse_tokens, ParseNode};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    source_file: String,
    output_file: String,
}

fn read_file_contents(file_path: String) -> String {
    let result = fs::read_to_string(file_path);
    match result {
        Ok(content) => return content,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }
}

fn write_to_file(file_path: String, content: String) {
    fs::write(file_path, content).unwrap();
}

fn main() {
    let args = Args::parse();
    let content = read_file_contents(args.source_file);

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
            return;
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
            return;
        }
    }

    println!("\nAST Tree:");
    ast.print(0);

    let program = generate(&ast);

    println!("\nGenerated Assembly:");
    println!("{}", program);

    write_to_file(args.output_file, program);
}
