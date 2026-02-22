use crate::rwl::ast::range::Range;
use crate::rwl::ast::position::Position;
use crate::rwl::ast::token::{Token, TokenType};
use crate::rwl::error::*;
use crate::rwl::value::{PropertyPath, ThemeProperty};
use crate::shared::color::{parse_hex_color, Color};
use crate::shared::utils::{is_alpha, is_numeric};

const SPLIT_CHARS: [char; 42] = [
    // rwl
    '{','}',
    '[',']',
    ',',
    ' ',
    '\n',
    '/','\\','*',
    '\'','"','`',
    ':','#','%','=',
    // rtr
    '(',')',
    '[',']',
    '{','}',
    ',',';',':','=','.',
    '+','-','*','/','%','^',
    '\\',
    '\'','"','`',
    ' ','\n',
    '!','?'
];

pub enum CommentType {
    None,
    Multiline,
    Singleline
}

pub fn tokenise(text: String) -> Vec<Token> {
    macro_rules! add_buf {
        ($buf:expr,$tokens:expr,$pos:expr) => {
                $tokens.push(Token {
                    token_type: $buf.clone().into(),
                    range: Range {
                        start: $pos.clone() - $buf.len(),
                        end: $pos.clone()
                    }
                });
        };
    }
    
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf: String = String::new();
    
    let mut pos: Position = Position {
        ln: 1,
        col: 1,
        i: 0,
        script: text.clone()
    };
    
    let mut comment_type: CommentType = CommentType::None;
    let mut i: usize = 0;
    let vec_chars = text.chars().collect::<Vec<char>>();
    let chars = text.chars();
    for char in chars {
        if char == '\n' {
            pos.col = 0;
            pos.ln += 1;
        }
        
        match comment_type {
            CommentType::None => (),
            CommentType::Multiline => {
                if i > 0 && vec_chars[i - 1] == '*' && char == '/' {
                    comment_type = CommentType::None;
                }
                pos = pos + 1;
                i += 1;
                continue;
            }
            CommentType::Singleline => {
                if char == '\n' {
                    comment_type = CommentType::None;
                } else {
                    pos = pos + 1;
                    i += 1;
                }
                continue;
            }
        }
        
        if char == '/' && i < text.len() - 1 && vec_chars[i + 1] == '*' {
            comment_type = CommentType::Multiline;
            pos = pos + 1;
            i += 1;
            continue;
        }
        if char == '/' && i < text.len() - 1 && vec_chars[i + 1] == '/' {
            comment_type = CommentType::Singleline;
            pos = pos + 1;
            i += 1;
            continue;
        }
        
        if SPLIT_CHARS.contains(&char) {
            if !buf.is_empty() {
                add_buf!(buf, tokens, pos);
                buf = String::new();
            }
            tokens.push(Token {
                token_type: char.into(),
                range: Range {
                    start: pos.clone(),
                    end: pos.clone() + 1
                }
            });
            pos += 1;
        } else {
            buf.push(char);
            pos += 1;
        }
        
        i += 1;
    }
    
    if !buf.is_empty() {
        add_buf!(buf, tokens, pos);
    }
    
    tokens
}

/*
pub fn tokenise(src: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut buf = String::new();
    
    for char in src.chars() {
        if SPLIT_CHARS.contains(&char) {
            if !buf.is_empty() {
                tokens.push(buf);
            }
            tokens.push(char.to_string());
            buf = String::new();
        } else {
            buf += char.to_string().as_str();
        }
    }
    if !buf.is_empty() {
        tokens.push(buf);
    }
    
    tokens
}
 */

pub type AstNodeOrErr = Result<AstNode, Error>;
pub type AstValueOrErr = Result<AstValue, Error>;

pub struct Pos {
    pub ln: usize,
    pub col: usize,
    
    pub char: usize,
    pub col_char: usize,
    
    pub i: usize,
    pub i_char: usize
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Empty,
    
    Document(Vec<AstNode>),
    Block(BlockType, AstHeader, Vec<AstNode>),
    
    Element(AstValue, AstHeader)
}

#[derive(Debug, Clone)]
pub enum BlockType {
    Root,
    Frame,
    Section,
    Script,
    Button,
    
    Unknown(String)
}

#[derive(Debug, Clone)]
pub enum AstValue {
    Str(String),
    Num(f32),
    Percentage(f32),
    Color(Color),
    Property(PropertyPath)
}

#[derive(Debug, Clone)]
pub struct AstHeader {
    pub attributes: Vec<AstHeaderItem>
}
#[derive(Debug, Clone)]
pub enum AstHeaderItem {
    Pair(String, AstValue),
    Flag(String)
}

