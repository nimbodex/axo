#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexer {
    pub input: String,
    position: usize,
    read_position: usize,
    ch: char,
}