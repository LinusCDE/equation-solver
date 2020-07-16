use std::{env,io};
use std::io::Write;
use crate::tokenizer::Token;
use crate::solver::solve;

mod tokenizer;
mod solver;

fn main() {
    // Get raw_equation either as first passed argument or prompt from user
    let args: Vec<String> = env::args().collect();
    let raw_equation: String = if args.len() > 1 {
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
    let parsed = Token::parse(&raw_equation);
    /*for token in &parsed {
        println!("----------");
        println!("Debug:  {:?}", token);
        println!("String: {}", token.as_string());
    }*/

    match solver::solve(&parsed) {
        Ok(result) => {
            println!("Solved: {}", result.as_string());
        },
        Err(message) => {
            println!("Error while solving: {}", message);
        }
    }
}
