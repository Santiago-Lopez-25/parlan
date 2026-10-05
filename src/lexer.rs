//! Lexer/Tokenizer and Token definitions

use crate::error::*;
use crate::error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Keywords
    VarKw,
    FuncKw,

    // Single-Character Symbols & Operators
    Assign,
    Semi,
    Colon,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,

    // Literals
    Int,
    Id,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::VarKw => write!(f, "`var`"),
            TokenKind::FuncKw => write!(f, "`func`"),
            TokenKind::Assign => write!(f, "`=`"),
            TokenKind::Semi => write!(f, "`;`"),
            TokenKind::Colon => write!(f, "`:`"),
            TokenKind::OpenParen => write!(f, "`(`"),
            TokenKind::CloseParen => write!(f, "`)`"),
            TokenKind::OpenBrace => write!(f, "`{{`"),
            TokenKind::CloseBrace => write!(f, "}}"),
            TokenKind::Int => write!(f, "integer"),
            TokenKind::Id => write!(f, "identifier"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Clone)]
pub struct Lexer<'lex> {
    cursor: usize,
    bytes: &'lex [u8],
}

impl<'lex> Lexer<'lex> {
    pub fn new(src: &'lex str) -> Self {
        let bytes = src.as_bytes();
        if bytes[bytes.len() - 1] != b'\0' {
            eprintln!("\x1b[31;1minternal error:\x1b[0m an unexpected error happend inside the compiler itself.\ninfo: lexer expected NUL terminated source.");
            std::process::exit(2);
        }

        Self {
            cursor: 0,
            bytes,
        }
    }

    fn byte(&self) -> u8 {
        self.bytes[self.cursor]
    }

    fn lookup_keyword(&self, start: usize) -> TokenKind {
        match &self.bytes[start..self.cursor] {
            b"var" => TokenKind::VarKw,
            b"func" => TokenKind::FuncKw,
            _ => TokenKind::Id
        }
    }

    fn lookup_character(&self, c: u8) -> Option<TokenKind> {
        match c {
            b'=' => Some(TokenKind::Assign),
            b';' => Some(TokenKind::Semi),
            b':' => Some(TokenKind::Colon),
            b'(' => Some(TokenKind::OpenParen),
            b')' => Some(TokenKind::CloseParen),
            b'{' => Some(TokenKind::OpenBrace),
            b'}' => Some(TokenKind::CloseBrace),
            _ => None
        }
    }

}

impl<'lex> Iterator for Lexer<'lex> {
    type Item = Result<Token, Diagnostic>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.byte() {
                c if c.is_ascii_whitespace() => self.cursor += 1,
                _ => break
            }
        }

        match (self.byte(), self.cursor) {
            // The NUL byte (0) represents the end of the byte stream
            (0,_) => None,

            // Identifiers and keywords
            (b'a'..=b'z' | b'A'..=b'Z' | b'_', start) => {
                while self.byte().is_ascii_alphanumeric() || self.byte() == b'_' { self.cursor += 1; }

                Some(Ok(Token { kind: self.lookup_keyword(start), span: Span(start, self.cursor) }))
            }

            // Number literals
            (b'0'..=b'9', start) => {
                while self.byte().is_ascii_digit() { self.cursor += 1; }

                Some(Ok(Token { kind: TokenKind::Int, span: Span(start, self.cursor) }))
            }

            // Sinlge character tokens
            ch if let Some(kind) = self.lookup_character(ch.0) => {
                self.cursor += 1;
                Some(Ok(Token { kind, span: Span(ch.1, self.cursor) }))
            }

            // Character not identified
            (b, start) => {
                self.cursor += 1;
                Some(Err(
                    error!(
                        Span(start, start),
                        "".into(),
                        "unknown start of token `{}`", b as char
                    )
                ))
            }
        }
    }
}