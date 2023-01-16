use std::fs;
use std::process::exit;
use crate::strings;
// use crate::compiler::lexer;
// use crate::compiler::ast::ast;

#[allow(non_snake_case)]
#[allow(unused_assignments)]
pub fn compile(inputFile: String) {
    let mut source = String::new();
    let data = fs::read_to_string(inputFile);
    match data {
        Ok(content) => { source = content; }
        Err(error) => { eprintln!("{}  ::  {}", strings::parse(strings::Errors::FileReadErr), error); exit(1); }
    }
    source = format!("\n{}", source);

    println!("File:\n{}\n\n", source);
    // let tokens = lexer::lexer::tokenize(&source);
    // println!("Tokens:\n{:?}\n\n", tokens); // debug
    // let ast = ast::ASTize(tokens);
    // println!("AST:\n{:?}", ast); // debug
}