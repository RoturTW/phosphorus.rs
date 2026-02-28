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
    ValueTypeMismatch(String, String),
    InvalidAnchor(String),
    InvalidAlignment(String),
    InvalidElemType(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Placeholder =>
                write!(f, "placeholder err :P"),
            
            // ast
            Error::UnexpectedToken { token, .. } =>
                write!(f, "unexpected token '{token}'"),
            Error::Expected { wanted, got, ..} =>
                write!(f, "expected {}, got '{}'",
                       wanted
                           .iter()
                           .map(|t| format!("'{t}'"))
                           .collect::<Vec<String>>()
                           .join(" or "),
                       got
                ),
            Error::ExpectedText { got, ..} =>
                write!(f, "expected text, got {got}"),
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
            Error::ValueTypeMismatch(wanted, got) =>
                write!(f, "expected {wanted}, got {got}"),
            Error::InvalidAnchor(got) =>
                write!(f, "invalid anchor '{got}'"), // add list of anchors?
            Error::InvalidAlignment(got) =>
                write!(f, "invalid alignment '{got}'"),  // add list of alignments?
            Error::InvalidElemType(type_name) =>
                write!(f, "cannot have {type_name} as element")
        }
    }
}
