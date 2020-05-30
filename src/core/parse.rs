use super::lex::*;
use super::token::*;

pub fn parse(inStr: &str)
{

    let mut l = Lexer::new(inStr);
    
    // 1. Lex/parse all functions
    // 2. Grab and Compile 'main'
    // 3. 

    loop
    {
        match l.lex().unwrap()
        {
            Token::NULL => (),
            _ => (),
        }
    }
    


}