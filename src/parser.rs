/*
The Parser or Syntactic analyzer:

the parser is usually (1) the second part of a compiler, it tooks the token stream
and generates something called an Abstrac Syntax Tree (or for short: AST). the AST
is a structure that represent the source code syntacticaly, and the parser first 
checks if the the source code (well, the token stream that represent the source code) is
syntacticaly correct for the language, that mean the parser of a C compiler is not the same
as the parser of a Rust compiler, because both has diferent syntaxis! 
the second thing that the parser does is construct the AST from the tokens, these two 
processes are usually done at the same time (2).

but for what is the AST? why we need to construct it? well, actually you don't need
an AST to get a functional compiler (1), but is more common to use it, but why?
the AST is usually used to simplify the other parts of the compiler (especialy the 
semantic analisys and backend), because it offers a simple structure that you can
walk and comprobe its structure. the AST is a tree structure conformed by nodes, these
nodes can be root nodes, leaf nodes or normal nodes (well, i name them like this); a root 
node is a node that doesn't have a father node (is a node that is not contained by other), 
leaf nodes are nodes that doesn't have child nodes (is a node that does not contain other, 
like numbers or literals); and there are normal nodes, these have both a father node and child(s) node(s)

another important thing in a parser is the operator precedence. operator precedence just says that
some operations should be done first than others.
in this compiler we use the following operator precedence:

equality ('==','!=') ->
comparation ('>','<','<=','>=') ->
terms ('+', '-') ->
factors ('*','/') ->
unary ('-', 'not') ->
call (function calls) ->
primarys (literals)

this should be read backwards, that mean primarys must be parsed before calls, calls before unary 
operators, unary operators before factors, and so on

almost every function here has the following structure (except parse_primary and parse_unary):

```
function parse_something() {
    left = parse_next_precedence_level()
    if there is the operator X {
        consume_operator(X)
        right = parse_next_precedence_level()
        return Node {left, right}
    }
    return left
}
```
this structure ensures that if the operator was not found, we just go to the next precedence

i will put some examples below, but you need to understand that an AST can be textualy
represented several forms.

---- EXAMPLES ----

with the previous stream of tokens we created (see the docs in lexer.rs file):
```
[
TK_voidtype,
TK_identifier,
TK_left_paren,
TK_right_paren,
TK_left_brace,
TK_right_brace
]
```
We can create the following AST (i hope my example is clear):
```
FuncDeclNode {
    type: TK_voidtype
    name: IdentifierNode { "something" }
    body: BlockNode {
        nodes: [
        
        ]
    }
}
```

---- NOTES ----

[1]. in the book 'crafting interpreters' by Robert Nystrom, Robert creates actually a compiler to bytecode
    instead of a interpreter (the second part, using C. the first part uses Java). and this doesn't have a
    parser or an AST, it just translate the token stream to bytecode

[2]. well, i will be more clear: with "at the same time" i'm not referring to the fact that it uses multithreating or 
    asynchronous programming, i mean the function that analyzes the token stream, also generates the node after the analysis
*/

