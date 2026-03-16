#![allow(dead_code,unused_imports,unused_variables)]

use std::{clone, fmt::format};

use crate::{ast::*, lexer::TokenType};

pub struct Backend {
    pub c: String, // C generated code
    buff: String, // buffer that will be write into `c` field
    padding: usize // padding
}

pub static BOILERPLATE: &str = r#"#include <stdlib.h>
#include <stdio.h>
#include <string.h>

typedef unsigned long long u64;
typedef signed long long i64;
typedef signed int i32;

#define vector Vector*
typedef struct {
    u64    len;
    u64    cap;
    u64    elem_size;
    void**  data;
} Vector;

#define vec_len(vec) (vec->len)
#define vec_cap(vec) (vec->cap)

vector new__vector(u64 elem_size) {
    vector vec = (vector)malloc(sizeof(Vector));
    vec->len = 0;
    vec->elem_size = elem_size;
    vec->cap = 15;
    vec->data = malloc(vec->cap * elem_size);
    return vec;
}

void free__vector(vector vec) {
    free(vec->data);
    free(vec);
    vec = NULL;
}

void push__vector(vector vec, void* data) {
    if (vec->len+1 == vec->cap) {
        vec->cap *= 2;
        vec->data = realloc(vec->data, vec->cap * vec->elem_size);
    }
    *(vec->data + (vec->len * vec->elem_size)) = data;
    vec->len++;
}

void* get__vector(vector vec, u64 idx) {
    if (idx >= vec->len) {
        fprintf(stderr, "error: trying to access index %llu while size is %llu", idx, vec->len);
        return NULL;
    }
    return vec->data + (idx * vec->elem_size);
}

"#;

impl Backend {
    pub fn new()->Self {
        return Backend {
            c: BOILERPLATE.to_string(),
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
        let tktype2ctype = |tktype: &TokenType| {
            match tktype {
                TokenType::FloatT => "double",
                TokenType::BoolT => "unsigned char",
                TokenType::IntT => "i64",
                TokenType::StringT => "char*",
                TokenType::VecT => "vector",
                _ => panic!("error: unkown tokentype `{tktype:?}` for type")
            }
        };
        match node {
            Node::Lit(lit) => {
                match lit {
                    Literal::Bool(b) => format!("{}", if *b {"1"} else {"0"}),
                    Literal::Float(f) => format!("{f}"),
                    Literal::Int(i) => format!("{i}"),
                    Literal::Str(s) => format!("\"{s}\"")
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
            Node::NewVector { vectype } => {
                return format!("new__vector(sizeof({}))",tktype2ctype(vectype))
            }
            Node::GetVector { vector, index } => {
                return format!("get__vector(usr_{},{index})",vector.span)
            }
            Node::Id(id) => format!("usr_{id}"),
            _ => panic!("error: stat Node ({node:?}) passed to Backend::emit_expr")
        }
    }
    fn emit_stat(&mut self, node: &Node) {
        let tktype2ctype = |tktype: &TokenType| {
            match tktype {
                TokenType::FloatT => "double",
                TokenType::BoolT => "unsigned char",
                TokenType::IntT => "i64",
                TokenType::StringT => "char*",
                TokenType::VecT => "vector",
                _ => panic!("error: unkown tokentype `{tktype:?}` for type ")
            }
        };
        match node {
            Node::VarDecl { vartype, name, value } => {
                let expr = self.emit_expr(value);
                let ctype = tktype2ctype(vartype);
                self.buff.push_str(format!("{ctype} usr_{name} = {expr};").as_str());
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
                self.buff.push_str(format!("{} {}{params} {{", if name != "main" {crtype} else {"int"}, if name != "main" {format!("usr_{name}")} else {"main".to_string()}).as_str()); self.push_buff();
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
            Node::FreeVector { vector } => {
                self.buff.push_str(format!("free__vector(usr_{});",vector.span).as_str()); self.push_buff();
            }
            Node::PushVector { vector, elem } => {
                let expr = self.emit_expr(elem);
                self.buff.push_str(format!("push__vector(usr_{},(void*){expr});",vector.span).as_str()); self.push_buff();
            }
            Node::Cblock { code } => {
                self.buff.push_str(code.as_str()); self.push_buff();
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