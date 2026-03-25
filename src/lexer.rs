use crate::token::{Token, TokenType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexer {
    pub input: String,
    pub position: usize,
    pub read_position: usize,
    pub ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        }
    }

    pub fn next_token(&self) -> Token {
        Token::new(TokenType::Unknown, "unknown")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenType;
    #[test]
    fn next_token() {
        let input = String::from("=+(){},;");
        let lexer = Lexer::new(input);

        let cases = vec![
            (
                TokenType::Assign,
                "=",
            ),
            (
                TokenType::Plus,
                "+",
            ),
            (
                TokenType::Lparen,
                "(",
            ),
            (
                TokenType::Rparen,
                ")",
            ),
            (
                TokenType::Lbrace,
                "{",
            ),
            (
                TokenType::Rbrace,
                "}",
            ),
            (
                TokenType::Comma,
                ",",
            ),
            (
                TokenType::Semicolon,
                ";",
            ),
            (
                TokenType::Eof,
                "",
            ),
        ];

        for (kind, literal) in cases {
            let token = lexer.next_token();

            assert_eq!(token.kind, kind, "base");
            assert_eq!(token.literal, literal, "base");
        }
    }
}