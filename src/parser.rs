#![allow(dead_code)]

use core::panic;

use crate::{ast::*, lexer::{Tk, TokenType}};

#[derive(Debug,Clone,PartialEq)]
pub struct Parser {
    idx: usize,
    tokens: Vec<Tk>
}

impl Parser {
    pub fn new(tokens: Vec<Tk>)->Self {
        return Parser {
            tokens,
            idx: 0
        }
    }
    fn peek(&mut self)->&Tk {
        if self.idx < self.tokens.len() {
            return &self.tokens[self.idx]
        } else {
            return &self.tokens[self.tokens.len()-1]
        }
    }
    fn eat(&mut self, tk_type: TokenType)->&Tk {
        if self.peek().tk_type == tk_type {
            self.idx += 1;
            return &self.tokens[self.idx-1];
        } else {
            panic!("error: expected token of type {tk_type:?}, found type {:?}",self.peek().tk_type)
        }
    }

    fn parse_decl(&mut self)->Node {
        match self.peek().tk_type {
            TokenType::Var => self.parse_var_decl(),
            TokenType::If => self.parse_if(),
            TokenType::Func => self.parse_func_decl(),
            TokenType::Return => self.parse_return(),
            _ => {
                return self.parse_expr();
            }
        }
    }
    fn parse_var_decl(&mut self)->Node {
        self.eat(TokenType::Var);
        let name = self.eat(TokenType::Id).span.clone();
        self.eat(TokenType::Colon);
        let vartype = self.eat(self.tokens[self.idx].tk_type.clone()).tk_type.clone();
        self.eat(TokenType::Assing);
        let expr = self.parse_expr();
        return Node::VarDecl { vartype, name, value: Box::new(expr) }    
    }
    fn parse_if(&mut self)->Node {
        self.eat(TokenType::If);
        let cond = self.parse_expr();
        let block = self.parse_block();
        if self.peek().tk_type == TokenType::Else {
            self.idx += 1;
            let else_block = self.parse_block();
            return Node::If { cond: Box::new(cond), block: Box::new(block), else_block: Some(Box::new(else_block)) }
        }
        return Node::If { cond: Box::new(cond), block: Box::new(block), else_block: None }
    }
    fn parse_func_decl(&mut self)->Node {
        self.eat(TokenType::Func);
        let name = self.eat(TokenType::Id).span.clone();
        self.eat(TokenType::Lparen);
        let mut parameters= Vec::new();
        if self.peek().tk_type != TokenType::Rparen {
            let id = self.eat(TokenType::Id).span.clone();
            self.eat(TokenType::Colon);
            let paramtype = self.eat(self.tokens[self.idx].tk_type.clone()).tk_type.clone();
            parameters.push((id,paramtype));
            while self.peek().tk_type == TokenType::Comma {
                self.eat(TokenType::Comma);
                let id = self.eat(TokenType::Id).span.clone();
                self.eat(TokenType::Colon);
                let paramtype = self.eat(self.tokens[self.idx].tk_type.clone()).tk_type.clone();
                parameters.push((id,paramtype));
            }
        }
        self.eat(TokenType::Rparen);
        self.eat(TokenType::Colon);
        let rettype = self.eat(self.tokens[self.idx].tk_type.clone()).tk_type.clone();
        let block = self.parse_block();
        return Node::Func { name, parameters, rettype, block: Box::new(block) }
    }
    fn parse_return(&mut self)->Node {
        self.eat(TokenType::Return);
        let expr = self.parse_expr();
        return Node::Return { expr: Box::new(expr) }
    }
    fn parse_block(&mut self)->Node {
        self.eat(TokenType::Lbrace);
        let mut nodes = Vec::new();
        while self.peek().tk_type != TokenType::Rbrace && self.peek().tk_type != TokenType::Eof {
            nodes.push(self.parse_decl());
        }
        self.eat(TokenType::Rbrace);
        return Node::Block { nodes }
    }

