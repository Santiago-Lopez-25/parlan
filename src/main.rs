mod lexer;

use lexer::*;
use std::time::Instant;
use std::fs;



fn main() {
    let mut debug = false;
    let args = std::env::args().collect::<Vec<String>>();
    let mut i: usize = 1;
    loop {
        if i == args.len() { break; }
        let carg = args[i].as_str();
        match carg {
            "--debug" => {
                debug = true;
            }
            _ => {

            }
        }
        i += 1;
    }

    let start = Instant::now();
    let source = fs::read_to_string("main.par").unwrap();
    let mut lex = Lexer::new(source);
    lex.lexer();
    if debug { println!("{:#?} >> {}", lex.tokens,start.elapsed().as_secs_f64()) }
}
