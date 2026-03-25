#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Unknown,
    Illegal,
    Eof,
    Ident,
    Int,

    Assign,
    Plus,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(kind: TokenType, literal: impl Into<String>) -> Self {
        Self {
            kind,
            literal: literal.into(),
        }
    }
}

