mod compiler;
use duohexyne::strings;
use std::env;
use std::process::exit;

#[allow(non_snake_case)]
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 { eprintln!("{}\n\n{}", strings::parse(strings::Errors::NoArgument), strings::command(strings::Commands::Help)); exit(1); }

    // Parsing parameters
    let mut inputFile = String::new();
    let mut outputFile = String::new();
    let mut index: usize = 1;

    while index < args.len() {
        let value = args[index].to_string();
        if value == "-h" || value == "--help" { println!("{}", strings::command(strings::Commands::Help)); exit(0);}
        if value == "-o" || value == "--output" {
            index += 1;
            if index >= args.len() { eprintln!("{}", strings::parse(strings::Errors::NoOutputFilename)); exit(1); }
            if outputFile.is_empty() { outputFile = args[index].to_string().clone();}
        } else {
            if inputFile.is_empty() || !inputFile.starts_with('-') { inputFile = value.clone(); }
        }
        index += 1;
    }

    // Checking filenames & Compiling
    if inputFile.is_empty() { eprintln!("{}", strings::parse(strings::Errors::NoInputFilename)); exit(1); }
    if outputFile.is_empty() { outputFile = "out".to_string(); }
    if !inputFile.contains(".") { inputFile += ".hxy"; }

    println!("Input: {} :: Output: {}\n", inputFile, outputFile); // debug

    compiler::compiler::compile(inputFile);
}