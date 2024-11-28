mod ast;
mod interpreter;
mod lexer;
mod parser;

use ast::ASTDisplay;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    #[arg(
        short = 'p',
        long = "path",
        help = "Path to the source file to interpret."
    )]
    path: String,

    #[arg(
        short = 's',
        long = "show-ast",
        help = "Displays the generated Abstract Syntax Tree (AST) from the source code."
    )]
    show_ast: bool,
}

// cargo run -- -p "./example.txt" -s

fn main() {
    let args = Args::parse();

    let content = fs::read_to_string(args.path).expect("Failed to read file");

    let tokens = lexer::lex(&content);
    let ast = parser::parse(&tokens);

    if args.show_ast {
        ast.display();
    }

    interpreter::execute(&ast);
}