#[derive(Debug)]
pub struct Parser {
    pub pointer: usize,
    pub tokens: Vec<Token>
}
impl Parser {
    fn at_end(&mut self) -> bool {
        self.pointer >= self.tokens.len()
    }
    fn get_next_start(&self) -> Position {
        self.tokens[self.pointer].range.start.clone()
    }
    fn get_last_end(&self) -> Position {
        self.tokens[self.pointer - 1].range.end.clone()
    }
    fn get_err_pos(&self) -> ErrPosition {
        ErrPosition
    }
    
    fn peek(&self) -> Token {
        self.peek_amount(0)
    }
    fn peek_amount(&self, i: usize) -> Token {
        if let Some(tkn) = self.tokens.get(self.pointer) {
            tkn.clone()
        } else {
            let last = self.tokens.last().unwrap();
            Token {
                token_type: TokenType::EOF,
                range: Range {
                    start: last.range.end.clone() + 1,
                    end: last.range.end.clone() + 2,
                }
            }
        }
    }
    fn consume(&mut self) -> Token {
        let tkn = self.tokens[self.pointer].clone();
        self.pointer += 1;
        tkn
    }
    fn consume_whitespace(&mut self) {
        while matches!(
            self.peek().token_type,
            TokenType::Space | TokenType::Newline
        ) {
            self.consume();
        }
    }
    fn expect<T>(&mut self, other: T) -> Result<Token, Error>
    where
        Token: PartialEq<T>, TokenType: From<T>
    {
        let tkn = self.consume();
        if tkn != other {
            return Err(Error::Expected {
                wanted: vec![other.into()],
                got: tkn.clone(),
                range: tkn.range
            });
        }
        Ok(tkn)
    }
    fn expect_multiple(&mut self, other: Vec<TokenType>) -> Result<Token, Error>
    {
        let tkn = self.consume();
        if !other.contains(&tkn.token_type) {
            return Err(Error::Expected {
                wanted: other,
                got: tkn.clone(),
                range: tkn.range
            });
        }
        Ok(tkn)
    }
    fn expect_text(&mut self) -> Result<String, Error> {
        let tkn = self.consume();
        
        match &tkn.token_type {
            TokenType::Text(txt) => Ok(txt.clone()),
            
            _ => Err(Error::ExpectedText {
                got: tkn.clone(),
                range: tkn.range
            })
        }
    }
    
    // top level
    pub fn parse(&mut self) -> AstNodeOrErr {
        let mut statements: Vec<AstNode> = Vec::new();
        
        while !self.at_end() {
            let statement = self.statement();
            
            if statement.is_ok() {
                statements.push(statement?);
            } else {
                return statement;
            }
        }
        
        Ok(
            AstNode::Document(statements)
        )
    }
    
    // statement
    fn parse_block(&mut self) -> Result<Vec<AstNode>, Error> {
        let mut statements: Vec<AstNode> = Vec::new();
        
        self.consume_whitespace();
        self.expect(TokenType::OpenCurly)?;
        self.consume_whitespace();
        
        while self.peek() != TokenType::CloseCurly && !self.at_end() {
            let statement = self.statement();
            self.consume_whitespace();
            
            match statement {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(err) => {
                    return Err(err);
                }
            }
            
            if self.peek() == TokenType::CloseCurly {
                break;
            }
            self.expect(TokenType::Comma)?;
            self.consume_whitespace();
        }
        
        self.consume_whitespace();
        self.expect(TokenType::CloseCurly)?;
        
        Ok(statements)
    }
    
    fn statement(&mut self) -> AstNodeOrErr {
        self.consume_whitespace();
        
        let start = self.get_next_start();
        let tkn = self.peek();
        
        // void elements
        
        // block
        if let TokenType::Text(text) = tkn.token_type && is_alpha(&*text) {
            return self.block_statement()
        }
        
        // element
        let value = self.value()?;
        self.consume_whitespace();
        let header = self.header()?;
        
        Ok(
            AstNode::Element(
                value,
                header
            )
        )
    }
    fn block_statement(&mut self) -> AstNodeOrErr {
        let start = self.get_next_start();
        let key = self.consume().to_string();
        
        // handle scripts
        
        self.consume_whitespace();
        let header = self.header()?;
        
        let body = self.parse_block()?;
        
        Ok(
            AstNode::Block(
                match key.as_str() {
                    "root" => BlockType::Root,
                    "frame" => BlockType::Frame,
                    "section" => BlockType::Section,
                    "button" => BlockType::Button,
                    
                    _ => BlockType::Unknown(key)
                },
                header,
                body
            )
        )
    }
    
