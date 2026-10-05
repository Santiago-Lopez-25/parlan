//! Syntactic Analyzer or Parser using a Recursive Descent approach

use crate::ast::*;
use crate::error::*;
use crate::error;
use crate::lexer::*;

/// Represents the Syntactic Analyzer
pub struct Parser<'parser> {
    lexer: std::iter::Peekable<Lexer<'parser>>,
    file: &'parser SourceFile<'parser>,
    src: &'parser str,
    pub errors: Vec<Diagnostic>
}

impl<'parser> Parser<'parser> {
    pub fn new(file: &'parser SourceFile, src: &'parser str, lexer: Lexer<'parser>) -> Self {
        Self {
            lexer: lexer.peekable(),
            file,
            src,
            errors: Vec::new()
        }
    }

    /// Returns the current token advancing if is not the end of the file,
    /// otherwise it returns `None`
    fn next_token(&mut self) -> Option<Token> {
        loop {
            match self.lexer.next()? {
                Ok(token) => return Some(token),
                Err(diag) => {
                    self.errors.push(diag);
                }
            }
        }
    }

    /// Returns the current token without advancing if is not the end of the file,
    /// otherwise it returns `None` 
    fn peek_token(&mut self) -> Option<&Token> {
        loop {
            if let Some(Err(_)) = self.lexer.peek() {
                if let Some(Err(diag)) = self.lexer.next() {
                    self.errors.push(diag);
                }
            } else {
                return match self.lexer.peek() {
                    Some(Ok(token)) => Some(token),
                    _ => None
                }
            }
        }
    }

    /// Skips tokens until it reaches a synchronization point (a `;`, `var`, or `func`)
    fn synchronize(&mut self) {
        while let Some(tok) = self.peek_token() {
            match tok.kind {
                TokenKind::Semi | TokenKind::VarKw | TokenKind::FuncKw | TokenKind::CloseBrace => return,
                _ => {
                    self.next_token();
                }
            }
        }
    }

    /// Takes a [`TokenKind`] and checks if the current tokens matchs the type,
    /// returns the token if it match, or returns the token's span if not
    fn expect(
        &mut self, 
        kind: TokenKind,
        msg: String,
        label_msg: String,
    ) -> Result<Token, Span> {
        match self.peek_token() {
            Some(tk) if tk.kind == kind => Ok(self.next_token().unwrap()),
            Some(other) => {
                let other = other.clone();
                self.errors.push(
                    error!(
                        other.span,
                        label_msg,
                        "{msg}"
                    )
                );
                self.synchronize();
                Err(other.span)
            }
            None => {
                self.errors.push(
                    error!(
                        Span(self.src.len() - 1, self.src.len()),
                        "".into(),
                        "unexpected end of file"
                    )
                );
                Err(Span(self.src.len() - 1, self.src.len()))
            }
        }
    }

    fn get_span(&self, span: Span) -> &str {
        &self.src[span.0..span.1]
    }

    fn parse_expr(&mut self) -> Expr<'parser> {
        let start_span = self.peek_token().map(|t| t.span).unwrap_or(Span(0,0));
        
