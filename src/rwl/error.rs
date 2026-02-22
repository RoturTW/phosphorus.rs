use std::fmt::{Display, Formatter};
use crate::rwl::ast::range::Range;
use crate::rwl::ast::token::{Token, TokenType};

#[derive(Debug)]
pub struct ErrPosition;

#[derive(Debug)]
pub enum Error {
    Placeholder,
    
    // ast
    UnexpectedToken {
        token: Token,
        range: Range
    },
    Expected {
        wanted: Vec<TokenType>,
        got: Token,
        range: Range
    },
    ExpectedText {
        got: Token,
        range: Range
    },
    CouldntParseNum,
    InvalidAttributeKey,
    InvalidHexLength,
    UnknownPropertySource {
        source: String
    },
    UnknownProperty {
        source: String,
        property: String
    },
    
    // update
    ValueTypeMismatch(ErrPosition, String, String),
    InvalidAnchor(ErrPosition, String),
    InvalidAlignment(ErrPosition, String),
    InvalidElemType(ErrPosition, String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Placeholder =>
                write!(f, "placeholder err :P"),
            
            // ast
            Error::UnexpectedToken { token, .. } =>
                write!(f, "unexpected token '{}'", token.to_string()),
            Error::Expected { wanted, got, ..} =>
                write!(f, "expected {}, got '{}'",
                       wanted
                           .into_iter()
                           .map(|t| format!("'{}'", t.to_string()))
                           .collect::<Vec<String>>()
                           .join(" or "),
                       got.to_string()
                ),
            Error::ExpectedText { got, ..} =>
                write!(f, "expected text, got {}", got.to_string()),
            Error::CouldntParseNum =>
                write!(f, "couldnt parse num"),
            Error::InvalidAttributeKey =>
                write!(f, "attribute key must consist of letters or _"),
            Error::InvalidHexLength =>
                write!(f, "hex values can only be 3 or 6 characters long"),
            Error::UnknownPropertySource { source } =>
                write!(f, "unknown property source '{source}'"),
            Error::UnknownProperty { source, property } =>
                write!(f, "unknown property '{property}' on {source}"),
            
            // update
            Error::ValueTypeMismatch(_, wanted, got) =>
                write!(f, "expected {wanted}, got {got}"),
            Error::InvalidAnchor(_, got) =>
                write!(f, "invalid anchor '{got}'"), // add list of anchors?
            Error::InvalidAlignment(_, got) =>
                write!(f, "invalid alignment '{got}'"),  // add list of alignments?
            Error::InvalidElemType(_, type_name) =>
                write!(f, "cannot have {type_name} as element")
        }
    }
}