    fn parse_expr(&mut self)->Node {
        return self.parse_eq()
    }
    fn parse_eq(&mut self)->Node {
        let mut left = self.parse_comparation();
        while  [TokenType::Eq,TokenType::Ne].contains(&self.peek().tk_type) {
            let op = self.peek().tk_type.clone();
            self.idx += 1;
            let right = self.parse_comparation();
            left = Node::BinOp { left: Box::new(left), op: (|opr:TokenType| {
                match opr {
                    TokenType::Eq => 9,
                    TokenType::Ne => 10,
                    _ => panic!("error: some inexplicable error ocurred")
                }
            })(op), right: Box::new(right) }
        }
        return left
    }
    fn parse_comparation(&mut self)->Node {
        let mut left = self.parse_term();
        while  [TokenType::Gt,
                TokenType::Ge,
                TokenType::Lt,
                TokenType::Le].
                contains(&self.peek().tk_type)
        {
            let op = self.peek().tk_type.clone();
            self.idx += 1;
            let right = self.parse_term();
            left = Node::BinOp { left: Box::new(left), op: (|opr:TokenType| {
                match opr {
                    TokenType::Lt => 5,
                    TokenType::Gt => 6,
                    TokenType::Le => 7,
                    TokenType::Ge => 8,
                    _ => panic!("error: some inexplicable error ocurred")
                }
            })(op), right: Box::new(right) }
        }
        return left
    }
    fn parse_term(&mut self)->Node {
        let mut left = self.parse_factor();
        while [TokenType::Plus,TokenType::Minus].contains(&self.peek().tk_type) {
            let op = self.peek().tk_type.clone();
            self.idx += 1;
            left = Node::BinOp { left: Box::new(left), op: (|opr:TokenType| {
                match opr {
                    TokenType::Plus => 0,
                    TokenType::Minus => 2,
                    _ => panic!("error: some inexplicable error ocurred")
                }
            })(op), right: Box::new(self.parse_factor()) }
        }
        return left;
    }
    fn parse_factor(&mut self)->Node {
        let mut left = self.parse_unary();
        while [TokenType::Star,TokenType::Slash].contains(&self.peek().tk_type) {
            let op = self.peek().tk_type.clone();
            self.idx += 1;
            left = Node::BinOp { left: Box::new(left), op: (|opr:TokenType| {
                match opr {
                    TokenType::Star => 2,
                    TokenType::Slash => 3,
                    _ => panic!("error: some inexplicable error ocurred")
                }
            })(op), right: Box::new(self.parse_unary()) }
        }
        return left;
    }
    fn parse_unary(&mut self)->Node {
        if self.peek().tk_type == TokenType::Minus {
            let op = self.peek().tk_type.clone();
            self.idx += 1;
            return Node::Unary { op: if op == TokenType::Bang{1} else {0}, value: Box::new(self.parse_unary()) }
        } else {
            return self.parse_call()
        }
    }
    fn parse_call(&mut self)->Node {
        let mut left = self.parse_primary();
        
        loop {
            if self.peek().tk_type == TokenType::Lparen {
                self.idx += 1;
                let mut args = Vec::new();
                if self.peek().tk_type != TokenType::Rparen {
                    let expr = self.parse_expr();
                    args.push(expr);
                    while self.peek().tk_type == TokenType::Comma {
                        self.eat(TokenType::Comma);
                        let expr = self.parse_expr();
                        args.push(expr);
                    }
                }
                self.eat(TokenType::Rparen);
                left = Node::Call { name: Box::new(left), args }
            } else {
                break;
            }
        }
        return left;
    }
    fn parse_primary(&mut self)->Node {
        match self.peek().tk_type.clone() {
            TokenType::IntL(n) => {
                self.idx += 1;
                return Node::Lit(Literal::Int(n))
            }
            TokenType::FloatL(n) => {
                self.idx += 1;
                return Node::Lit(Literal::Float(n));
            }
            TokenType::BoolL(b) => {
                self.idx += 1;
                return Node::Lit(Literal::Bool(b))
            }
            TokenType::StringL(s) => {
                self.idx += 1;
                return Node::Lit(Literal::Str(s))
            }
            TokenType::Id => {
                let id = self.peek().span.clone();
                self.idx += 1;
                match self.peek().tk_type {
                    TokenType::Assing => {
                        self.idx += 1;
                        let expr = self.parse_expr();
                        return Node::VarReassing { name: id, value: Box::new(expr) }
                    }
                    _ => {
                        return Node::Id(id)
                    }
                }
            }
            TokenType::Lparen => {
                self.idx += 1;
                let expr = self.parse_expr();
                self.eat(TokenType::Rparen);
                return expr;
            }
            _ => {
                panic!("error: unexpected primary token type `{:?}`",self.peek().tk_type)
            }
        }
    }
    pub fn parse_program(&mut self)->Program {
        let mut nodes = Vec::new();
        while self.idx < self.tokens.len() {
            if self.peek().tk_type == TokenType::Eof {break;}
            nodes.push(self.parse_decl());
        }
        return Program { nodes }
    }
}