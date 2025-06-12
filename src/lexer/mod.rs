#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(String),
    Literal(String),
    Operator(String),
    Punctuation(String),
    Comment(String),
    Whitespace(String),
}

pub struct Lexer {
    input: String,
}
