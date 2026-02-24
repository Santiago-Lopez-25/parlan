#![allow(dead_code)]

use std::{collections::HashMap};

#[derive(Debug,Clone,PartialEq)]
pub enum TokenType {
    NewVector,
    FreeVector,

    Var,
    If,
    Else,
    Func,
    Return,
    Id,
    Colon,
    Semicolon,
    Comma,
    Assing,
    Plus,
    Minus,
    Star,
    Slash,
    Not,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    IntT,
    FloatT,
    BoolT,
    StringT,
    VecT,
    IntL(usize),
    FloatL(f64),
    BoolL(bool),
    StringL(String),
    Eof
}

#[derive(Clone,PartialEq)]
pub struct Tk {
    pub tk_type: TokenType,
    pub span: String,
    pub line: usize
}
impl Tk {
    pub fn new(tk_type: TokenType, span:String, line:usize)->Self {
        return Tk{
            tk_type,
            span,
            line
        };
    }
}
impl std::fmt::Debug for Tk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Token [{}] {:?} >> `{}`",self.line,self.tk_type, self.span)
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct Lexer {
    source: String,
    pos: usize,
    pub curr_line: usize,
    pub tokens: Vec<Tk>
}
impl Lexer {
    pub fn new(source: String)->Self {
        return Lexer{
            source,
            pos: 0,
            curr_line: 0,
            tokens: Vec::new()
        };
    }
    pub fn lexer(&mut self) {
        let keywords = HashMap::from([
            ("__new_vector",TokenType::NewVector),
            ("__free_vector",TokenType::FreeVector),
            ("var",TokenType::Var),
            ("int",TokenType::IntT),
            ("float",TokenType::FloatT),
            ("bool",TokenType::BoolT),
            ("string",TokenType::StringT),
            ("vector",TokenType::VecT),
            ("func",TokenType::Func),
            ("if",TokenType::If),
            ("else",TokenType::Else),
            ("return",TokenType::Return),
            ("true",TokenType::BoolL(true)),
            ("false",TokenType::BoolL(false)),
            ("not",TokenType::Not)
        ]);
        let symbols = HashMap::from([
            (';',TokenType::Semicolon),
            (':',TokenType::Colon),
            (',',TokenType::Comma),
            ('=',TokenType::Assing),
            ('+',TokenType::Plus),
            ('-',TokenType::Minus),
            ('*',TokenType::Star),
            ('/',TokenType::Slash),
            ('<',TokenType::Lt),
            ('>',TokenType::Gt),
            ('(',TokenType::Lparen),
            (')',TokenType::Rparen),
            ('{',TokenType::Lbrace),
            ('}',TokenType::Rbrace)
        ]);
        let mut temp = String::new();
        let chars = self.source.chars().collect::<Vec<char>>();
        loop {
            if self.pos == self.source.len() {
                self.tokens.push(Tk::new(TokenType::Eof, temp.clone(), self.curr_line));
                break;
            }
            let c = chars[self.pos];
            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    while self.pos < self.source.len() {
                        if chars[self.pos].is_alphanumeric() || chars[self.pos] == '_' {
                            temp.push(chars[self.pos]);
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                    if keywords.contains_key(&temp.as_str()) {
                        self.tokens.push(Tk::new(keywords.get(&temp.as_str()).unwrap().clone(), temp.clone(), self.curr_line));
                        temp = String::new();
                    } else {
                        self.tokens.push(Tk::new(TokenType::Id, temp.clone(), self.curr_line));
                        temp = String::new();
                    }
                }
                '0'..='9' => {
                    let mut dot = false;
                    while self.pos < self.source.len() {
                        if chars[self.pos].is_ascii_digit() || chars[self.pos] == '.' {
                            if chars[self.pos] == '.' && dot {
                                panic!("error [{}]: malformed numeric literal expected only one dot, found two",self.curr_line);
                            } else if chars[self.pos] == '.' {
                                dot = true;
                            }
                            temp.push(chars[self.pos]);
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                    if dot {
                        self.tokens.push(Tk::new(TokenType::FloatL(temp.parse().unwrap()), temp.clone(), self.curr_line));
                        temp = String::new()
                    } else {
                        self.tokens.push(Tk::new(TokenType::IntL(temp.parse().unwrap()), temp.clone(), self.curr_line));
                        temp = String::new()
                    }
                }
                '"' => {
                    self.pos += 1;
                    while self.pos < chars.len() {
                        if chars[self.pos] != '"' {
                            temp.push(chars[self.pos]);
                            self.pos += 1;
                        } else {
                            self.pos += 1;
                            break;
                        }    
                    }
                    self.tokens.push(Tk::new(TokenType::StringL(temp.clone()), temp, self.curr_line));
                    temp = String::new();
                }
                _ => {
                    if c == '/' && chars[self.pos + 1] == '/' {
                        self.pos += 1;
                        while self.pos < chars.len() {
                            if chars[self.pos] == '\n' {
                                break;
                            }
                            self.pos += 1;
                        }
                    } else if c == '>' && chars[self.pos + 1] == '=' {
                        self.pos += 2;
                        self.tokens.push(Tk::new(TokenType::Ge, ">=".to_string(), self.curr_line));
                    }  else if c == '<' && chars[self.pos + 1] == '=' {
                        self.pos += 2;
                        self.tokens.push(Tk::new(TokenType::Le, "<=".to_string(), self.curr_line));
                    }  else if c == '!' && chars[self.pos + 1] == '=' {
                        self.pos += 2;
                        self.tokens.push(Tk::new(TokenType::Ne, "!=".to_string(), self.curr_line));
                    }  else if c == '=' && chars[self.pos + 1] == '=' {
                        self.pos += 2;
                        self.tokens.push(Tk::new(TokenType::Eq, "==".to_string(), self.curr_line));
                    } else if symbols.contains_key(&c) {
                        self.tokens.push(Tk::new(symbols.get(&c).unwrap().clone(), c.to_string(), self.curr_line));
                        self.pos += 1;
                    } else if c.is_whitespace() {
                        if c == '\n' { self.curr_line += 1 }
                        self.pos += 1;
                    } else {
                        panic!("error: unknown start of token `{c}` at line {}",self.curr_line);
                    }
                }
            }
        }
    }
}