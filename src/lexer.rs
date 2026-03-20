/*
The Lexer, Scanner or Tokenizer:

the Lexer is the first part of a compiler, it breaks the source code (usually text),
into a stream of tokens. these tokens are the smallest meaningful element within the source code,
and every single structure of the source language is formated of tokens.

---- EXAMPLES ----
the best way to understand is to see a simple example, here is one:

taking this simple source code (in C language):
```
void something() {}
```
the lexer will output the following tokens (the names of the tokens are referencial):
[
TK_voidtype,
TK_identifier,
TK_left_paren,
TK_right_paren,
TK_left_brace,
TK_right_brace
]

if you see the example above you will see that the lexer doesn't generate an `whitespace token`,
that because the white spaces doesn't matter; that why in language like rust or every other
it doesn't matter if you write ```let x = 5;``` or ```let       x=    5   ;```, because the compiler just 
skips the white spaces!
*/

#![allow(dead_code)]

use std::{collections::HashMap};

// we use a simple enum to represent every type of token
#[derive(Debug,Clone,PartialEq)]
pub enum TokenType {
    NewVector,
    FreeVector,
    GetVector,
    PushVector,

    Var,
    If,
    Else,
    Func,
    Return,
    While,
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
    Cblock(String),
    Eof
}

// an struct to represent and store information of a token
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
impl std::fmt::Debug for Tk { // a simple debug printer
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Token [{}] {:?} >> `{}`",self.line,self.tk_type, self.span)
    }
}

/*
the main struct of the lexer, storing all relevant information like
the source code, the current index, current line and, obviusly, the tokens!
*/
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
    // this is the main function
    pub fn lexer(&mut self) {
        /*
        to maintain the code simple i decided to use the most simple but functional way to create a lexer (as i know), the comments
        above every partexplines it: 
        */

        // here i used a hashmap to store all the keywords.
        // because the keywords will never changes we can pair them with their
        // repective token type, and the best way to store pairs is using a
        // hash map (due to its near O(1) performance in almost all its operations)
        let keywords = HashMap::from([
            ("__new_vector",TokenType::NewVector),
            ("__free_vector",TokenType::FreeVector),
            ("__get_vector",TokenType::GetVector),
            ("__push_vector",TokenType::PushVector),
            ("var",TokenType::Var),
            ("int",TokenType::IntT),
            ("float",TokenType::FloatT),
            ("bool",TokenType::BoolT),
            ("string",TokenType::StringT),
            ("vector",TokenType::VecT),
            ("func",TokenType::Func),
            ("while",TokenType::While),
            ("if",TokenType::If),
            ("else",TokenType::Else),
            ("return",TokenType::Return),
            ("true",TokenType::BoolL(true)),
            ("false",TokenType::BoolL(false)),
            ("not",TokenType::Not)
        ]);
        // we use the same aproach as above, because the symbols will always be the same
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

        /* now we enter in the token recognition and generation */

        // we use a string to store the character in the current index
        // the we use this string to recognize the token
        let mut temp = String::new();

        // as in rust we cannot index an String directly, i convert the source into a vector of characters
        let chars = self.source.chars().collect::<Vec<char>>();
        loop {
            // if we have reached the end of the code, we generate the EOF (End Of File) token and exit of the loop
            if self.pos == self.source.len() {
                self.tokens.push(Tk::new(TokenType::Eof, temp.clone(), self.curr_line));
                break;
            }

            // we get the character in the current position (or index)
            let c = chars[self.pos];
            match c {
                'a'..='z' | 'A'..='Z' | '_' => { // if the char is alphabetic (or a '_'), that mean is and identifier or a keyword
                    while self.pos < self.source.len() {
                        if chars[self.pos].is_alphanumeric() || chars[self.pos] == '_' { // after the first letter, a identifier can have numbers or '_' (underscore)
                            temp.push(chars[self.pos]);
                            self.pos += 1; // always advance the position
                        } else {
                            break;
                        }
                    }
                    if temp.as_str() == "c_code" { // we need to handle our special case of c_code string
                        while chars[self.pos] == ' ' {self.pos += 1}
                        if chars[self.pos] == '"' && chars[self.pos+1] == '"' {
                            self.pos += 2;
                            let mut t = String::new();
                            while !(chars[self.pos] == '"' && chars[self.pos+1] == '"') {
                                t.push(chars[self.pos]);
                                self.pos += 1;
                            }
                            self.pos += 2; 
                            self.tokens.push(Tk::new(TokenType::Cblock(t.clone()), t, self.curr_line)); // we push a new token
                        }
                        temp = String::new(); // we restore the temporal string
                    } else if keywords.contains_key(&temp.as_str()) { // if our keyword hashmap contains the auxiliar string, is a keyword
                        self.tokens.push(Tk::new(keywords.get(&temp.as_str()).unwrap().clone(), temp.clone(), self.curr_line));
                        temp = String::new();
                    } else { // if not, is an identifier
                        self.tokens.push(Tk::new(TokenType::Id, temp.clone(), self.curr_line));
                        temp = String::new();
                    }
                }
                '0'..='9' => { // if the start is a number, must be a number
                    let mut dot = false; // a sentinel flag
                    while self.pos < self.source.len() {
                        if chars[self.pos].is_ascii_digit() || chars[self.pos] == '.' { // if the char is a number or is a dot, is a number
                            if chars[self.pos] == '.' && dot { // a number cannot have two dots
                                // we use panic! for simplicity
                                panic!("error [{}]: malformed numeric literal expected only one dot, found two",self.curr_line);
                            } else if chars[self.pos] == '.' { // we have not found any dot so is correct
                                dot = true;
                            }
                            temp.push(chars[self.pos]);
                            self.pos += 1;
                        } else {
                            break;
                        }
                    }
                    if dot { // if the number has a dot, is a float
                        self.tokens.push(Tk::new(TokenType::FloatL(temp.parse().unwrap()), temp.clone(), self.curr_line));
                        temp = String::new()
                    } else { // else is a integer
                        self.tokens.push(Tk::new(TokenType::IntL(temp.parse().unwrap()), temp.clone(), self.curr_line));
                        temp = String::new()
                    }
                }
                '"' => { // this must be a string
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
                    if c == '/' && chars[self.pos + 1] == '/' { // a single line comment
                        self.pos += 1;
                        while self.pos < chars.len() {
                            if chars[self.pos] == '\n' {
                                break;
                            }
                            self.pos += 1;
                        }
                    } else if c == '/' && chars[self.pos + 1] == '*' { // a multiline comment
                        self.pos += 1;
                        while self.pos < chars.len() {
                            if chars[self.pos] == '*' && chars[self.pos + 1] == '/' {
                                self.pos += 2;
                                break;
                            }
                            self.pos += 1;
                        }
                    } 
                    // in this section we identify the 2 character operators
                    else if c == '>' && chars[self.pos + 1] == '=' {
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
                    } 
                    // and for single character operators
                    else if symbols.contains_key(&c) {
                        self.tokens.push(Tk::new(symbols.get(&c).unwrap().clone(), c.to_string(), self.curr_line));
                        self.pos += 1;
                    } else if c.is_whitespace() { // we don't want white spaces
                        if c == '\n' { self.curr_line += 1 }
                        self.pos += 1;
                    } else {
                        // if all of the above conditions failed, that is an error!
                        // again, we use panic! just for simplicity 
                        panic!("error: unknown start of token `{c}` at line {}",self.curr_line);
                    }
                }
            }
        }
    }
}