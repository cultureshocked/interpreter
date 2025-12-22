use crate::token::{Token, TokenType};
use std::collections::LinkedList;

pub trait ASTNode {
    fn token_literal(&self) -> String;
}

pub trait ASTStatement {
    fn statement_node();
}

pub trait ASTExpression {
    fn expression_node();
}

pub struct Program {
    s: Vec<Box<dyn ASTNode>>
}

impl ASTNode for Program {
    fn token_literal(&self) -> String {
        if self.s.len() > 0 {
            return self.s
                .get(0)
                .unwrap()
                .token_literal();
        }

        String::from("")
    }
}
