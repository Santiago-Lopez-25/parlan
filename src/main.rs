mod lexer;
mod parser;
mod ast;
mod ir_emiter;
mod backend;

use lexer::*;
use parser::*;
use backend::*;
use std::io::Write;
use std::time::Instant;
use std::fs;



fn main() {
    let mut debug = false;
    let mut time = false;
    let mut check = false;
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
            "--check_gen" => {
                check = true;
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

    /* not used 
    start = Instant::now();
    let mut emiter = IrEmiter::new();
    emiter.emit_ir(&program);
    if debug { println!("{:#?} >> {}", emiter.ir,start.elapsed().as_secs_f64()) }
    else if time { println!("IR generation time: {:.5} (roots generated: {})",start.elapsed().as_secs_f64(), emiter.ir.len()) }
    */

    start = Instant::now();
    let mut be = Backend::new();
    be.emit_c(&program);
    if debug { println!("please, go to `out.c` file to see generated code") }
    if time { println!("codegen time: {:.5} (lines generated: {})",start.elapsed().as_secs_f64(),be.c.lines().count()) }
    let mut out = fs::File::create("out.c").expect("some error ocurred while creating out file");
    out.write(be.c.as_bytes()).expect("error while trying to write to out file");
    if check { 
        let cc = std::process::Command::new("clang").args(["out.c","-fsyntax-only"]).output().expect("error");
        println!("check: {}. error (if aplycable):\n{} ",if cc.status.success() {"✅"} else {"❌"},String::from_utf8(cc.stderr).expect("error: stderr has invalid utf8 characters"))
    }
}
