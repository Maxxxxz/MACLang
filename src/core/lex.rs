mod token;
use token::Token::{self, *};

// A Lexer object
// Contains the content that will be lexed
pub struct Lexer<'z>
{
    content: &'z Vec<String>
}

// Lifetimes stop the object's data from being lost in other variables when going out of scope?
impl<'z> Lexer<'z>
{
    pub fn new(inVec: Vec<String>) -> Self
    {
        Lexer
        {
            content: &inVec
        }
    }

    pub fn next(self: &mut Lexer<'z>) -> Result<Token, String>
    {
        Ok(NULL)
    }

}

pub fn start(inVec: Vec<String>)
{

    let l = Lexer::new(inVec);
    lex(l)

}

// Temporary for now
pub fn lex(ls: Lexer)// -> Vec<Token>
{
    for word in ls.content
    {
        match word
        {
            _ => println!("recieved: {}", word),
        }
    }
}