#![allow(dead_code,unused)]

use crate::{ast::*, lexer::TokenType};
use std::collections::HashMap;

#[derive(Debug,Clone)]
pub enum Instr {
    Move {
        dest: u32,
        source: Box<Instr>
    },
    Func {
        id: u32,
        params: Vec<u32>,
        rettype: TokenType
    },
    Call {
        dest: u32,
        id: u32,
        args: Vec<Instr>
    },
    Ret {
        expr: Box<Instr>
    },
    End,
    Add {
        dest: u32,
        s0: Box<Instr>,
        s1: Box<Instr>
    },
    Sub {
        dest: u32,
        s0: Box<Instr>,
        s1: Box<Instr>
    },
    Mul {
        dest: u32,
        s0: Box<Instr>,
        s1: Box<Instr>
    },
    Div {
        dest: u32,
        s0: Box<Instr>,
        s1: Box<Instr>
    },
    JmpTrue {
        source: Box<Instr>,
        label: Box<Instr>
    },
    Jmp {
        label: Box<Instr>  
    },
    Label {
        id: u32
    },
    Lt {
        dest: u32,
        s0: Box<Instr>,
        s1: Box<Instr>
    },
    Gt {
        dest: u32,
        s0: Box<Instr>,
        s1: Box<Instr>
    },
    Le {
        dest: u32,
        s0: Box<Instr>,
        s1: Box<Instr>
    },
    Ge {
        dest: u32,
        s0: Box<Instr>,
        s1: Box<Instr>
    },
    Eq {
        dest: u32,
        s0: Box<Instr>,
        s1: Box<Instr>
    },
    Ne {
        dest: u32,
        s0: Box<Instr>,
        s1: Box<Instr>
    },
    Reg (u32),
    IntL (usize),
    FloatL (f64),
    BoolL (bool),
    StrL (String)
}


pub struct IrEmiter {
    curr_reg: u32,
    curr_label: u32,
    curr_function: u32,
    vars: Vec<HashMap<String,u32>>,
    funcs: HashMap<String,u32>,
    pub ir: Vec<Instr>
}

fn node_to_string(node: &Node)->String {
    match node {
        Node::Id(str) => return str.clone(),
        _ => {
            panic!("error: unexpected Node type to convert to String")
        }
    }
}

impl IrEmiter {
    pub fn new()->Self {
        return Self {
            curr_reg: 0,
            curr_label: 0,
            curr_function: 0,
            vars: vec![HashMap::new()],
            funcs: HashMap::new(),
            ir: Vec::new()
        }
    }
    fn unique_label(&mut self)->u32 {
        self.curr_label += 1;
        return self.curr_label - 1
    }
    fn next_reg(&mut self)->u32 {
        self.curr_reg += 1;
        return self.curr_reg - 1;
    }