#![allow(dead_code)]

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
    // ---- auxiliar functions ----

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
        // here we select the function to parse the current structure (may be a statement or an expretion)
        match self.peek().tk_type {
            TokenType::Var => self.parse_var_decl(),
            TokenType::If => self.parse_if(),
            TokenType::Func => self.parse_func_decl(),
            TokenType::Return => self.parse_return(),
            TokenType::While => self.parse_while(),
            _ => {
                return self.parse_expr();
            }
        }
    }
    fn parse_var_decl(&mut self)->Node {
        // here we analyze the structure of a variable declaration
        self.eat(TokenType::Var);
        let name = self.eat(TokenType::Id).span.clone();
        self.eat(TokenType::Colon);
        let vartype = self.eat(self.tokens[self.idx].tk_type.clone()).tk_type.clone();
        self.eat(TokenType::Assing);
        let expr = self.parse_expr();
        // and here we generate the code
        return Node::VarDecl { vartype, name, value: Box::new(expr) }    
    }
    fn parse_if(&mut self)->Node {
        // again the same thing here
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
    fn parse_while(&mut self)->Node {
        // we use the same logic
        self.eat(TokenType::While);
        let condition = Box::new(self.parse_expr());
        let block = Box::new(self.parse_block());
        return Node::While { condition, block }
    }
    fn parse_func_decl(&mut self)->Node {
        // we use  the same logic
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
        // we use the same logic
        self.eat(TokenType::Return);
        let expr = self.parse_expr();
        return Node::Return { expr: Box::new(expr) }
    }
    fn parse_block(&mut self)->Node {
        // and the same logic
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
                    TokenType::Eq => 8,
                    TokenType::Ne => 9,
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
                    TokenType::Lt => 4,
                    TokenType::Gt => 5,
                    TokenType::Le => 6,
                    TokenType::Ge => 7,
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
                    TokenType::Minus => 1,
                    _ => panic!("error: some inexplicable error ocurred")
                }
            })(op), right: Box::new(self.parse_factor()) }
        }
        return left;
    }
    fn parse_factor(&mut self)->Node {
        let mut left = self.parse_or();
        while [TokenType::Star,TokenType::Slash].contains(&self.peek().tk_type) {
            let op = self.peek().tk_type.clone();
            self.idx += 1;
            left = Node::BinOp { left: Box::new(left), op: (|opr:TokenType| {
                match opr {
                    TokenType::Star => 2,
                    TokenType::Slash => 3,
                    _ => panic!("error: some inexplicable error ocurred")
                }
            })(op), right: Box::new(self.parse_or()) }
        }
        return left;
    }
    fn parse_or(&mut self)->Node {
        let mut left = self.parse_and();
        while self.peek().tk_type == TokenType::Or {
            self.idx += 1;
            left = Node::BinOp { left: Box::new(left), op: 11, right: Box::new(self.parse_and()) }
        }
        return left;
    }
    fn parse_and(&mut self)->Node {
        let mut left = self.parse_unary();
        while self.peek().tk_type == TokenType::And {
            self.idx += 1;
            left = Node::BinOp { left: Box::new(left), op: 10, right: Box::new(self.parse_unary()) }
        }
        return left;
    }
    fn parse_unary(&mut self)->Node {
        if self.peek().tk_type == TokenType::Minus {
            let op = self.peek().tk_type.clone();
            self.idx += 1;
            return Node::Unary { op: if op == TokenType::Not{1} else {0}, value: Box::new(self.parse_unary()) }
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
            TokenType::NewVector => {
                self.eat(TokenType::NewVector);
                self.eat(TokenType::Lparen);
                if [TokenType::BoolT,TokenType::IntT,TokenType::FloatT,TokenType::StringT].contains(&self.peek().tk_type) {
                    let vectype = self.peek().tk_type.clone();
                    self.eat(vectype.clone());
                    self.eat(TokenType::Rparen);
                    return Node::NewVector { vectype }
                } else { panic!("error: unexpected type `{:?}` for vector ",self.peek().tk_type) }
            }
            TokenType::FreeVector => {
                self.eat(TokenType::FreeVector);
                self.eat(TokenType::Lparen);
                let vector = self.eat(TokenType::Id).clone();
                self.eat(TokenType::Rparen);
                return Node::FreeVector { vector }
            }
            TokenType::GetVector => {
                self.eat(TokenType::GetVector);
                self.eat(TokenType::Lparen);
                let vector = self.eat(TokenType::Id).clone();
                self.eat(TokenType::Comma);
                let index = self.peek().span.parse::<u64>().expect("error: expected an integer for the index");
                self.idx += 1;
                self.eat(TokenType::Rparen);
                return Node::GetVector { vector, index }
            }
            TokenType::PushVector => {
                self.eat(TokenType::PushVector);
                self.eat(TokenType::Lparen);
                let vector = self.eat(TokenType::Id).clone();
                self.eat(TokenType::Comma);
                let elem = Box::new(self.parse_expr());
                self.eat(TokenType::Rparen);
                return Node::PushVector { vector, elem }
            }
            TokenType::Cblock(code) => {
                self.idx += 1;
                return Node::Cblock { code }
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
