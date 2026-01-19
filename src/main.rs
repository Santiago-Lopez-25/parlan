mod lexer;

use lexer::*;

fn main() {
    let source = String::from(r#"
var id: int = 575;    
"#);
    let mut lex = Lexer::new(source);
    lex.lexer();
    println!("{:#?}", lex.tokens)
}
