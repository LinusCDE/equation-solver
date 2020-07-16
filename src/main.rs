use std::{env,io};
use std::io::Write;
use crate::tokenizer::{NumberToken, Token, OperatorToken};
use std::ptr::eq;

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
    //let raw_equation = raw_equation.replace(" ", "").replace("\t", "").to_owned();

    println!("You entered: {}", raw_equation);

    let mut cursor = 0;
    while cursor < raw_equation.len() {
        let (le, token) = NumberToken::from(&raw_equation[cursor..]);

        if let Some(token) = token {
                println!("The number token consumed {len} chars, is of type {ttype} and as \
                as_string \"{str}\" (integer = {is_integer}, decimal = {is_decimal})",
                         len = le, ttype = token.type_name(), str = token.as_string(),
                         is_integer = token.is_integer(), is_decimal = token.is_decimal());
            cursor += le;
            continue
        }

        let (le, token) = OperatorToken::from(&raw_equation[cursor..]);

        if let Some(token) = token {
            println!("The operator consumed {len} chars, is of type {ttype} and as \
                as_string \"{str}\"",
                     len = le, ttype = token.type_name(), str = token.as_string());
            cursor += le;
            continue
        }

        // No matched found at current pos. Try next one
        cursor += 1
    }
}