        if let Some(token) = self.next_token() {
            match token.kind {
                TokenKind::Int => Expr::Literal(Literal::Integer(self.get_span(token.span).parse().unwrap())),
                _ => {
                    self.errors.push(
                        error!(
                            token.span,
                            "expected an expression".into(),
                            "expected an expression, found `{}` instead",
                            self.get_span(token.span)
                        )
                    );
                    self.synchronize();
                    Expr::Error(start_span)   
                }
            }
        } else {
            self.errors.push(
                error!(
                    start_span,
                    "expected an expression".into(),
                    "unexpected end of file"
                )
            );
            return Expr::Error(start_span)
        }
    }

    fn parse_stmt_var_decl(&mut self) -> Stmt<'parser> {
        let start_span = self.peek_token().map(|t| t.span).unwrap_or(Span(0,0));

        // consume `var`
        self.next_token();

        let curr = self.peek_token().unwrap().kind;
        let name = match self.expect(
            TokenKind::Id, 
            format!("expected identifier after `var`, found {} instead", curr),
            "expected identifier".into()
        ) {
            Ok(tok) => tok,
            Err(span) => return Stmt::Error(Span(start_span.0, span.1))
        };

        // TODO: accept an optional explicit type
        let ty = Ty::Unknown;

        let curr = self.peek_token().unwrap().kind;
        match self.expect(
            TokenKind::Assign, 
            format!("expected `=`, found {} instead", curr), 
            "expected `=` here".into()
        ) {
            Ok(_) => {}
            Err(span) => return Stmt::Error(Span(start_span.0, span.1))
        };

        let expr = self.parse_expr();

        match self.expect(
            TokenKind::Semi, 
            "expected `;` after variable declaration".into(), 
            "expected `;` here".into()
        ) {
            Ok(_) => {}
            Err(span) => return Stmt::Error(Span(start_span.0, span.1))
        };

        Stmt::VarDecl {
            name: &self.src[name.span.0..name.span.1], 
            ty, 
            expr 
        }
    }

    fn parse_stmt_block(&mut self) -> Stmt<'parser> {
        match self.expect(
            TokenKind::OpenBrace, 
            "expected `{`".into(), 
            "expected `{` here".into(),
        ) {
            Ok(_) => {},
            Err(span) => return Stmt::Error(span)
        };

        let mut stmts = Vec::new();

        while self.peek_token().is_some() && self.peek_token().unwrap().kind != TokenKind::CloseBrace {
            stmts.push(self.parse_stmt());
        }

        match self.expect(
            TokenKind::CloseBrace, 
            "expected `}`".into(), 
            "expected `}` here".into()
        ) {
            Ok(_) => {},
            Err(span) => return Stmt::Error(span)
        };

        Stmt::Block { stmts }
    }

    pub fn parse_stmt(&mut self) -> Stmt<'parser> {
        match self.peek_token() {
            Some(tk) if tk.kind == TokenKind::VarKw => self.parse_stmt_var_decl(),
            Some(tk) if tk.kind == TokenKind::OpenBrace => self.parse_stmt_block(),
            Some(other) => {
                let span = other.span;

                self.errors.push(
                    error!(
                        span,
                        "".into(),
                        "expected a statemet"
                    )
                );
                self.synchronize();
                Stmt::Error(span)
            }
            None => {
                self.errors.push(
                    error!(
                        Span(self.src.len() - 1, self.src.len()),
                        "expected a statement".into(),
                        "unexpected end of file, expected an statement"
                    )
                );
                Stmt::Error(Span(self.src.len(), self.src.len()))
            }
        }
    }

    fn parse_item_func_decl(&mut self) -> AstItem<'parser> {
        let start_span = self.peek_token().map(|t| t.span).unwrap_or(Span(0,0));

        // consume `func`
        self.next_token();

        let name = match self.expect(
            TokenKind::Id, 
            "expected identifier after `func`".into(),
            "expected identifier".into()
        ) {
            Ok(tk) => tk,
            Err(span) => return AstItem::Error(Span(start_span.0, span.1))
        };

        match self.expect(
            TokenKind::OpenParen, 
            "expected `(` after function name".into(), 
            "unterminaded function declaration".into(),
        ) {
            Ok(_) => {},
            Err(span) => return AstItem::Error(Span(start_span.0, span.1))
        };

        let args: Vec<(Expr, Ty)> = vec![];

        match self.expect(
            TokenKind::CloseParen, 
            "expected `)`, functions doesn't support parameters yet".into(),
            "expected `)`".into()
        ) {
            Ok(_) => {},
            Err(span) => return AstItem::Error(Span(start_span.0, span.1))
        };

        // TODO: accept optional return type (or default to void)
        let ret = Ty::Void;

        let body = self.parse_stmt_block();
        
        AstItem::Function { 
            name: &self.src[name.span.0..name.span.1], 
            args, 
            ret, 
            body 
        }
    }

    fn parse_item(&mut self) -> AstItem<'parser> {
        match self.peek_token() {
            Some(tk) if tk.kind == TokenKind::FuncKw => self.parse_item_func_decl(),
            Some(other) => {
                let span = other.span;
                self.errors.push(
                    error!(
                        span,
                        "".into(),
                        "expected an item"
                    )
                );
                self.synchronize();
                AstItem::Error(span)
            }
            None => {
                self.errors.push(
                    error!(
                        Span(self.src.len() - 1, self.src.len()),
                        "expected an item".into(),
                        "unexpected end of file"
                    )
                );
                AstItem::Error(Span(self.src.len(), self.src.len()))
            }
        }
    }

    pub fn parse(&mut self) -> AstModule<'parser> {
        let mut items = Vec::new();
        
        while let Some(_) = self.peek_token() {
            items.push(self.parse_item());
        }

        AstModule { 
            file: self.file, 
            items 
        }
    }
}