use std::{env,io};
use std::io::Write;
use crate::tokenizer::{NumberToken, Token};

mod tokenizer;

fn main() {
    // Get raw_equation either as first passed argument or prompt from user
    let args: Vec<String> = env::args().collect();
    let raw_equation: String = if(args.len() > 1) {
        args[1].trim().to_owned()
    }else {
        let mut line = String::new();
        loop {
            print!("Enter the equation: ");
            io::stdout().flush();
            match io::stdin().read_line(&mut line) {
                Ok(length) => {
                    line = line.trim().to_owned(); // raw line has \n at end. trimming that away.
                    if line.len() > 0 {
                        break line
                    }
                },
                Err(e) => { }
            }
        }
    };

    // Remove all spaces and tabs
    let raw_equation = raw_equation.replace(" ", "").replace("\t", "").to_owned();

    println!("You entered: {}", raw_equation);

    let (le, token) = NumberToken::from(raw_equation.as_str());

    match token {
        Some(token) => {
            println!("The number token consumed {len} chars, is of type {ttype} and \
            as as_string \"{str}\" (integer = {is_integer}, decimal = {is_decimal})",
                     len=le, ttype=token.type_name(), str=token.as_string(),
                     is_integer=token.is_integer(), is_decimal=token.is_decimal())
        },
        None => {
            println!("The number token consumed {len} chars. No token as returned", len=le)
        }
    }
}
