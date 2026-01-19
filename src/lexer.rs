#![allow(dead_code, unused_imports)]

use std::collections::HashMap;

use crate::lexer;

#[derive(Debug,Clone,PartialEq)]
pub enum TokenType {
    Var,
    Id(String),
    Colon,
    Semicolon,
    Assing,
    IntT,
    IntL(usize),
    Eof
}

#[derive(Debug,Clone,PartialEq)]
pub struct Tk {
    tk_type: TokenType,
    span: String,
    line: usize
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

#[derive(Debug,Clone,PartialEq)]
pub struct Lexer {
    source: String,
    pos: usize,
    curr_line: usize,
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
    fn jump_space(&mut self) {
        loop {
            if self.pos == self.source.len() {break;}
            if [" ","\n","\r","\t"].contains(&&self.source[self.pos..self.pos+1]) {
                self.pos += 1;
            } else {
                break;
            }
        }
    }
    pub fn lexer(&mut self) {
        let keywords = HashMap::from([
            ("var",TokenType::Var),
            ("int",TokenType::IntT)
        ]);
        let symbols = HashMap::from([
            (';',TokenType::Semicolon),
            (':',TokenType::Colon),
            ('=',TokenType::Assing)
        ]);
        let mut temp = String::new();
        let chars = self.source.chars().collect::<Vec<char>>();
        loop {
            if self.pos == self.source.len() {
                self.tokens.push(Tk::new(TokenType::Eof, "eof".to_string(), self.curr_line));
                break;
            }
            let c = chars[self.pos];
            match c {
                'a'..'z' | 'A'..'Z' | '_' => {
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
                        self.tokens.push(Tk::new(TokenType::Id(temp.clone()), temp.clone(), self.curr_line));
                        temp = String::new();
                    }
                }
                '0'..'9' => {
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
                        todo!()
                    } else {
                        self.tokens.push(Tk::new(TokenType::IntL(temp.parse().unwrap()), temp.clone(), self.curr_line));
                        temp = String::new()
                    }
                }
                _ => {
                    if symbols.contains_key(&c) {
                        self.tokens.push(Tk::new(symbols.get(&c).unwrap().clone(), c.to_string(), self.curr_line));
                        self.pos += 1;
                    } else if c.is_whitespace() {
                        if c == '\n' { self.curr_line += 1 }
                        self.pos += 1;
                    } else {
                        panic!("error: unknown start of token `{c}`");
                    }
                }
            }
        }
    }
}