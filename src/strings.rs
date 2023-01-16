pub enum Errors {
    NoArgument,
    NoInputFilename,
    NoOutputFilename,
    FileReadErr,
    BufferGetCharacter,
} pub fn parse(code: Errors) -> String {
    let symbol: &str = "\x1b[31;1mxx\x1b[0m";
    match code {
        Errors::NoArgument => format!("{} No arguments passed.", symbol),
        Errors::NoInputFilename => format!("{} No input filename given to compile.", symbol),
        Errors::NoOutputFilename => format!("{} No output filename given for flag `-o` or `--output`", symbol),
        Errors::FileReadErr => format!("{} Unable to read input file.", symbol),
        Errors::BufferGetCharacter => format!("{} Unable to get character in Buffer", symbol),
    }
}

pub enum Commands {
    Help,
} pub fn command(code: Commands) -> String {
    match code {
        Commands::Help => "\x1b[1mDuoHexyne v0.1.0\x1b[0m\nA \x1b[4mcompiled\x1b[0m programming language.\n\nProgram parameters:\n    \x1b[1mDuoHexyne\x1b[0m <INPUT> [FLAGS]\n\nProgram flags:\n    \x1b[1m-h\x1b[0m & \x1b[1m--help\x1b[0m  ::  Shows DuoHexyne's description and usage.".to_string(),
    }
}