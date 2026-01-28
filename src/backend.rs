#![allow(dead_code,unused_imports,unused_variables)]

use std::{clone, fmt::format};

use crate::{ast::*, lexer::TokenType};

pub struct Backend {
    pub c: String, // C generated code
    buff: String, // buffer that will be write into `c` field
    padding: usize // padding
}

impl Backend {
    pub fn new()->Self {
        return Backend {
            c: format!(
r#"#include <stdlib.h>
#include <stdio.h>

"#),
            buff: String::new(),
            padding: 0
        }
    }
    fn push_buff(&mut self) {
        self.c.push_str(" ".repeat(self.padding * 4).as_str());
        self.c.push_str(self.buff.as_str());
        self.c.push('\n');
        self.buff.clear();
    }
    fn emit_expr(&mut self, node: &Node)->String {
        match node {
            Node::Lit(lit) => {
                match lit {
                    Literal::Bool(b) => format!("{}", if *b {"1"} else {"0"}),
                    Literal::Float(f) => format!("{f}"),
                    Literal::Int(i) => format!("{i}"),
                    Literal::Str(s) => format!("{s}")
                }
            }
            Node::BinOp {left, op, right} => {
                let lhs = self.emit_expr(left);
                let rhs = self.emit_expr(right);
                match op {
                    0 => {
                        return format!("{lhs} + {rhs}")
                    }
                    1 => {
                        return format!("{lhs} - {rhs}")
                    }
                    2 => {
                        return format!("{lhs} * {rhs}")
                    }
                    3 => {
                        return format!("{lhs} / {rhs}")
                    }
                    4 => {
                        return format!("{lhs} < {rhs}")
                    }
                    5 => {
                        return format!("{lhs} > {rhs}")
                    }
                    6 => {
                        return format!("{lhs} <= {rhs}")
                    }
                    7 => {
                        return format!("{lhs} >= {rhs}")
                    }
                    8 => {
                        return format!("{lhs} == {rhs}")
                    }
                    9 => {
                        return format!("{lhs} != {rhs}")
                    }
                    other => panic!("error: unknown operator with value: {}",other)
                }
            }
            Node::Unary { op, value } => {
                let expr = self.emit_expr(value);
                if *op == 0 {
                    return format!("-{expr}")
                } else {
                    return format!("!{expr}")
                }
            }
            Node::Call { name, args } => {
                let mut args0 = Vec::new();
                for node in args {
                    args0.push(self.emit_expr(node));
                }
                return format!("{}({})",self.emit_expr(name),args0.join(",")); 

            }
            Node::Id(id) => id.clone(),
            _ => panic!("error: stat Node ({node:?}) passed to Backend::emit_expr")
        }
    }
    fn emit_stat(&mut self, node: &Node) {
        let tktype2ctype = |tktype: &TokenType| {
            match tktype {
                TokenType::FloatT => "double",
                TokenType::BoolT => "unsigned char",
                TokenType::IntT => "int",
                TokenType::StringT => "char*",
                _ => panic!("error: unkown tokentype for type")
            }
        };
        match node {
            Node::VarDecl { vartype, name, value } => {
                let expr = self.emit_expr(value);
                let ctype = tktype2ctype(vartype);
                self.buff.push_str(format!("{ctype} {name} = {expr};").as_str());
                self.push_buff();
            }
            Node::Block { nodes } => {
                for node in nodes {
                    self.emit_stat(node);
                }
            }
            Node::Return { expr } => {
                let expr = self.emit_expr(expr);
                self.buff.push_str(format!("return {expr};").as_str()); 
                self.push_buff();
            }
            Node::Func { name, parameters, rettype, block } => {
                let crtype = tktype2ctype(rettype);
                let mut params = Vec::new();
                for param_pair in parameters {
                    params.push(format!("{} {}", tktype2ctype(&param_pair.1), param_pair.0));
                }
                let params = format!("({})",params.join(","));
                self.buff.push_str(format!("{crtype} {name}{params} {{").as_str()); self.push_buff();
                self.padding += 1;
                self.emit_stat(block);
                self.padding -= 1;
                self.buff.push_str("}"); self.push_buff();
            }
            Node::If { cond, block, else_block } => {
                let cond = self.emit_expr(cond);
                self.buff.push_str(format!("if ({cond}) {{").as_str()); self.push_buff();
                self.padding += 1;
                self.emit_stat(block);
                self.padding -= 1;
                self.buff.push('}');
                if let Some(else_block) = else_block {
                    self.buff.push_str(" else {"); self.push_buff();
                    self.padding += 1;
                    self.emit_stat(else_block);
                    self.padding -= 1;
                    self.buff.push_str("}"); self.push_buff();
                } else {
                    self.push_buff();
                }
            }
            Node::VarReassing { name, value } => {
                let value = self.emit_expr(value);
                self.buff.push_str(format!("{name} = {value};").as_str()); self.push_buff();
            }
            _ => panic!("error: expr Node ({node:?}) passed to Backend::emit_stat")
        }
    }
    pub fn emit_c(&mut self, prog: &Program) {
        for node in &prog.nodes {
            self.emit_stat(node);
        }
    }
} 