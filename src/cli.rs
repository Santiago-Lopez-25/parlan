//! Command Line Interface Of Parlan
//! 
//! This CLI uses [`lexopt`] to handle commands and flags more easily. It could be done by hand
//! but using [`lexopt`] let us focus on other things like the parser or semantic analyzer while
//! having a decent CLI.

// Added temporarily to eliminate warnings
// TODO: delete this 
#![allow(unused)]

use std::path::PathBuf;
use lexopt::prelude::*;

pub enum Command {
    Build {
        input: PathBuf,
        output: PathBuf,
        cc: String,
    },
}

pub struct Config {
    pub cmd: Command,
    pub dump_tokens: bool,
    pub dump_ast: bool,
    pub lexing_only: bool,
}

const HELP_MSG: &str = 
r#"Usage: parlan [OPTIONS] [COMMANDS]

Commands:
        build <INPUT>    Compile INPUT into an executable (not yet fully implemented)

Options:
    -o <OUTPUT>          Write executable into OUTPUT [defaults to INPUT name] [build only] (not yet implemented)
        --cc <CC>        Use CC to compile the resulting C code [defaults to Clang] [build only] (not yet implemented)
        --help           Print this message and exit
        --version        Print version information and exit

Debug Options:
        --dump-tokens    Print the token stream to stderr
        --dump-ast       Print the ast to stderr
        --lexing-only    Stop after tokenizing input
"#;

pub fn cli() -> Result<Config, lexopt::Error> {
    let mut command = None;
    let mut dump_tokens = false;
    let mut dump_ast = false;
    let mut lexing_only = false;
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Value(ref val) => {
                let cmd = val.clone().string()?;
                match cmd.as_str() {
                    "build" => {
                        command = Some(parse_build(&mut parser)?);
                    }
                    _ => return Err(arg.unexpected())
                }
            }
            Long("help") => {
                println!("{}", HELP_MSG);
                std::process::exit(0);
            }
            Long("version") => {
                println!("parlan v0.3 (under development)");
                std::process::exit(0);
            }
            Long("dump-tokens") => dump_tokens = true,
            Long("dump-ast") => dump_ast = true,
            Long("lexing-only") => lexing_only = true,
            _ => return Err(arg.unexpected())
        }
    }
    
    Ok(Config { 
        cmd: command.unwrap(), 
        dump_tokens,
        dump_ast,
        lexing_only,
    })
}

fn parse_build(parser: &mut lexopt::Parser) -> Result<Command, lexopt::Error> {
    let mut input = PathBuf::default();
    let mut output = None;
    let mut cc = "clang".to_string();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('o') => {
                output = Some(PathBuf::from(parser.value()?));
            }
            Long("cc") => {
                cc = parser.value()?.to_string_lossy().to_string();
            }
            Value(arg) => {
                input = PathBuf::from(arg);
            }
            _ => return Err(arg.unexpected())
        }
    }

    Ok(Command::Build { 
        input: input.clone(), 
        output: if let Some(output) = output {
            output
        } else {
            input.with_extension("exe")
        },
        cc
    })
}