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

fn main() {
    let args = Args::parse();
    let content = read_file_contents(args.file_path);
    println!("{}", content);
}
