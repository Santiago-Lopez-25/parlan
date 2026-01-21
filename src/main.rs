mod lexer;
mod parser;
mod ast;

use lexer::*;
use parser::*;
use std::time::Instant;
use std::fs;



fn main() {
    let mut debug = false;
    let mut time = false;
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
            _ => {

            }
        }
        i += 1;
    }
    
    let source = fs::read_to_string("main.par").unwrap();
    let mut start = Instant::now();
    let mut lex = Lexer::new(source);
    lex.lexer();
    if debug { println!("{:#?} >> {}", lex.tokens,start.elapsed().as_secs_f64()) }
    else if time { println!("lexing time: {:.5} (tokens generated: {}; lines: {})",start.elapsed().as_secs_f64(), lex.tokens.len(), lex.curr_line) }

    start = Instant::now();
    let mut pars = Parser::new(lex.tokens.clone());
    let program = pars.parse_program();
    if debug { println!("{:#?} >> {}", program.nodes,start.elapsed().as_secs_f64()) }
    else if time { println!("parsing time: {:.5} (roots generated: {}, size: {:.10} mb)",start.elapsed().as_secs_f64(), program.nodes.len(), program.size() as f64 / 1_048_576.0) }
}
