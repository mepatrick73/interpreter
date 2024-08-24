use crate::macros::*;

enum TokenType {
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
}

#[derive(new)]
struct Token {
    token_type: TokenType,
    literal: String,
}
