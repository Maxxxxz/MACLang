use std::env;
use std::fs::{File, read_to_string};
use std::io::{Read};

//  From my understanding of module, main calls both lex and parse
//  Both lex and parse call token, meaning that in order to have
//  the same types we must bring it into scope directly above both
//  of them by doing this.
// #[path="./core/lex.rs"]
// mod lex;
// #[path="./core/parse.rs"]
// mod parse;
// #[path="./core/token.rs"]
// mod token;

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() > 1
    {
        let inFileName = &args[1];
        println!("{}", inFileName);
        let toLex = readFile(inFileName);
        // call parser here
        // lex::start(toLex.as_str());
    }
    else
    {
        panic!("Not enough args!");
    }

    
}

fn readFile(inFile: &str) -> String //Vec<String>
{
    let mut file = File::open(inFile).unwrap();

    let mut out: String = String::new();
    file.read_to_string(&mut out);

    // let tVec: Vec<&str> = out.split_whitespace().collect();

    // let mut outVec = Vec::new();

    // for a in tVec
    // {
    //     outVec.push(String::from(a))
    // }

    return out;
}