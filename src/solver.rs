use crate::tokenizer::{*};
use crate::tokenizer::Number::{*};
use crate::tokenizer::Token::{*};
use crate::tokenizer::Operator::{*};

fn add(n1: &Number, n2: &Number) -> Number {
    if n1.is_decimal() || n2.is_decimal() {
        Decimal(n1.as_decimal() + n2.as_decimal())
    }else {
        Integer(n1.as_integer().unwrap() + n2.as_integer().unwrap()) // Should never crash
    }
}

fn subtract(n1: &Number, n2: &Number) -> Number {
    if n1.is_decimal() || n2.is_decimal() {
        Decimal(n1.as_decimal() - n2.as_decimal())
    }else {
        Integer(n1.as_integer().unwrap() - n2.as_integer().unwrap()) // Should never crash
    }
}

fn multiply(n1: &Number, n2: &Number) -> Number {
    if n1.is_decimal() || n2.is_decimal() {
        Decimal(n1.as_decimal() * n2.as_decimal())
    }else {
        Integer(n1.as_integer().unwrap() * n2.as_integer().unwrap()) // Should never crash
    }
}

fn divide(n1: &Number, n2: &Number) -> Number {
    if n1.is_decimal() || n2.is_decimal() {
        Decimal(n1.as_decimal() / n2.as_decimal())
    }else {
        Integer(n1.as_integer().unwrap() / n2.as_integer().unwrap()) // Should never crash
    }
}

fn modulo(n1: &Number, n2: &Number) -> Number {
    if n1.is_decimal() || n2.is_decimal() {
        Decimal(n1.as_decimal() % n2.as_decimal())
    }else {
        Integer(n1.as_integer().unwrap() % n2.as_integer().unwrap()) // Should never crash
    }
}

pub fn solve(tokens: &Vec<Token>) -> Result<Number, String> {
    let mut first: Option<&Token> = None;
    let mut op: Option<&Token> = None;
    let mut last: Option<&Token> = None;
    let mut has_more = false;

    for token in tokens.iter() {
        if let None = first {
            first = Some(token);
        }else if let None = op {
            op = Some(token);
        }else if let None = last {
            last = Some(token);
        }else {
            has_more = true;
            break;
        }
    }

    let result = if first.is_some() && op.is_none() && last.is_none() && matches!(*first.unwrap(), GroupTokenType(_)) {
        if let GroupTokenType(group) = first.unwrap() {
            solve(&group.tokens)
        }else {
            Err(String::from("Unexpected case!!! (1)"))
        }
    }else if first.is_some() && op.is_some() && last.is_some() && matches!(*op.unwrap(), OperatorTokenType(_)) {
        if let OperatorTokenType(operator) = op.unwrap() {
            let first_num = match first.unwrap() {
                OperatorTokenType(_) => return Err(String::from("First token can't be an operator!")),
                GroupTokenType(group) => solve(&group.tokens)?,
                NumberTokenType(num) => num.number.clone(),
            };
            let last_num = match last.unwrap() {
                OperatorTokenType(_) => return Err(String::from("First token can't be an operator!")),
                GroupTokenType(group) => solve(&group.tokens)?,
                NumberTokenType(num) => num.number.clone(),
            };

            // Calculate
            match operator.operator {
                Addition => Ok(add(&first_num, &last_num)),
                Subtraction => Ok(subtract(&first_num, &last_num)),
                Multiplication => Ok(multiply(&first_num, &last_num)),
                Division => Ok(divide(&first_num, &last_num)),
                Modulo => Ok(modulo(&first_num, &last_num)),
            }
        }else {
            Err(String::from("Unexpected case!!! (2)"))
        }
    }else {
        return Err(format!("Unknown case!"));
    };

    if has_more && result.is_ok() {
        let mut new_tokens: Vec<Token> = Vec::new();
        new_tokens.push(NumberTokenType(NumberToken { number: result.unwrap() }));
        let mut i = -1;
        for token in tokens.iter() {
            i += 1;
            if i > 2 {
                new_tokens.push(token.clone())
            }
        }
        solve(&new_tokens)
    }else {
        result
    }
}