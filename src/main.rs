mod lexer;
mod parser;
mod ast;
mod ir_emiter;
mod backend;

use lexer::*;
use parser::*;
use backend::*;
use std::io::Write;
use std::time::{Duration, Instant};
use std::fs;



fn main() {
    let mut debug = false;
    let mut time = false;
    let mut only_lex = false;
    let args = std::env::args().collect::<Vec<String>>();
    let mut i: usize = 1;
    loop {
        if i == args.len() { break; }
        let carg = args[i].as_str();
        match carg {
            "--debug" => {
                debug = true;
            }
            "--time" => {
                time = true;
            }
            "--lex-only" => {
                only_lex = true;
            }
            _ => {

            }
        }
        i += 1;
    }

    let lex_time = |time_lex: std::time::Duration, lex: &Lexer| {
        if debug { println!("{:#?} >> {}", lex.tokens,time_lex.as_secs_f64()) }
        else if time { println!("lexing time: {:.5} (tokens generated: {}; lines: {})",time_lex.as_secs_f64(), lex.tokens.len(), lex.curr_line) }
    };
    let parse_time = |time_parse: std::time::Duration, program: &ast::Program| {
        if debug { println!("{:#?} >> {}", program.nodes,time_parse.as_secs_f64()) }
        else if time { println!("parsing time: {:.5} (roots generated: {}, size: {:.10} mb)",time_parse.as_secs_f64(), program.nodes.len(), program.size() as f64 / 1_048_576.0) }
    };
    let codegen_time = |time_codegen: Duration, be: &Backend| {
        if time { println!("codegen time: {:.5} (lines generated: {})",time_codegen.as_secs_f64(),be.c.lines().count() - crate::backend::BOILERPLATE.lines().count()) }
    };

    let source = fs::read_to_string("main.par").unwrap();
    let mut start = Instant::now();
    let mut lex = Lexer::new(source);
    lex.lexer();
    let time_lex = start.elapsed();
    
    if only_lex {
        lex_time(time_lex,&lex);
        return;
    }

    start = Instant::now();
    let mut pars = Parser::new(lex.tokens.clone());
    let program = pars.parse_program();
    let time_parse = start.elapsed();

    start = Instant::now();
    let mut be = Backend::new();
    be.emit_c(&program);
    let time_codegen = start.elapsed();

    lex_time(time_lex,&lex);
    parse_time(time_parse,&program);
    codegen_time(time_codegen,&be);

    let mut out = fs::File::create("out.c").expect("cannot create file");
    out.write(be.c.as_bytes()).expect("cannot write the file");
}