    // header stuff
    fn header(&mut self) -> Result<AstHeader, Error> {
        let mut attributes = Vec::new();
        
        if self.peek() == TokenType::OpenSquare {
            self.consume();
            self.consume_whitespace();
            while self.peek() != TokenType::CloseSquare && !self.at_end() {
                attributes.push(self.header_item()?);
                
                self.consume_whitespace();
                if self.peek() != TokenType::CloseSquare {
                    self.expect_multiple(vec![
                        TokenType::Comma,
                        TokenType::CloseSquare
                    ])?;
                }
                self.consume_whitespace();
            }
            self.expect(TokenType::CloseSquare)?;
        }
        
        Ok(AstHeader {
            attributes
        })
    }
    fn header_item(&mut self) -> Result<AstHeaderItem, Error> {
        self.consume_whitespace();
        
        let tkn = self.consume().to_string();
        
        self.consume_whitespace();
        if self.peek() == TokenType::Equal {
            if !is_alpha(tkn.as_str()) {
                return Err(Error::InvalidAttributeKey);
            }
            self.consume();
            self.consume_whitespace();
            let value = self.value()?;
            return Ok(AstHeaderItem::Pair(tkn, value))
        }
        
        if is_alpha(tkn.as_str()) {
            return Ok(AstHeaderItem::Flag(tkn));
        }
        
        Err(
            Error::UnexpectedToken {
                token: self.peek(),
                range: self.peek().range
            }
        )
    }
    
    // values
    fn value(&mut self) -> AstValueOrErr {
        if matches!(
            self.peek().token_type,
            TokenType::Quote |
            TokenType::DoubleQuote |
            TokenType::BackQuote
        ) {
            return self.str()
        }
        if let TokenType::Text(text) = self.peek().token_type && is_numeric(&*text) {
            return self.num();
        }
        if self.peek() == TokenType::Hash {
            return self.color();
        }
        if let TokenType::Text(text) = self.peek().token_type && is_alpha(&*text) {
            return self.property();
        }
        
        Err(
            Error::UnexpectedToken {
                token: self.peek(),
                range: self.peek().range
            }
        )
    }
    fn str(&mut self) -> AstValueOrErr {
        let quote = self.expect_multiple(vec![
            TokenType::Quote,
            TokenType::DoubleQuote,
            TokenType::BackQuote
        ])?;
        let mut content = String::new();
        
        while self.peek() != quote && !self.at_end() {
            let tkn = self.peek();
            
            if tkn == TokenType::BackSlash {
                self.consume();
                let tkn = self.consume().to_string();
                let char = &tkn[0..1];
                
                content = format!("{}{}{}", content, match char {
                    "n" => "\n",
                    
                    _ => char
                }, &tkn[1..tkn.len()]);
            } else {
                let tkn = self.consume();
                content = format!("{content}{tkn}");
            }
        }
        
        self.expect(quote.token_type)?;
        
        Ok(AstValue::Str(content))
    }
    fn num(&mut self) -> AstValueOrErr {
        // TODO: support decimals?
        
        if !is_numeric(&*self.peek().to_string()) {
            return Err(
                Error::UnexpectedToken {
                    token: self.peek(),
                    range: self.peek().range
                }
            );
        }
        
        let value: Result<i32, _> = self.consume().to_string().parse::<i32>();
        if let Err(_e) = value {
            return Err(Error::CouldntParseNum);
        }
        let value = value.unwrap();
        
        if self.peek() == "%" {
            self.consume();
            Ok(AstValue::Percentage(value as f32))
        } else {
            Ok(AstValue::Num(value as f32))
        }
    }
    fn color(&mut self) -> AstValueOrErr {
        self.expect(TokenType::Hash)?;
        let value = self.consume().to_string();
        if ![3,6].contains(&value.len()) {
            return Err(Error::InvalidHexLength);
        }
        Ok(AstValue::Color(
            // TODO: handle this better? :P
            parse_hex_color(&*value).unwrap()
        ))
    }
    fn property(&mut self) -> AstValueOrErr {
        let source = self.expect_text()?;
        
        self.consume_whitespace();
        self.expect(TokenType::Colon)?;
        self.consume_whitespace();
        
        let name = self.expect_text()?;
        
        Ok(AstValue::Property(
            match source.as_str() {
                "theme" => {
                    PropertyPath::Theme(match name.as_str() {
                        "back" => ThemeProperty::Back,
                        "prim" => ThemeProperty::Prim,
                        "seco" => ThemeProperty::Seco,
                        "tert" => ThemeProperty::Tert,
                        "text" => ThemeProperty::Text,
                        "accent" => ThemeProperty::Accent,
                        
                        _ => {
                            return Err(Error::UnknownProperty {
                                source,
                                property: name
                            });
                        }
                    })
                },
                
                _ => {
                    return Err(Error::UnknownPropertySource {
                        source
                    });
                }
            }
        ))
    }
}
