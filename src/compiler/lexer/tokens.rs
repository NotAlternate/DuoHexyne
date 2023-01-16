#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Token {
	pub lineNum:   usize,       // Which line is it     (Used for error messages)
	pub start: usize,           // Where does it starts (Used for error messages)
	pub end:   usize,           // Where does it end    (Used for error messages)
    pub value:     String,      // "func"               (Used to debug)
    pub tokenType: TokenType,   // TokenType::Function
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum TokenType {
    // Symbols
    Semicolon,           // ;
    DoubleColon,         // 99% of the time used to make the syntax cool
    Colon,               // local:io
                         //      ^
    // Comparison
    EqualsTo,            // 2 =  3  False
    NotEqualsTo,         // 2 != 3  True
    GreaterThan,         // 2 >  3  False
    GreaterThanEquals,   // 2 >= 3  False
    LessThan,            // 2 <  3  True
    LessThanEquals,      // 2 <= 3  True

    // Parenthesis & Braces
    OpenParenthesis,     // (
    CloseParenthesis,    // )
    OpenBraces,          // {
    CloseBraces,         // }

    // Arithmetic
    Plus,                // 2 + 3 =  5
    Minus,               // 2 - 3 = -1
    Multiplication,      // 2 * 3 =  6
    Division,            // 2 / 3 = .. (very long decimal)
    Modulus,             // 2 % 3 = .. (lazy)

    // Data types
    Void,                // (No return type)
    Integer,             // 256     (Non-decimals)
    Float,               // 420.69  (Decimals)
    String,              // "Series of characters"
    Character,           // 'h'     (Single characters)
    Boolean,             // True/False

    // Conditional
    While,               // Loop ... if conditions are true.
    IfStatement,         // If ... condition are true, run ...
    ElseStatement,       // If all conditions are false, run ...

    // Functions
    Function,            // func <--
    ReturnType,          // (Token used in functions to denote return type)
                         // func main -> int
    // Modules                        ^^^
    Import,              // import ...;              (importing a module to use)
    As,                  // import ... as ...;       (import module with a different name)
    Without,             // import ... without ...;  (import everything excluding ...)

    // Variables
    Assignment,          // var ... = ...;      (Unable to change value afterwards)
    Mutable,             // mut var ... = ...;  (Able to change value afterwards)

    // Other
    Identifier,          // Variable name, Function name, Module name, etc
    Number               // numberic.
}