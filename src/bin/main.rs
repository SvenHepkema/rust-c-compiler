use std::fs;

use rust_c_compiler::generator::{generate_assembly, generate_operations, print_operations};
use rust_c_compiler::lexer::{print_tokens, tokenize, Token};
use rust_c_compiler::parser::{parse_tokens, ParseNode, print_ast};

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
        Ok(content) => content,
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

    let tokens: Vec<Token> = match tokenize(content) {
        Ok(lexed_tokens) => lexed_tokens,
        Err(reason) => {
            println!("Encountered error during the tokenizing step: {}", reason);
            return;
        }
    };

    println!("\nTokens:");
    print_tokens(&tokens);

    let ast: ParseNode = match parse_tokens(tokens) {
        Ok(parsed_ast) => parsed_ast,
        Err(reason) => {
            println!("Encountered error during the parser step: {}", reason);
            return;
        }
    };

    println!("\nAST Tree:");
    print_ast(&ast);

    let operations = generate_operations(&ast);
    print_operations(&operations);

    let assembly = generate_assembly(&operations);

    println!("\nGenerated Assembly:");
    println!("{}", assembly);

    write_to_file(args.output_file, assembly);
}
