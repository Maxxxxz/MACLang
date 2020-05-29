#[path="./lex.rs"]
mod lex;
use lex::*;

pub fn parse(inStr: &str)
{

    let l = Lexer::new(inStr);
    
    // 1. Lex/parse all functions
    // 2. Grab and Compile 'main'
    // 3. 

    loop
    {
        l.lex();
    }
    


}