//! Entry point of the compiler

use crate::cli::cli;
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term;

// modules
//mod error;
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

            let mut files = SimpleFiles::new();
            let file_id = files.add(input.file_name().unwrap().to_string_lossy(), &src);
            
            let writer = StandardStream::stderr(ColorChoice::Auto);
            let diag_config = term::Config::default();

            let lexer = lexer::Lexer::new(&src, file_id);

            if config.dump_tokens {
                let tokens: Vec<_> = lexer.clone().collect();
                for tk in tokens {
                    match tk {
                        Ok(tk) => eprintln!("{tk:?}"),
                        Err(diag) => term::emit_to_write_style(&mut writer.lock(), &diag_config, &files, &diag).unwrap()
                    }
                }
            }

            if config.lexing_only {
                return;
            }

            let mut parser = parser::Parser::new(file_id, &src, lexer);
            let ast = parser.parse();

            if config.dump_ast {
                eprintln!("{ast:#?}");
            }
            for err in parser.errors {
                term::emit_to_write_style(&mut writer.lock(), &diag_config, &files, &err).unwrap();
            }
        }
    }
}