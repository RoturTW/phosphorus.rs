use std::fmt::{Display, Formatter};
use crate::shared::range::Range;
use crate::shared::token::{Token, TokenType};

#[derive(Debug)]
pub enum Error {
    Placeholder,
    
    // ast
    UnexpectedToken {
        token: Box<Token>,
        range: Box<Range>
    },
    Expected {
        wanted: Vec<TokenType>,
        got: Box<Token>,
        range: Box<Range>
    },
    ExpectedText {
        got: Box<Token>,
        range: Box<Range>
    },
    ExpectedNum {
        got: Box<Token>,
        range: Box<Range>
    },
    
    TestNeedsName,
    TestNeedsCode,
    InvalidCodeType {
        range: Box<Range>
    },
    TextMustHaveIndent
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Placeholder =>
                write!(f, "placeholder err :P"),
            
            // ast
            Error::UnexpectedToken { token, .. } =>
                write!(f, "unexpected token '{token}'"),
            Error::Expected { wanted, got, range } =>
                write!(f, "expected {}, got '{got}' at {range}",
                       wanted
                           .iter()
                           .map(|t| format!("'{t}'"))
                           .collect::<Vec<String>>()
                           .join(" or ")
                ),
            Error::ExpectedText { got, ..} =>
                write!(f, "expected text, got {got}"),
            Error::ExpectedNum { got, ..} =>
                write!(f, "expected num, got {got}"),
            
            Error::TestNeedsName =>
                write!(f, "test needs name"),
            Error::TestNeedsCode =>
                write!(f, "test needs code"),
            Error::InvalidCodeType { .. } =>
                write!(f, "invalid code type, expected expr or program"),
            Error::TextMustHaveIndent =>
                write!(f, "code / result must have indent"),
        }
    }
}
