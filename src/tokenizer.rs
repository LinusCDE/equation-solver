use crate::tokenizer::Number::{Integer, Decimal};
use regex::Regex;
use crate::tokenizer::Operator::{Addition, Subtraction, Multiplication, Division, Modulo};
use crate::tokenizer::Token::{NumberTokenType, OperatorTokenType};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Number {
    Integer(i64),
    Decimal(f64)
}

#[derive(Debug)]
pub struct NumberToken {
    pub number: Number
}

#[derive(Debug)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
}

#[derive(Debug)]
pub struct OperatorToken {
    pub operator: Operator
}

#[derive(Debug)]
pub enum Token {
    NumberTokenType(NumberToken),
    OperatorTokenType(OperatorToken),
}

impl Token {
    pub fn as_string(&self) -> String {
        match self {
            NumberTokenType(token) => token.number.as_string(),
            OperatorTokenType(token) => token.operator.as_str().to_owned(),
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            NumberTokenType(_) => "NumberToken",
            OperatorTokenType(_) => "OperatorToken",
        }
    }

    pub fn parse(content: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut cursor = 0;

        while cursor < content.len() {
            let (le, token) = NumberToken::from(&content[cursor..]);

            if let Some(token) = token {
                //println!("The number token consumed {len} chars, is of type {ttype} and as \
                //as_string \"{str}\" (integer = {is_integer}, decimal = {is_decimal})",
                //         len = le, ttype = token.type_name(), str = token.as_string(),
                //         is_integer = token.is_integer(), is_decimal = token.is_decimal());
                cursor += le;
                tokens.push(NumberTokenType(token));
                continue
            }

            let (le, token) = OperatorToken::from(&content[cursor..]);

            if let Some(token) = token {
                //println!("The operator consumed {len} chars, is of type {ttype} and as \
                //as_string \"{str}\"",
                //         len = le, ttype = token.type_name(), str = token.as_string());
                cursor += le;
                tokens.push(OperatorTokenType(token));
                continue
            }

            cursor += 1;
        }

        tokens
    }
}

impl Number {
    fn as_string(&self) -> String {
        match self {
            Integer(integer) => format!("{}", integer),
            Decimal(decimal) => format!("{}", decimal)
        }
    }
}

impl NumberToken {
    pub fn is_integer(&self) -> bool {
        return if let Integer(_) = self.number { true } else { false }
    }

    pub fn is_decimal(&self) -> bool {
        return if let Decimal(_) = self.number { true } else { false }
    }

    pub fn from(content: &str) -> (usize, Option<NumberToken>) {
        //println!("Given: {}", content);
        let mut offset: usize = 0;
        let pattern = Regex::new("-?[0-9]+(\\.[0-9]+)?").unwrap();
        let content: String = match pattern.find(content) {
            Some(result) => {
                let res_string = content[result.start()..result.end()].to_owned();
                //println!("Match from {start} to {end}: {str}", start=result.start(), end=result.end(), str=res_string);
                offset = result.start();
                res_string
            },
            None => return (0, None)
        };

        if content.len() == 0 || offset > 0 {
            return (0, None)
        }

        if let Ok(parsed_integer) = content.parse::<i64>() {
            return (offset + content.len(), Some(NumberToken { number: Integer(parsed_integer) }))
        }
        if let Ok(parsed_decimal) = content.parse::<f64>() {
            return (offset + content.len(), Some(NumberToken { number: Decimal(parsed_decimal) }))
        }

        (0, None)
    }
}

impl Operator {
    fn as_str(&self) -> &str {
        match self {
            Addition => "+",
            Subtraction => "−",
            Multiplication => "*",
            Division => "/",
            Modulo => "%",
        }
    }
}

impl OperatorToken {
    pub fn from(content: &str) -> (usize, Option<OperatorToken>) {
        //println!("Given: {}", content);
        let mut offset: usize = 0;
        let pattern = Regex::new("[-+*/%]").unwrap();
        let content: String = match pattern.find(content) {
            Some(result) => {
                let res_string = content[result.start()..result.end()].to_owned();
                //println!("Match from {start} to {end}: {str}", start=result.start(), end=result.end(), str=res_string);
                offset = result.start();
                res_string
            },
            None => return (0, None)
        };

        if content.len() == 0 || offset > 0 {
            return (0, None)
        }

        if content.eq_ignore_ascii_case("+") {
            return (offset + content.len(), Some(OperatorToken { operator: Addition }))
        }
        if content.eq_ignore_ascii_case("-") {
            return (offset + content.len(), Some(OperatorToken { operator: Subtraction }))
        }
        if content.eq_ignore_ascii_case("*") {
            return (offset + content.len(), Some(OperatorToken { operator: Multiplication }))
        }
        if content.eq_ignore_ascii_case("/") {
            return (offset + content.len(), Some(OperatorToken { operator: Division }))
        }
        if content.eq_ignore_ascii_case("%") {
            return (offset + content.len(), Some(OperatorToken { operator: Modulo }))
        }

        (0, None)
    }
}