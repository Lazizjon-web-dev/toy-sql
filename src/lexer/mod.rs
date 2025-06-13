use std::iter::{self, from_fn};

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Operator(Op),
    Identifier(String),
    Literal(String),
    Comment(String),
    Number(f64),
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
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }

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
                '0'..='9' => {
                    let mut is_float = false;
                    let num_str: String = iter::once(ch)
                        .chain(from_fn(|| {
                            iter.by_ref().next_if(|s| {
                                if s.is_ascii_digit() {
                                    true
                                } else if *s == '.' && !is_float {
                                    is_float = true;
                                    true
                                } else {
                                    false
                                }
                            })
                        }))
                        .collect();
                    let n: f64 = num_str
                        .parse()
                        .map_err(|_| LexerError::InvalidNumberFormat)?;
                    tokens.push(Token::Number(n));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let word = iter::once(ch)
                        .chain(from_fn(|| {
                            iter.by_ref()
                                .next_if(|s| s.is_ascii_alphanumeric() || *s == '_')
                        }))
                        .collect::<String>();
                    tokens.push(match word.to_lowercase().as_str() {
                        "select" => Token::Keyword(Select),
                        "and" => Token::Operator(And),
                        "or" => Token::Operator(Or),
                        "not" => Token::Operator(Not),
                        _ => Token::Literal(word),
                    });
                }
                '#' => {
                    tokens.push(Token::Comment(
                        iter::once(ch)
                            .chain(from_fn(|| iter.by_ref().next_if(|s| *s != '\n')))
                            .collect(),
                    ));
                }
                '+' | '-' | '*' | '/' | ':' | '=' | ',' | '(' | ')' => {
                    tokens.push(Token::Operator(
                        Op::from_char(ch).ok_or(LexerError::UnrecognizedToken)?,
                    ));
                }
                '!' | '<' | '>' => {
                    let mut operator = String::from(ch);
                    if let Some(&next_ch) = iter.peek()
                        && (next_ch == '=' || (ch == '<' && next_ch == '>'))
                    {
                        operator.push(iter.next().unwrap());
                    }
                    tokens.push(Token::Operator(
                        Op::from_str(&operator).ok_or(LexerError::UnrecognizedToken)?,
                    ));
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
