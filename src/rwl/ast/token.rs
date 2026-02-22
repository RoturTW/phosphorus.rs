use std::cmp::PartialEq;
use std::fmt::{Debug, Display, Formatter};
use crate::rwl::ast::range::Range;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub range: Range
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token {:?} at {}", self.token_type, self.range)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_type.to_string())
    }
}

impl PartialEq<Token> for TokenType {
    fn eq(&self, other: &Token) -> bool {
        *self == other.token_type
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        self.token_type == *other
    }
}

impl PartialEq<&str> for Token {
    fn eq(&self, other: &&str) -> bool {
        self.token_type == *other
    }
}

impl PartialEq<Token> for Token {
    fn eq(&self, other: &Token) -> bool {
        self.token_type == *other
    }
}

impl PartialEq<&str> for TokenType {
    fn eq(&self, other: &&str) -> bool {
        if let TokenType::Text(txt) = &self {
            txt.as_str() == *other
        } else {
            false
        }
    }
}

/*
impl PartialEq<TokenType> for TokenType {
    fn eq(&self, other: &TokenType) -> bool {
        if let (
            TokenType::Text(a_txt),
            TokenType::Text(b_txt)
        ) = (self, other.clone()) {
            *a_txt == b_txt
        } else {
            *self == *other
        }
    }
}
*/

#[derive(Clone, PartialEq)]
pub enum TokenType {
    OpenParen, CloseParen,
    OpenSquare, CloseSquare,
    OpenCurly, CloseCurly,
    
    LeftArrow, RightArrow,
    
    Comma, SemiColon, Colon, Period, Hash,
    Plus, Minus, Star, Slash, Mod, Pow,
    ExclamationMark, QuestionMark,
    BackSlash, Pipe, Equal,
    
    Quote, DoubleQuote, BackQuote,
    
    Space, Newline,
    EOF,
    
    Text(String)
}

pub const TOKENS: [char; 29] = [
    '(', ')',
    '[', ']',
    '{', '}',
    
    '<', '>',
    
    ',',';',':','.','#',
    '+','-','*','/','%','^',
    '!', '?',
    '\\', '|', '=',
    
    '\'','"','`',
    
    ' ','\n',
];

impl From<char> for TokenType {
    fn from(value: char) -> Self {
        match value {
            '(' => TokenType::OpenParen, ')' => TokenType::CloseParen,
            '[' => TokenType::OpenSquare, ']' => TokenType::CloseSquare,
            '{' => TokenType::OpenCurly, '}' => TokenType::CloseCurly,
            
            '<' => TokenType::LeftArrow, '>' => TokenType::RightArrow,
            
            ',' => TokenType::Comma, ';' => TokenType::SemiColon, ':' => TokenType::Colon, '.' => TokenType::Period, '#' => TokenType::Hash,
            '+' => TokenType::Plus, '-' => TokenType::Minus, '*' => TokenType::Star, '/' => TokenType::Slash, '%' => TokenType::Mod, '^' => TokenType::Pow,
            '!' => TokenType::ExclamationMark, '?' => TokenType::QuestionMark,
            '\\' => TokenType::BackSlash, '|' => TokenType::Pipe, '=' => TokenType::Equal,
            
            '\'' => TokenType::Quote, '"' => TokenType::DoubleQuote, '`' => TokenType::BackQuote,
            
            ' ' => TokenType::Space, '\n' => TokenType::Newline,
            
            _ => panic!("unknown TokenType {}", value)
        }
    }
}

impl From<String> for TokenType {
    fn from(value: String) -> Self {
        TokenType::Text(value)
    }
}

impl From<&str> for TokenType {
    fn from(value: &str) -> Self {
        TokenType::Text(String::from(value))
    }
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Space => write!(f, "<space>"),
            TokenType::Newline => write!(f, "<newline>"),
            TokenType::EOF => write!(f, "<EOF>"),
            
            _ => write!(f, "`{}`", self.to_string())
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TokenType::OpenParen => "(", TokenType::CloseParen => ")",
            TokenType::OpenSquare => "[", TokenType::CloseSquare => "]",
            TokenType::OpenCurly => "{", TokenType::CloseCurly => "}",
            
            TokenType::LeftArrow => "<", TokenType::RightArrow => ">",
            
            TokenType::Comma => ",", TokenType::SemiColon => ";", TokenType::Colon => ":", TokenType::Period => ".", TokenType::Hash => "#",
            TokenType::Plus => "+", TokenType::Minus => "-", TokenType::Star => "*", TokenType::Slash => "/", TokenType::Mod => "%", TokenType::Pow => "^",
            TokenType::ExclamationMark => "!", TokenType::QuestionMark => "?",
            TokenType::BackSlash => "\\", TokenType::Pipe => "|", TokenType::Equal => "=",
            
            TokenType::Quote => "'", TokenType::DoubleQuote => "\"", TokenType::BackQuote => "'",
            
            TokenType::Space => " ", TokenType::Newline => "\n",
            TokenType::EOF => "",
            
            TokenType::Text(txt) => txt
        })
    }
}