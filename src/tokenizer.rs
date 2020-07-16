use crate::tokenizer::Number::{Integer, Decimal};
use regex::Regex;

pub trait Token {
    fn as_string(&self) -> String;
    fn type_name(&self) -> &str;
    // TODO: find possibility to declare constructor method from(content: &str) -> (usize, Option<something Self>)
}

pub enum Number {
    Integer(i64),
    Decimal(f64)
}

impl Number {
    fn as_string(&self) -> String {
        match self {
            Integer(integer) => format!("{}", integer),
            Decimal(decimal) => format!("{}", decimal)
        }
    }
}

pub struct NumberToken {
    pub number: Number
}

impl NumberToken {
    pub fn is_integer(&self) -> bool {
        return if let Integer(_) = self.number { true } else { false }
    }

    pub fn is_decimal(&self) -> bool {
        return if let Decimal(_) = self.number { true } else { false }
    }

    pub(crate) fn from(content: &str) -> (usize, Option<NumberToken>) {
        println!("Given: {}", content);
        let mut offset: usize = 0;
        let pattern = Regex::new("-?[0-9]+(\\.[0-9]+)?").unwrap();
        let content: String = match pattern.find(content) {
            Some(result) => {
                let res_string = content[result.start()..result.end()].to_owned();
                println!("Match from {start} to {end}: {str}", start=result.start(), end=result.end(), str=res_string);
                offset = result.start();
                res_string
            },
            None => return (0, None)
        };

        if content.len() == 0 {
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

impl Token for NumberToken {
    fn as_string(&self) -> String {
        self.number.as_string()
    }

    fn type_name(&self) -> &str {
        "NumberToken"
    }
}