    fn emit_expr(&mut self, node: &Node)->Instr {
        match node {
            Node::Lit(lit) => {
                match lit {
                    Literal::Int(n) => {
                        return Instr::IntL(*n)
                    }
                    Literal::Float(n) => {
                        return Instr::FloatL(*n)
                    }
                    Literal::Bool(b) => {
                        return Instr::BoolL(*b)
                    }
                    Literal::Str(s) => {
                        return Instr::StrL(s.clone())
                    }
                }
            }
            Node::BinOp {left, op, right} => {
                let lhs = self.emit_expr(left);
                let rhs = self.emit_expr(right);
                match op {
                    0 => {
                        let dest = self.next_reg();
                        self.ir.push(Instr::Add {
                            dest,
                            s0: Box::new(lhs),
                            s1: Box::new(rhs)
                        });
                        return Instr::Reg (dest)
                    }
                    1 => {
                        let dest = self.next_reg();
                        self.ir.push(Instr::Sub {
                            dest,
                            s0: Box::new(lhs),
                            s1: Box::new(rhs)
                        });
                        return Instr::Reg (dest)
                    }
                    2 => {
                        let dest = self.next_reg();
                        self.ir.push(Instr::Mul {
                            dest,
                            s0: Box::new(lhs),
                            s1: Box::new(rhs)
                        });
                        return Instr::Reg (dest)
                    }
                    3 => {
                        let dest = self.next_reg();
                        self.ir.push(Instr::Div {
                            dest,
                            s0: Box::new(lhs),
                            s1: Box::new(rhs)
                        });
                        return Instr::Reg (dest)
                    }
                    4 => {
                        let dest = self.next_reg();
                        self.ir.push(Instr::Lt {
                            dest,
                            s0: Box::new(lhs),
                            s1: Box::new(rhs)
                        });
                        return Instr::Reg (dest)
                    }
                    5 => {
                        let dest = self.next_reg();
                        self.ir.push(Instr::Gt {
                            dest,
                            s0: Box::new(lhs),
                            s1: Box::new(rhs)
                        });
                        return Instr::Reg (dest)
                    }
                    6 => {
                        let dest = self.next_reg();
                        self.ir.push(Instr::Le {
                            dest,
                            s0: Box::new(lhs),
                            s1: Box::new(rhs)
                        });
                        return Instr::Reg (dest)
                    }
                    7 => {
                        let dest = self.next_reg();
                        self.ir.push(Instr::Ge {
                            dest,
                            s0: Box::new(lhs),
                            s1: Box::new(rhs)
                        });
                        return Instr::Reg (dest)
                    }
                    8 => {
                        let dest = self.next_reg();
                        self.ir.push(Instr::Eq {
                            dest,
                            s0: Box::new(lhs),
                            s1: Box::new(rhs)
                        });
                        return Instr::Reg (dest)
                    }
                    9 => {
                        let dest = self.next_reg();
                        self.ir.push(Instr::Ne {
                            dest,
                            s0: Box::new(lhs),
                            s1: Box::new(rhs)
                        });
                        return Instr::Reg (dest)
                    }
                    other => panic!("error: unknown operator with value: {}",other)
                }
            }
            Node::Call {name, args} => {
                let mut arguments = Vec::new();
                for arg in args {
                    arguments.push(self.emit_expr(arg));
                }       
                let dest = self.next_reg();
                self.ir.push(Instr::Call {
                    dest,
                    id: *self.funcs.get(&node_to_string(name.as_ref())).unwrap(),
                    args: arguments
                });
                return Instr::Reg (dest);
            }
            Node::Id (id) => {
                return Instr::Reg (*self.vars.last().unwrap().get(id).unwrap())
            }
            _ => {
                panic!("error: a not expretion Node passed to `emit_expr`")
            }    
        }
    }
    fn emit_stat(&mut self, node: &Node) {
        match node {
            Node::VarDecl {vartype: _, name, value} => {
                let reg = self.emit_expr(value);
                let new_var = self.next_reg();
                self.vars.last_mut().unwrap().insert(name.clone(),new_var);
                self.ir.push(Instr::Move {
                    dest: new_var,
                    source: Box::new(reg)
                })
            }
            Node::Block { nodes } => {
                for node in nodes {
                    self.emit_stat(node);
                }
            }
            Node::Func { name, parameters, rettype, block } => {
                let id = self.curr_function;
                self.funcs.insert(name.clone(), self.curr_function);
                self.curr_function += 1;
                self.vars.push(HashMap::new());
                let mut params = Vec::new();
                for parameter in parameters {
                    self.vars.last_mut().unwrap().insert(parameter.0.clone(), self.curr_reg);
                    params.push(self.curr_reg);
                    self.curr_reg += 1;
                }
                self.ir.push(Instr::Func { id, params, rettype: rettype.clone()});
                self.emit_stat(block.as_ref());
                self.ir.push(Instr::End);
                self.vars.pop();
            }
            Node::If { cond, block, else_block } => {
                let expr = self.emit_expr(cond.as_ref());
                if else_block.is_some() {
                    let then_l = self.curr_label;
                    self.ir.push(Instr::JmpTrue { source: Box::new(expr), label: Box::new(Instr::Label { id: self.curr_label }) });
                    self.curr_label += 1;
                    let else_l = self.curr_label;
                    self.ir.push(Instr::Jmp { label: Box::new(Instr::Label { id: self.curr_label }) });
                    self.curr_label += 1;
                    self.ir.push(Instr::Label { id: then_l });
                    self.emit_stat(block);
                    self.ir.push(Instr::Label { id: else_l });
                    self.emit_stat(else_block.as_ref().expect(""));
                } else {
                    let then_l = self.curr_label;
                    self.ir.push(Instr::JmpTrue { source: Box::new(expr), label: Box::new(Instr::Label { id: self.curr_label }) });
                    self.curr_label += 1;
                    let else_l = self.curr_label;
                    self.ir.push(Instr::Jmp { label: Box::new(Instr::Label { id: self.curr_label }) });
                    self.curr_label += 1;
                    self.ir.push(Instr::Label { id: then_l });
                    self.emit_stat(block);
                    self.ir.push(Instr::Label { id: else_l });
                }
            }
            Node::Return { expr } => {
                let expr = self.emit_expr(expr);
                self.ir.push(Instr::Ret { expr: Box::new(expr) });
            }
            Node::VarReassing { name, value } => {
                let expr = self.emit_expr(value);
                self.ir.push(Instr::Move { dest: *self.vars.last().unwrap().get(name).unwrap(), source: Box::new(expr) });
            }
            _ => unimplemented!()
        }
    }
    pub fn emit_ir<'a>(&mut self, ast: &'a Program) {
        for stat in &ast.nodes {
            self.emit_stat(&stat);
        }
    }
}
