use crate::strings;
use std::process::exit;
use crate::compiler::lexer::tokens;

// Parsing tokens
#[allow(non_snake_case)]
pub fn tokenize(src: &String) -> Vec<tokens::Token> {
    let mut value: Vec<tokens::Token> = Vec::new();
    let mut buffer = Buffer::new(src);
    let mut lineNum: usize = 0;

    while buffer.inBounds() {
        let data = buffer.getCharacter();

        if data == ' ' { buffer.advance(); continue; }
        else if data == '\n' { lineNum += 1; }

        value.push(tokens::Token {lineNum, value: "func".to_string(), start: buffer.getLinePos(&lineNum),end: buffer.getLinePos(&lineNum),
        tokenType: tokens::TokenType::Function}); // debug
        // STILL DEBUG, IF YOU RUN IT, IT WOULD FLOOD AS IT WILL PUSH TOKENTYPE::FUNCTION FOR EVERY CHARACTERS (END OF FILE ASWELL)

        buffer.advance()
    }

    value
}

// Buffer
struct Buffer {
	data: String,
	index: usize
}
#[allow(non_snake_case)]
#[allow(dead_code)]
impl Buffer {
    pub fn new(source: &String) -> Buffer { Buffer { data: source.clone(), index: 0 } }
    pub fn inBounds(&self) -> bool { self.index < self.data.len() }
    pub fn advance(&mut self) { self.index += 1 }
    pub fn next(&mut self) -> char { self.advance(); self.getCharacter() }
    pub fn getLinePos(&self, lineNum: &usize) -> usize { self.index - &self.data.match_indices(*&'\n').nth(*&(lineNum - 1)).unwrap().0 }
    pub fn getCharacter(&self) -> char { let result = match self.data.chars().nth(self.index) { Some(_result) => _result, None => { eprintln!("{}", strings::parse(strings::Errors::BufferGetCharacter)); exit(1); } }; result }
}