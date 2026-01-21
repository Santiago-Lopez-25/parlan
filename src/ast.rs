#![allow(dead_code,unused_imports)]

use crate::lexer::{Tk, TokenType};

#[derive(Debug,Clone,PartialEq)]
pub enum Literal {
    Int (usize),
    Float (f64),
    Str (String),
    Bool (bool)
}

#[derive(Debug,Clone,PartialEq)]
pub enum Node {
    Lit (Literal),
    BinOp {
        left: Box<Node>,
        op: u8,
        right: Box<Node>
    },
    Unary {
        op: u8,
        value: Box<Node>
    },
    VarDecl {
        vartype: TokenType,
        name: String,
        value: Box<Node>
    },
    VarReassing {
        name: String,
        value: Box<Node>
    },
    If {
        cond: Box<Node>,
        block: Box<Node>,
        else_block: Option<Box<Node>>
    },
    Func {
        name: String,
        parameters: Vec<(String,TokenType)>,
        rettype: TokenType,
        block: Box<Node>
    },
    Call {
        name: Box<Node>,
        args: Vec<Node>
    },
    Return {
        expr: Box<Node>
    },
    Block {
        nodes: Vec<Node>
    },
    Id (String)
}

#[derive(Debug)]
pub struct Program {
    pub nodes: Vec<Node>
}

impl Program {
    pub fn size(&self)->usize {
        return size_of::<Node>() * self.nodes.len()
    }
}