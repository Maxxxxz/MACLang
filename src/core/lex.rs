use token::*

// A Lexer object
// Contains the content that will be lexed
pub struct Lexer
{
    content: Vec<String>
}

impl Lexer
{
    pub fn new(inVec: Vec<String>) -> Self
    {
        Lexer
        {
            content: inVec
        }
    }

    pub fn next(self: &mut Lexer) -> Result<Token, &'static str>
    {
        ...
    }

}

pub fn start(inVec: Vec<String>)
{

    let mut l = Lexer::new(inVec);

    lex(l)

}

// Temporary for now
pub fn lex(ls: Lexer)// -> Vec<Token>
{
    for word in inVec
    {
        match word
        {
            _ => println!("recieved: {}", word),
        }
    }
}