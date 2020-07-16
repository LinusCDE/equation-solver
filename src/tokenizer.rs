use crate::tokenizer::Number::{Integer, Decimal};
use crate::tokenizer::Operator::{Addition, Subtraction, Multiplication, Division, Modulo};
use crate::tokenizer::Token::{NumberTokenType, OperatorTokenType, GroupTokenType};
use std::fmt::Debug;

use regex::Regex;

#[derive(Debug)]
pub enum Number {
    Integer(i64),
    Decimal(f64)
}

#[derive(Debug)]
pub struct NumberToken {
    pub number: Number
}

#[derive(Debug, Copy, Clone)]
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
pub struct GroupToken { // ( ... )
    pub tokens: Vec<Token>
}

#[derive(Debug)]
pub enum Token {
    NumberTokenType(NumberToken),
    OperatorTokenType(OperatorToken),
    GroupTokenType(GroupToken),
}

impl Token {
    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        match self {
            NumberTokenType(token) => token.number.as_string(),
            OperatorTokenType(token) => token.operator.as_str().to_owned(),
            GroupTokenType(token) => token.as_string(),
        }
    }

    #[allow(dead_code)]
    pub fn type_name(&self) -> &str {
        match self {
            NumberTokenType(_) => "NumberToken",
            OperatorTokenType(_) => "OperatorToken",
            GroupTokenType(_) => "NestedToken",
        }
    }

    pub fn parse(content: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut cursor = 0;

        while cursor < content.len() {
            let (accepted_length, token) = OperatorToken::from(&content[cursor..]);

            if let Some(token) = token {
                //println!("The operator consumed {len} chars, is of type {ttype} and as \
                //as_string \"{str}\"",
                //         len = le, ttype = token.type_name(), str = token.as_string());
                cursor += accepted_length;
                tokens.push(OperatorTokenType(token));
                continue
            }

            let (accepted_length, token) = NumberToken::from(&content[cursor..]);

            if let Some(token) = token {
                //println!("The number token consumed {len} chars, is of type {ttype} and as \
                //as_string \"{str}\" (integer = {is_integer}, decimal = {is_decimal})",
                //         len = le, ttype = token.type_name(), str = token.as_string(),
                //         is_integer = token.is_integer(), is_decimal = token.is_decimal());
                cursor += accepted_length;
                tokens.push(NumberTokenType(token));
                continue
            }

            let (accepted_length, token) = GroupToken::from(&content[cursor..]);

            if let Some(token) = token {
                //println!("The operator consumed {len} chars, is of type {ttype} and as \
                //as_string \"{str}\"",
                //         len = le, ttype = token.type_name(), str = token.as_string());
                cursor += accepted_length;
                tokens.push(GroupTokenType(token));
                continue
            }

            cursor += 1;
        }

        tokens
    }
}

impl Number {
    pub fn as_string(&self) -> String {
        match self {
            Integer(integer) => format!("{}", integer),
            Decimal(decimal) => format!("{}", decimal)
        }
    }

    #[allow(dead_code)]
    pub fn is_integer(&self) -> bool {
        return if let Integer(_) = self { true } else { false }
    }

    #[allow(dead_code)]
    pub fn is_decimal(&self) -> bool {
        return if let Decimal(_) = self { true } else { false }
    }

    pub fn as_decimal(&self) -> f64 {
        match self {
            Integer(integer) => *integer as f64,
            Decimal(decimal) => *decimal,
        }
    }

    pub fn as_integer(&self) -> Result<i64, ()> {
        match self {
            Integer(integer) => Ok(*integer),
            Decimal(_) => Err(()),
        }
    }
}

impl NumberToken {
    pub fn from(content: &str) -> (usize, Option<NumberToken>) {
        //println!("Given: {}", content);
        let offset: usize;
        lazy_static! {
            static ref PATTERN: Regex = Regex::new("-?[0-9]+(\\.[0-9]+)?").unwrap();
        }

        let content: String = match PATTERN.find(content) {
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
    #[allow(dead_code)]
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
        let offset: usize;
        lazy_static! {
            static ref PATTERN: Regex = Regex::new("[-+*/%]").unwrap();
        }
        let content: String = match PATTERN.find(content) {
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

impl GroupToken {
    pub fn from(content: &str) -> (usize, Option<GroupToken>) {
        //println!("Given: {}", content);
        if content.len() == 0 {
            return (0, None)
        }

        if ! content[0..1].eq_ignore_ascii_case("(") {
            return (0, None)
        }

        let mut nested_count = 0;
        let mut i: i32 = -1;
        let length: usize = 'find_context: loop {
            for char in content.chars() {
                i += 1;
                if char == '(' {
                    nested_count += 1
                } else if char == ')' {
                    nested_count -= 1;
                    if nested_count == 0 {
                        break 'find_context (i + 1) as usize
                    }
                }
            }
            break 'find_context 0 // Invalid (didn't find matching closing bracket in given content)
        };

        if length > 0 {
            let inner_content = &content[1..length-1]; // "("inner_content")"
            (length, Some(GroupToken { tokens: Token::parse(inner_content)}))
        }else {
            (0, None)
        }
    }

    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        let mut string = String::from("(");
        for token in self.tokens.iter() {
            string.push_str(token.as_string().as_str());
        }
        string.push_str(")");
        string
    }
}

impl Clone for Number {
    fn clone(&self) -> Self {
        match self {
            Integer(integer) => Integer(*integer),
            Decimal(decimal) => Decimal(*decimal),
        }
    }
}

impl Clone for NumberToken {
    fn clone(&self) -> Self {
        return NumberToken { number: self.number.clone() }
    }
}

impl Clone for OperatorToken {
    fn clone(&self) -> Self {
        return OperatorToken { operator: self.operator.clone() }
    }
}

impl Clone for GroupToken {
    fn clone(&self) -> Self {
        let mut tokens: Vec<Token> = Vec::new();
        for token in self.tokens.iter() {
            tokens.push(token.clone());
        }
        GroupToken { tokens }
    }
}

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            GroupTokenType(token) => GroupTokenType(token.clone()),
            OperatorTokenType(token) => OperatorTokenType(token.clone()),
            NumberTokenType(token) => NumberTokenType(token.clone()),
        }
    }
}