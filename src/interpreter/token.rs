#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // identifiers
    IDENT(String),
    INT(i64),

    // operators
    ASSIGN,
    PLUS,
    MINUS,
    ASTERISK,
    FSLASH,
    BANG,
    LT,
    GT,
    EQ,
    NEQ,

    // delims
    COMMA,
    SEMICOLON,

    // braces of all sorts
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LSQUARE,
    RSQUARE,

    // other keywords
    FUNC,
    LET,
    IF,
    ELSE,
    RETURN,
    TRUE,
    FALSE
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub t: TokenType,
    pub l: String
}
