use std::iter::{self, from_fn};

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
    Add, // Addition
    Sub, // Substraction,
    Mul, // Multiplication,
    Div, // Division,
    // Comparison Operators
    Eq,  // Equal,
    Neq, // NotEqual,
    Lt,  // LessThan,
    Gt,  // GreaterThan,
    Lte, // LessThanOrEqual,
    Gte, // GreaterThanOrEqual,
    // Punctuation
    Comma,
    LParen, // LeftParenthesis,
    RParen, // RightParenthesis,
    Colon,
}
use Op::*;

impl Op {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Sub),
            '*' => Some(Self::Mul),
            '/' => Some(Self::Div),
            '=' => Some(Self::Eq),
            ',' => Some(Self::Comma),
            '(' => Some(Self::LParen),
            ')' => Some(Self::RParen),
            ':' => Some(Self::Colon),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "!=" => Some(Op::Neq),
            "<>" => Some(Op::Neq),
            "<=" => Some(Op::Lte),
            ">=" => Some(Op::Gte),
            "<" => Some(Op::Lt),
            ">" => Some(Op::Gt),
            "!" => Some(Op::Not),
            _ => None,
        }
    }
}

pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn tokenize(&self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = vec![];
        let mut iter = self.input.chars().peekable();

        while let Some(ch) = iter.next() {
            match ch {
                ' ' | '\t' | '\n' => continue, // Skip whitespace
                '"' => {
                    let value = iter::once(ch)
                        .chain(from_fn(|| {
                            let mut is_escaped = false;
                            iter.by_ref().next_if(|s| {
                                if s == &'\\' {
                                    is_escaped = true;
                                }

                                if s == &'"' {
                                    if is_escaped {
                                        is_escaped = false;
                                    } else {
                                        return false;
                                    }
                                }

                                true
                            })
                        }))
                        .collect::<String>();
                    tokens.push(Token::Literal(value));
                }
                _ => return Err(LexerError::UnrecognizedToken),
            }
        }

        Ok(tokens)
    }
}

#[derive(Debug)]
pub enum LexerError {
    UnrecognizedToken,
    InvalidNumberFormat,
}
