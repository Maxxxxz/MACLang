use super::lex::*;
use super::token::*;

pub struct Parser<'z>
{
    lexer: &'z mut Lexer<'z>,

}

impl<'z> Parser<'z>
{
    pub fn parse(inStr: &str)// -> what to return to compiler? 🤔
    {
    
        let mut l = Lexer::new(inStr);
        
        // 1. Lex/parse all functions
        // 2. Grab and Compile 'main'
        // 3. 
    
        loop
        {
            match l.lex().unwrap()
            {
                Token::FN   => (),
                Token::NULL => (),
                _ => (),
            }
        }
        
    
    
    }
}