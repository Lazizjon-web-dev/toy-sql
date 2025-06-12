#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Operator(Op),
    Identifier(String),
    Literal(String),
    Comment(String),
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Select,
    Insert,
    Delete,
}
use Keyword::*;

#[derive(Debug, PartialEq)]
pub enum Op {
    // Logical Operators
    And,
    Or,
    Not,
    // Arithmetic Operators
    Addition,
    Substraction,
    Multiplication,
    Division,
    // Comparison Operators
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}
use Op::*;

pub struct Lexer {
    input: String,
}
