mod token;
use token::Token::{self, *};

// A Lexer object
// Contains the content that will be lexed
pub struct Lexer<'z>
{
    pub content: &'z str,
    pub location: LocInfo,
    commentDepth: u64,
}

// A Location Info object
// Contains the current location of the reader
pub struct LocInfo
{
    row: u64,
    col: u64,
}

impl LocInfo
{
    fn nextRow(&mut self, x: u64)
    {
        self.row = self.row + x;
        self.col = 0;
    }

    fn nextCol(&mut self, x: u64)
    {
        self.col = self.col + x;
    }
}

// Lifetimes stop the object's data from being lost in other variables when going out of scope?
impl<'z> Lexer<'z>
{
    pub fn new(inStr: &'z str) -> Self
    {
        Lexer
        {
            content: &inStr,
            location: LocInfo {row: 0, col: 0},
            commentDepth: 0,
        }
    }

    pub fn next(self: &mut Lexer<'z>) -> Result<Token, String>
    {
        self.lex
    }

    pub fn lex(self: &mut Lexer<'z>) -> Result<Token, String>
    {
        let ls = self;
        let toLex = ls.content;

        if toLex.starts_with("fn"){Ok(Token::FN)}
        else if toLex.starts_with("/*")
        {
            ls.commentDepth = ls.commentDepth + 1;
            ls.content = toLex.split_at(2).1;
            return ls.lex();
        }
        else if toLex.starts_with("*/")
        {
            ls.commentDepth = ls.commentDepth - 1;
            ls.content = toLex.split_at(2).1;
            return ls.lex();
        }
        // else if toLex.starts_with("")
        else    // match in here
        {
            Err("Unexpected Token".to_string())
        }

        // println!("recieved: {}", ls.content);

    }

}