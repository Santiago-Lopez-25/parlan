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
    Id (String),
    NewVector {
        vectype: TokenType
    },
    FreeVector {
        vector: Tk
    }
}

#[derive(Debug)]
pub struct Program {
    pub nodes: Vec<Node>
}

impl Program {
    fn aux_size(&self,node:&Node)->usize {
        match node {
            Node::Call { name, args } => {
                let mut aux = 0;
                for nod in args {
                    aux += self.aux_size(nod);
                }
                return self.aux_size(name) * aux
            }
            Node::BinOp { left, op, right } => {
                return self.aux_size(left) + self.aux_size(right) + size_of_val(op)
            }
            Node::Block { nodes } => {
                let mut aux = 0;
                for nod in nodes {
                    aux += self.aux_size(nod);
                }
                return aux;
            }
            Node::Func { name: _ , parameters, rettype: _, block } => {
                let mut aux = 0;
                for _ in parameters {
                    aux += size_of::<String>() + size_of::<TokenType>();
                }
                return size_of::<String>() + aux + size_of::<TokenType>() + self.aux_size(block)
            }
            Node::Id(_) => size_of::<String>(),
            Node::If { cond, block, else_block } => {
                return self.aux_size(cond) + self.aux_size(block) + size_of_val(else_block)
            }
            Node::Lit(l) => size_of_val(l),
            Node::Return { expr } => self.aux_size(expr),
            Node::Unary { op, value } => size_of_val(op) + self.aux_size(value),
            Node::VarDecl { vartype, name, value } => size_of_val(vartype) + size_of_val(name) + self.aux_size(value),
            Node::VarReassing { name, value } => self.aux_size(value) + size_of_val(name),
            Node::NewVector { vectype:_ } => return size_of::<TokenType>(),
            Node::FreeVector { vector:_ } => return size_of::<Tk>()
        }
    }
    pub fn size(&self)->usize {
        let mut aux = 0;
        for node in &self.nodes {
            aux += self.aux_size(node);
        }
        return aux
    }
}