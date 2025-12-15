use crate::token::{Token, TokenType};
use std::collections::HashMap;

const keywords: [(&str, TokenType); 7] = [
    ("lambda", TokenType::FUNC),
    ("let", TokenType::LET),
    ("true", TokenType::TRUE),
    ("false", TokenType::FALSE),
    ("return", TokenType::RETURN),
    ("if", TokenType::IF),
    ("else", TokenType::ELSE)
];

fn lookup_keyword(word: &str) -> Option<TokenType> {
    for n in keywords {
        if n.0 == word {
            return Some(n.1);
        } 
    }

    None
}

#[derive(Debug)]
pub struct Lexer {
    pub input: String,
    pub position: i64,
    pub read_position: i64,
    pub current_char: char
}

impl Lexer {
    pub fn read_char(&mut self) {
        // println!("LEXER STATE:");
        // println!("{:?}", self);
        if self.read_position as usize >= self.input.len() {
            self.current_char = '\u{0}';
        } else {
            self.current_char = self.input.chars().nth(self.read_position as usize).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
        // println!("{:?}", self);
    }

    pub fn back_char(&mut self) {
        self.position -= 1;
        self.read_position -= 1;
        self.current_char = self.input.chars().nth(self.read_position as usize).unwrap();
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok: Token = match self.current_char {
            '=' => match self.peek_ahead() {
                '=' => {
                    self.read_char();
                    Token { t: TokenType::EQ, l: "==".to_string() }
                },
                _ => Token { t: TokenType::ASSIGN, l: "=".to_string() }
            },
            ';' => Token { t: TokenType::SEMICOLON, l: ";".to_string() },
            '(' => Token { t: TokenType::LPAREN, l: "(".to_string() },
            ')' => Token { t: TokenType::RPAREN, l: ")".to_string() },
            '{' => Token { t: TokenType::LBRACE, l: "{".to_string() },
            '}' => Token { t: TokenType::RBRACE, l: "}".to_string() },
            ',' => Token { t: TokenType::COMMA, l: ",".to_string() },
            '+' => Token { t: TokenType::PLUS, l: "+".to_string() },
            '!' => match self.peek_ahead() {
                '=' => {
                    self.read_char();
                    Token { t: TokenType::NEQ, l: "!=".to_string() }
                },
                _ => Token { t: TokenType::BANG, l: "!".to_string() }
            },
            '<' => Token { t: TokenType::LT, l: "<".to_string() },
            '>' => Token { t: TokenType::GT, l: ">".to_string() },
            '*' => Token { t: TokenType::ASTERISK, l: "*".to_string() },
            '/' => Token { t: TokenType::FSLASH, l: "/".to_string() },
            '-' => Token { t: TokenType::MINUS, l: "-".to_string() },
            '\u{0}' => Token { t: TokenType::EOF, l: "".to_string() },
            _ => if self.is_letter() {
                self.parse_string()
            } else if self.is_digit() {
                self.parse_number()
            } else {
                Token { t: TokenType::ILLEGAL, l: "something horrible".to_string() }
            }
        };

        self.read_char();
        // println!("Got token: {:?}", tok);
        tok
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position as usize;
        while self.is_letter() {
            self.read_char();
        }

        self.back_char();
        self.input.get(pos..=self.position as usize).unwrap().to_string()
    }

    fn parse_string(&mut self) -> Token {
        println!("Calling parse_string");
        let word = self.read_identifier().to_string();
        println!("parsed: {word}");
        Token {
            t: match lookup_keyword(&word) {
                Some(x) => x,
                None => TokenType::IDENT(String::from(&word))
            },
            l: String::from(&word)
        }
    }

    fn parse_number(&mut self) -> Token {
        println!("Calling parse_number");
        let pos = self.position as usize;
        while self.is_digit() {
            self.read_char();
        }

        println!("positions: ({}, {})", pos, self.position);

        self.back_char();

        let n_as_s: String = self.input.get(pos..=self.position as usize).unwrap().to_string();

        let n: i64 = match n_as_s.parse() {
            Ok(res) => res,
            Err(e) => panic!("{}", e)
        };

        println!("parse_number Parsed: {n}");
        println!("current char post_number: {}", self.current_char);

        Token { 
            t: TokenType::INT(n), 
            l: n_as_s
        }
    }

    fn is_letter(&self) -> bool {
        self.current_char.is_alphabetic() || self.current_char == '_'
    }

    fn is_digit(&self) -> bool {
        self.current_char.is_ascii_digit()
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_whitespace() {
            self.read_char()
        }
    }

    fn peek_ahead(&self) -> char {
        if self.read_position as usize >= self.input.len() {
            return '\u{0}';
        }

        self.input.chars().nth(self.read_position as usize).unwrap()
    }
}

pub fn new(i: String) -> Lexer {
    Lexer {
        input: i.clone(),
        position: 0,
        read_position: 1,
        current_char: i.chars().nth(0).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;".to_string();

        let expected: Vec<Token> = vec![
            Token { t: TokenType::ASSIGN, l: "=".to_string() },
            Token { t: TokenType::PLUS, l: "+".to_string() },
            Token { t: TokenType::LPAREN, l: "(".to_string() },
            Token { t: TokenType::RPAREN, l: ")".to_string() },
            Token { t: TokenType::LBRACE, l: "{".to_string() },
            Token { t: TokenType::RBRACE, l: "}".to_string() },
            Token { t: TokenType::COMMA, l: ",".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },
            Token { t: TokenType::EOF, l: "".to_string() }
        ];

        let mut lex = new(input);

        for t in expected {
            let curr_tok = lex.next_token();
            assert_eq!(t.l, curr_tok.l);
            assert_eq!(t.t, curr_tok.t);
        }
    }

    #[test]
    fn test_identifiers() {
        let input = "
let five = 5;
let ten = 10;
let add = lambda(x, y) {
    x + y;
};

let result = add(five, ten);".to_string();

        let expected: Vec<Token> = vec![
            Token { t: TokenType::LET, l: "let".to_string() },
            Token { t: TokenType::IDENT(String::from("five")), l: "five".to_string() },
            Token { t: TokenType::ASSIGN, l: "=".to_string() },
            Token { t: TokenType::INT(5), l: "5".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },

            Token { t: TokenType::LET, l: "let".to_string() },
            Token { t: TokenType::IDENT(String::from("ten")), l: "ten".to_string() },
            Token { t: TokenType::ASSIGN, l: "=".to_string() },
            Token { t: TokenType::INT(10), l: "10".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },

            Token { t: TokenType::LET, l: "let".to_string() },
            Token { t: TokenType::IDENT(String::from("add")), l: "add".to_string() },
            Token { t: TokenType::ASSIGN, l: "=".to_string() },
            Token { t: TokenType::FUNC, l: "lambda".to_string() },
            Token { t: TokenType::LPAREN, l: "(".to_string() },
            Token { t: TokenType::IDENT(String::from("x")), l: "x".to_string() },
            Token { t: TokenType::COMMA, l: ",".to_string() },
            Token { t: TokenType::IDENT(String::from("y")), l: "y".to_string() },
            Token { t: TokenType::RPAREN, l: ")".to_string() },
            Token { t: TokenType::LBRACE, l: "{".to_string() },

            Token { t: TokenType::IDENT(String::from("x")), l: "x".to_string() },
            Token { t: TokenType::PLUS, l: "+".to_string() },
            Token { t: TokenType::IDENT(String::from("y")), l: "y".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },

            Token { t: TokenType::RBRACE, l: "}".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },

            Token { t: TokenType::LET, l: "let".to_string() },
            Token { t: TokenType::IDENT(String::from("result")), l: "result".to_string() },
            Token { t: TokenType::ASSIGN, l: "=".to_string() },
            Token { t: TokenType::IDENT(String::from("add")), l: "add".to_string() },
            Token { t: TokenType::LPAREN, l: "(".to_string() },
            Token { t: TokenType::IDENT(String::from("five")), l: "five".to_string() },
            Token { t: TokenType::COMMA, l: ",".to_string() },
            Token { t: TokenType::IDENT(String::from("ten")), l: "ten".to_string() },
            Token { t: TokenType::RPAREN, l: ")".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },
            Token { t: TokenType::EOF, l: "".to_string() }
        ];

        let mut lex = new(input);
        
        for t in expected {
            let curr_tok = lex.next_token();
            assert_eq!(t.l, curr_tok.l);
            assert_eq!(t.t, curr_tok.t);
        }
    }

    #[test]
    fn test_all_symbols() {
        let input = "let five = 5;
let ten = 10;

let add = lambda(x, y) {
  x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
	return true;
} else {
	return false;
}

10 == 10;
10 != 9;".to_string();
        
        let mut lex = new(input);
        let expected = vec![
            Token { t: TokenType::LET, l: "let".to_string() },
            Token { t: TokenType::IDENT(String::from("five")), l: "five".to_string() },
            Token { t: TokenType::ASSIGN, l: "=".to_string() },
            Token { t: TokenType::INT(5), l: "5".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },

            Token { t: TokenType::LET, l: "let".to_string() },
            Token { t: TokenType::IDENT(String::from("ten")), l: "ten".to_string() },
            Token { t: TokenType::ASSIGN, l: "=".to_string() },
            Token { t: TokenType::INT(10), l: "10".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },

            Token { t: TokenType::LET, l: "let".to_string() },
            Token { t: TokenType::IDENT(String::from("add")), l: "add".to_string() },
            Token { t: TokenType::ASSIGN, l: "=".to_string() },
            Token { t: TokenType::FUNC, l: "lambda".to_string() },
            Token { t: TokenType::LPAREN, l: "(".to_string() },
            Token { t: TokenType::IDENT(String::from("x")), l: "x".to_string() },
            Token { t: TokenType::COMMA, l: ",".to_string() },
            Token { t: TokenType::IDENT(String::from("y")), l: "y".to_string() },
            Token { t: TokenType::RPAREN, l: ")".to_string() },
            Token { t: TokenType::LBRACE, l: "{".to_string() },

            Token { t: TokenType::IDENT(String::from("x")), l: "x".to_string() },
            Token { t: TokenType::PLUS, l: "+".to_string() },
            Token { t: TokenType::IDENT(String::from("y")), l: "y".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },

            Token { t: TokenType::RBRACE, l: "}".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },

            Token { t: TokenType::LET, l: "let".to_string() },
            Token { t: TokenType::IDENT(String::from("result")), l: "result".to_string() },
            Token { t: TokenType::ASSIGN, l: "=".to_string() },
            Token { t: TokenType::IDENT(String::from("add")), l: "add".to_string() },
            Token { t: TokenType::LPAREN, l: "(".to_string() },
            Token { t: TokenType::IDENT(String::from("five")), l: "five".to_string() },
            Token { t: TokenType::COMMA, l: ",".to_string() },
            Token { t: TokenType::IDENT(String::from("ten")), l: "ten".to_string() },
            Token { t: TokenType::RPAREN, l: ")".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },

            Token { t: TokenType::BANG, l: "!".to_string() },
            Token { t: TokenType::MINUS, l: "-".to_string() },
            Token { t: TokenType::FSLASH, l: "/".to_string() },
            Token { t: TokenType::ASTERISK, l: "*".to_string() },
            Token { t: TokenType::INT(5), l: "5".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },

            Token { t: TokenType::INT(5), l: "5".to_string() },
            Token { t: TokenType::LT, l: "<".to_string() },
            Token { t: TokenType::INT(10), l: "10".to_string() },
            Token { t: TokenType::GT, l: ">".to_string() },
            Token { t: TokenType::INT(5), l: "5".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },

            Token { t: TokenType::IF, l: "if".to_string() },
            Token { t: TokenType::LPAREN, l: "(".to_string() },
            Token { t: TokenType::INT(5), l: "5".to_string() },
            Token { t: TokenType::LT, l: "<".to_string() },
            Token { t: TokenType::INT(10), l: "10".to_string() },
            Token { t: TokenType::RPAREN, l: ")".to_string() },
            Token { t: TokenType::LBRACE, l: "{".to_string() },

            Token { t: TokenType::RETURN, l: "return".to_string() },
            Token { t: TokenType::TRUE, l: "true".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },
            Token { t: TokenType::RBRACE, l: "}".to_string() },

            Token { t: TokenType::ELSE, l: "else".to_string() },
            Token { t: TokenType::LBRACE, l: "{".to_string() },

            Token { t: TokenType::RETURN, l: "return".to_string() },
            Token { t: TokenType::FALSE, l: "false".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },
            Token { t: TokenType::RBRACE, l: "}".to_string() },

            Token { t: TokenType::INT(10), l: "10".to_string() },
            Token { t: TokenType::EQ, l: "==".to_string() },
            Token { t: TokenType::INT(10), l: "10".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },

            Token { t: TokenType::INT(10), l: "10".to_string() },
            Token { t: TokenType::NEQ, l: "!=".to_string() },
            Token { t: TokenType::INT(9), l: "9".to_string() },
            Token { t: TokenType::SEMICOLON, l: ";".to_string() },
            Token { t: TokenType::EOF, l: "".to_string() }
        ];

        for t in expected {
            let curr_tok = lex.next_token();
            assert_eq!(t.t, curr_tok.t);
            assert_eq!(t.l, curr_tok.l);
        }
    }
}
