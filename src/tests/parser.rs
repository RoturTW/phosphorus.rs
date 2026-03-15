use crate::shared::position::Position;
use crate::shared::range::Range;
use crate::shared::token::{Token, TokenType};
use crate::shared::utils::{is_alpha, is_numeric};
use crate::tests::error::Error;

#[derive(Debug)]
pub struct Test {
    pub name: String,
    pub desc: Option<String>,
    pub code: Code,
    pub result: Vec<(Vec<String>,Vec<String>)>
}

#[derive(Debug, Clone)]
pub enum Code {
    Expr(Vec<CodePart>),
    Program(Vec<CodePart>)
}

#[derive(Debug, Clone)]
pub enum CodePart {
    Str(String),
    Var(usize)
}

type Maybe<T> = Result<T, Error>;

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
    
    fn peek_amount(&self, amount: isize) -> Token {
        if let Some(tkn) = self.tokens.get((self.pointer.cast_signed() + (amount - 1)) as usize) {
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
    fn peek(&self) -> Token {
        self.peek_amount(1)
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
    fn expect<T>(&mut self, other: T) -> Maybe<Token>
    where
        Token: PartialEq<T>, TokenType: From<T>
    {
        let tkn = self.consume();
        if tkn != other {
            return Err(Error::Expected {
                wanted: vec![other.into()],
                got: Box::new(tkn.clone()),
                range: Box::new(tkn.range)
            });
        }
        Ok(tkn)
    }
    fn expect_multiple(&mut self, other: Vec<TokenType>) -> Maybe<Token>
    {
        let tkn = self.consume();
        if !other.contains(&tkn.token_type) {
            return Err(Error::Expected {
                wanted: other,
                got: Box::new(tkn.clone()),
                range: Box::new(tkn.range)
            });
        }
        Ok(tkn)
    }
    fn expect_text(&mut self) -> Maybe<String> {
        let tkn = self.consume();
        
        if let TokenType::Text(txt) = &tkn.token_type && is_alpha(txt) {
            Ok(txt.clone())
        } else {
            Err(Error::ExpectedText {
                got: Box::new(tkn.clone()),
                range: Box::new(tkn.range)
            })
        }
    }
    fn expect_num(&mut self) -> Maybe<String> {
        let tkn = self.consume();
        
        if let TokenType::Text(txt) = &tkn.token_type && is_numeric(txt) {
            Ok(txt.clone())
        } else {
            Err(Error::ExpectedNum {
                got: Box::new(tkn.clone()),
                range: Box::new(tkn.range)
            })
        }
    }
    
    // top level
    #[allow(clippy::too_many_lines)]
    pub fn parse(&mut self) -> Maybe<Test> {
        let mut name: Option<String> = None;
        let mut desc: Option<String> = None;
        let mut code: Option<Code> = None;
        let mut result: Vec<(Vec<String>, Vec<String>)> = Vec::new();
        
        self.consume_whitespace();
        while !self.at_end() {
            self.consume_whitespace();
            
            if self.peek() == "name" || self.peek() == "desc" {
                let tkn = self.consume();
                self.consume_whitespace();
                self.expect(TokenType::Colon)?;
                self.consume_whitespace();
                
                let mut txt = String::new();
                while !(self.peek() == TokenType::Newline || self.at_end()) {
                    txt = format!("{txt}{}", self.consume());
                }
                
                match tkn.token_type.to_string().as_str() {
                    "name" => {
                        name = Some(txt);
                    }
                    "desc" => {
                        desc = Some(txt);
                    }
                    
                    _ => panic!()
                }
                continue;
            }
            
            if self.peek() == "code" {
                self.consume();
                self.consume_whitespace();
                self.expect(TokenType::OpenParen)?;
                self.consume_whitespace();
                let code_type_start = self.get_next_start();
                let code_type = self.expect_text()?;
                let code_type_end = self.get_last_end();
                self.consume_whitespace();
                self.expect(TokenType::CloseParen)?;
                self.consume_whitespace();
                self.expect(TokenType::Colon)?;
                while self.peek() == TokenType::Space { self.consume(); }
                self.expect(TokenType::Newline)?;
                
                let data = self.parse_code()?;
                
                code = Some(match code_type.as_str() {
                    "expr" => {
                        Code::Expr(data)
                    }
                    "program" => {
                        Code::Program(data)
                    }
                    
                    _ => {
                        return Err(Error::InvalidCodeType {
                            range: Box::new(Range { start: code_type_start, end: code_type_end })
                        })
                    }
                });
                
                continue;
            }
            
            if self.peek() == "result" {
                self.consume();
                self.consume_whitespace();
                let mut vars = Vec::new();
                if self.peek() == TokenType::OpenParen {
                    self.consume();
                    self.consume_whitespace();
                    while !(self.peek() == TokenType::CloseParen || self.at_end()) {
                        let mut txt = String::new();
                        while !(self.peek() == TokenType::CloseParen || self.peek() == TokenType::Comma || self.at_end()) {
                            txt = format!("{txt}{}", self.consume());
                        }
                        vars.push(txt);
                        self.consume_whitespace();
                        if self.peek() != TokenType::CloseParen {
                            self.expect_multiple(vec![
                                TokenType::Comma,
                                TokenType::CloseParen
                            ])?;
                        }
                        self.consume_whitespace();
                    }
                    self.expect(TokenType::CloseParen)?;
                }
                self.consume_whitespace();
                self.expect(TokenType::Colon)?;
                while self.peek() == TokenType::Space { self.consume(); }
                self.expect(TokenType::Newline)?;
                
                let data = self.parse_lines()?;
                
                result.push((data, vars));
                
                continue;
            }
            
            return Err(
                Error::UnexpectedToken {
                    token: Box::new(self.peek()),
                    range: Box::new(self.peek().range)
                }
            )
        }
        
        if name.is_none() {
            return Err(Error::TestNeedsName);
        }
        if code.is_none() {
            return Err(Error::TestNeedsCode);
        }
        
        let test = Test {
            name: name.unwrap(),
            desc,
            code: code.unwrap(),
            result,
        };
        
        Ok(test)
    }
    
    fn parse_lines(&mut self) -> Maybe<Vec<String>> {
        let mut indent: usize = 0;
        while self.peek() == TokenType::Space {
            indent += 1;
            self.consume();
        }
        if indent == 0 {
            return Err(Error::TextMustHaveIndent)
        }
        
        let mut lines: Vec<String> = Vec::new();
        
        // parse first line manually
        let mut line = String::new();
        while !(self.peek() == TokenType::Newline || self.at_end()) {
            line = format!("{line}{}", self.consume());
        }
        if !self.at_end() {
            self.expect(TokenType::Newline)?;
        }
        lines.push(line);
        
        while self.peek() == TokenType::Space {
            for _ in 0..indent {
                self.expect(TokenType::Space)?;
            }
            
            let mut line = String::new();
            while !(self.peek() == TokenType::Newline || self.at_end()) {
                line = format!("{line}{}", self.consume());
            }
            if !self.at_end() {
                self.expect(TokenType::Newline)?;
            }
            lines.push(line);
        }
        
        Ok(lines)
    }
    
    fn parse_code(&mut self) -> Maybe<Vec<CodePart>> {
        let mut indent: usize = 0;
        while self.peek() == TokenType::Space {
            indent += 1;
            self.consume();
        }
        if indent == 0 {
            return Err(Error::TextMustHaveIndent)
        }
        
        let mut lines: Vec<CodePart> = Vec::new();
        
        // parse first line manually
        let mut line = Vec::new();
        while !(self.peek() == TokenType::Newline || self.at_end()) {
            line.push(self.parse_code_part()?);
        }
        if !self.at_end() {
            self.expect(TokenType::Newline)?;
            line.push(CodePart::Str(String::from("\n")));
        }
        lines.append(&mut line);
        
        while self.peek() == TokenType::Space {
            for _ in 0..indent {
                self.expect(TokenType::Space)?;
            }
            
            let mut line = Vec::new();
            while !(self.peek() == TokenType::Newline || self.at_end()) {
                line.push(self.parse_code_part()?);
            }
            if !self.at_end() {
                self.expect(TokenType::Newline)?;
                line.push(CodePart::Str(String::from("\n")));
            }
            lines.append(&mut line);
        }
        
        Ok(lines)
    }
    fn parse_code_part(&mut self) -> Maybe<CodePart> {
        if self.peek() == TokenType::Dollar {
            self.consume();
            let idx: usize = self.expect_num()?.parse().unwrap_or(0);
            return Ok(CodePart::Var(idx));
        }
        
        Ok(CodePart::Str(self.consume().to_string()))
    }
}
