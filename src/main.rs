/*
this is the main file, where the pipeline starts and ends
*/

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
    // these variables are used to know if the user wants debugs prints, time logs or only the tokens
    // this is especially for development
    let mut debug = false;
    let mut time = false;
    let mut only_lex = false;
    let mut source_file = "";
    let args = std::env::args().collect::<Vec<String>>(); // we collect the command line arguments into a vector
    
    /*
    here, i use a infinite loop instead of a for loop because if we use a for loop
    we cannot advance the index to access the next argument.
    well, actually with only this 3 commands we don't need to advance the index to see the next argument because we don't need to see it
    but i use a loop statement anyway to show the idea
    */
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
            file_name => {
                source_file = file_name
            }
        }
        i += 1;
    }

    /*
    i decided to use closures because is more tiny, but you can use a normal function instead 
    */
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

    let source = fs::read_to_string(source_file).unwrap(); // we read the file into a String
    let mut start = Instant::now(); // start the timer
    let mut lex = Lexer::new(source); // we initialize a new Lexer instance
    lex.lexer(); // and we tokenize the source code into a vector of tokens
    let time_lex = start.elapsed(); // now we stop the clock and see how much time it took to tokenize
    
    if only_lex { // if we passed the `--lex-only` flag, we just want the tokens
        lex_time(time_lex,&lex);
        return;
    }

    // now we do the same process as above but for the generation of the AST
    start = Instant::now();
    let mut pars = Parser::new(lex.tokens.clone());
    let program = pars.parse_program();
    let time_parse = start.elapsed();

    // and the same process!
    start = Instant::now();
    let mut be = Backend::new();
    be.emit_c(&program);
    let time_codegen = start.elapsed();

    // now we call all of our debug functions (well, actually closures)
    lex_time(time_lex,&lex);
    parse_time(time_parse,&program);
    codegen_time(time_codegen,&be);

    // and we write all the generated code into the output file (out.c)
    let mut out = fs::File::create("out.c").expect("cannot create file");
    out.write(be.c.as_bytes()).expect("cannot write the file");
}
