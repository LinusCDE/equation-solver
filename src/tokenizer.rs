use crate::tokenizer::Number::{Integer, Decimal};

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
    number: Number
}

impl NumberToken {
    fn is_integer(&self) -> bool {
        return if let Integer(_) = self.number { true } else { false }
    }

    fn is_decimal(&self) -> bool {
        return if let Decimal(_) = self.number { true } else { false }
    }

    pub(crate) fn from(content: &str) -> (usize, Option<NumberToken>) {
        if content.len() == 0 {
            return (0, None)
        }

        if let Ok(parsed_integer) = content.parse::<i64>() {
            return (content.len(), Some(NumberToken { number: Integer(parsed_integer) }))
        }
        if let Ok(parsed_decimal) = content.parse::<f64>() {
            return (content.len(), Some(NumberToken { number: Decimal(parsed_decimal) }))
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