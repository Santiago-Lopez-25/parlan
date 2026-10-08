//! Entry point of the compiler

use crate::cli::cli;
use crate::error::SourceFile;

// modules
mod error;
mod cli;
mod lexer;
mod parser;
mod ast;

fn main() {
    let config = match cli() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    match config.cmd {
        cli::Command::Build { input, output: _, cc: _ } => {
            let mut src = match std::fs::read_to_string(&input) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("error while reading input file: {e}");
                    std::process::exit(1);
                }
            };
            src.push(0 as char); // append NUL byte

            let file = SourceFile::new(input.file_name().unwrap().to_str().unwrap(), &src);

            let lexer = lexer::Lexer::new(&src);

            if config.dump_tokens {
                let tokens: Vec<_> = lexer.clone().collect();
                for tk in tokens {
                    match tk {
                        Ok(tk) => eprintln!("{tk:?}"),
                        Err(diag) => diag.emit(&file),
                    }
                }
            }

            if config.lexing_only {
                return;
            }

            let mut parser = parser::Parser::new(&file, &src, lexer);
            let ast = parser.parse();

            if config.dump_ast {
                eprintln!("{ast:#?}");
            }

            for err in parser.errors {
                err.emit(&file);
            }
        }
    }
}