use crate::tokenizer::Number::{Integer, Decimal};
use regex::Regex;
use crate::tokenizer::Operator::{Addition, Subtraction, Multiplication, Division, Modulo};

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

impl Token for NumberToken {
    fn as_string(&self) -> String {
        self.number.as_string()
    }

    fn type_name(&self) -> &str {
        "NumberToken"
    }
}

pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
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

pub struct OperatorToken {
    pub operator: Operator
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

impl Token for OperatorToken {
    fn as_string(&self) -> String {
        self.operator.as_str().to_owned()
    }

    fn type_name(&self) -> &str {
        "OperatorToken"
    }
}