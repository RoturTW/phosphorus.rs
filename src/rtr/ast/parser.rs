use crate::rtr::ast::node::{AssignmentOp, AstConditionalType, AstExpression, AstProgram, AstStatement, AstTopLevelStatement, BinaryOp, EventTarget, PropertyKey, Target, UnaryOp};
use crate::rtr::error::Error;
use crate::shared::color::parse_hex_color;
use crate::shared::range::Range;
use crate::shared::position::Position;
use crate::shared::token::{Token, TokenType};
use crate::shared::utils::{is_alpha, is_numeric};

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
            Err(Error::ExpectedText {
                got: Box::new(tkn.clone()),
                range: Box::new(tkn.range)
            })
        }
    }
    
    // top level
    pub fn parse(&mut self) -> Maybe<AstProgram> {
        let mut events: Vec<AstTopLevelStatement> = Vec::new();
        
        while !self.at_end() {
            let statement = self.top_level_statement()?;
            
            match &statement {
                AstTopLevelStatement::Event { .. } => {
                    events.push(statement);
                }
            }
            self.consume_whitespace();
        }
        
        Ok(
            AstProgram {
                events
            }
        )
    }
    
    // top level statement
    fn top_level_statement(&mut self) -> Maybe<AstTopLevelStatement> {
        self.consume_whitespace();
        
        if self.peek() == "event" {
            return self.event();
        }
        
        // TODO: top level decls
        Err(
            Error::UnexpectedToken {
                token: Box::new(self.peek()),
                range: Box::new(self.peek().range)
            }
        )
    }
    
    fn event(&mut self) -> Maybe<AstTopLevelStatement> {
        self.expect("event")?;
        self.consume_whitespace();
        self.expect(TokenType::OpenParen)?;
        let target = self.event_target()?;
        self.consume_whitespace();
        self.expect(TokenType::CloseParen)?;
        self.consume_whitespace();
        let body = self.block()?;
        
        Ok(AstTopLevelStatement::Event {
            body,
            target
        })
    }
    
    // event targets
    fn event_target(&mut self) -> Maybe<EventTarget> {
        let pos = self.pointer;
        
        let target = self.target()?;
        
        self.consume_whitespace();
        
        if self.peek() == TokenType::Colon {
            self.consume();
            self.consume_whitespace();
            let event_name = self.expect_text()?;
            return Ok(EventTarget::Property {
                target,
                event_name
            });
        }
        
        self.pointer = pos;
        
        Ok(EventTarget::Global {
            name: self.expect_text()?
        })
    }
    
    // targets
    fn target(&mut self) -> Maybe<Target> {
        if self.peek() == TokenType::Star {
            return Ok(Target::Any);
        }
        
        if self.peek() == TokenType::Hash {
            self.consume();
            return Ok(Target::Id(self.expect_text()?));
        }
        
        Ok(Target::Element(self.expect_text()?))
    }
    
    // statement
    fn block(&mut self) -> Maybe<Vec<AstStatement>> {
        let mut statements: Vec<AstStatement> = Vec::new();
        
        self.consume_whitespace();
        self.expect(TokenType::OpenCurly)?;
        self.consume_whitespace();
        
        while !(self.peek() == TokenType::CloseCurly || self.at_end()) {
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
        }
        
        self.expect(TokenType::CloseCurly)?;
        
        Ok(statements)
    }
    
    fn statement(&mut self) -> Maybe<AstStatement> {
        self.consume_whitespace();
        
        let tkn = self.peek();
        
        if tkn == TokenType::OpenCurly {
            let body = self.block()?;
            return Ok(AstStatement::Block {
                body
            });
        }
        if let TokenType::Text(name) = tkn.token_type {
            if name.as_str() == "if" {
                return self.branch();
            }
            if name.as_str() == "while" || name.as_str() == "until" {
                return self.conditional_loop();
            }
            if name.as_str() == "repeat" {
                return self.repeat();
            }
            if name.as_str() == "for" {
                return self.for_loop();
            }
        }
        
        let expr = self.expression()?;
        self.expect(TokenType::SemiColon)?;
        Ok(AstStatement::Expression(expr))
    }
    
    fn branch(&mut self) -> Maybe<AstStatement> {
        let start = self.get_next_start();
        
        self.expect("if")?;
        self.consume_whitespace();
        self.expect(TokenType::OpenParen)?;
        self.consume_whitespace();
        let cond = self.expression()?;
        self.consume_whitespace();
        self.expect(TokenType::CloseParen)?;
        let body = Box::new(self.statement()?);
        
        let mut elifs = Vec::new();
        
        self.consume_whitespace();
        while self.peek() == "elif" {
            self.consume();
            self.consume_whitespace();
            self.expect(TokenType::OpenParen)?;
            self.consume_whitespace();
            let cond = self.expression()?;
            self.consume_whitespace();
            self.expect(TokenType::CloseParen)?;
            self.consume_whitespace();
            let body = self.statement()?;
            
            elifs.push((cond, body));
            self.consume_whitespace();
        }
        
        let mut else_body = None;
        if self.peek() == "else" {
            self.consume();
            self.consume_whitespace();
            else_body = Some(Box::new(self.statement()?));
        }
        
        Ok(AstStatement::Branch {
            cond,
            body,
            elifs,
            else_body,
            range: Range { start, end: self.get_last_end() }
        })
    }
    fn conditional_loop(&mut self) -> Maybe<AstStatement> {
        let start = self.get_next_start();
        
        let token = self.expect_multiple(vec!["while".into(), "until".into()])?;
        let TokenType::Text(token_text) = token.token_type else { panic!() };
        
        let kind = match token_text.as_str() {
            "while" => AstConditionalType::While,
            "until" => AstConditionalType::Until,
            _ => panic!()
        };
        
        self.consume_whitespace();
        
        self.expect(TokenType::OpenParen)?;
        self.consume_whitespace();
        let cond = self.expression()?;
        self.consume_whitespace();
        self.expect(TokenType::CloseParen)?;
        self.consume_whitespace();
        let body = self.statement()?;
        
        Ok(AstStatement::ConditionalLoop {
            kind,
            cond,
            body: Box::new(body),
            range: Range { start, end: self.get_last_end() }
        })
    }
    fn repeat(&mut self) -> Maybe<AstStatement> {
        let start = self.get_next_start();
        
        self.expect("repeat")?;
        self.consume_whitespace();
        
        self.expect(TokenType::OpenParen)?;
        self.consume_whitespace();
        let amount = self.expression()?;
        self.consume_whitespace();
        self.expect(TokenType::CloseParen)?;
        self.consume_whitespace();
        let body = self.statement()?;
        
        Ok(AstStatement::Repeat {
            amount,
            body: Box::new(body),
            range: Range { start, end: self.get_last_end() }
        })
    }
    fn for_loop(&mut self) -> Maybe<AstStatement> {
        let start = self.get_next_start();
        
        self.expect("for")?;
        self.consume_whitespace();
        
        self.expect(TokenType::OpenParen)?;
        self.consume_whitespace();
        let var = self.expect_text()?;
        self.consume_whitespace();
        self.expect(TokenType::Comma)?;
        self.consume_whitespace();
        let iterator = self.expression()?;
        self.consume_whitespace();
        self.expect(TokenType::CloseParen)?;
        self.consume_whitespace();
        let body = self.statement()?;
        
        Ok(AstStatement::For {
            var,
            iterator,
            body: Box::new(body),
            range: Range { start, end: self.get_last_end() }
        })
    }
    
    // expression
    fn expression(&mut self) -> Maybe<AstExpression> {
        self.declare()
    }
    
    fn declare(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        if let TokenType::Text(t) = self.peek().token_type && is_alpha(&t) {
            let ptr = self.pointer;
            let name = self.expect_text()?;
            self.consume_whitespace();
            if self.peek() == TokenType::Colon && self.peek_amount(2) == TokenType::Equal {
                self.consume();
                self.consume();
                self.consume_whitespace();
                
                let val = self.expression()?;
                
                return Ok(AstExpression::Declare {
                    name,
                    value: Box::new(val),
                    range: Range { start, end: self.get_last_end() }
                });
            }
            self.pointer = ptr;
        }
        
        self.assignment()
    }
    fn assignment(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        let expr = self.equality()?;
        self.consume_whitespace();
        
        let mut op = None;
        
        if self.peek() == TokenType::QuestionMark
        && self.peek_amount(2) == TokenType::QuestionMark
        && self.peek_amount(3) == TokenType::Equal
        {
            op = Some(AssignmentOp::NullishCoalescence);
        }
        
        if self.peek_amount(2) == TokenType::Equal && matches!(self.peek().token_type,
            TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash | TokenType::Mod | TokenType::Pow
        ) {
            op = Some(match self.peek().token_type {
                TokenType::Plus => AssignmentOp::Add,
                TokenType::Minus => AssignmentOp::Sub,
                TokenType::Star => AssignmentOp::Mul,
                TokenType::Slash => AssignmentOp::Div,
                TokenType::Mod => AssignmentOp::Mod,
                TokenType::Pow => AssignmentOp::Pow,
                
                _ => panic!()
            });
        }
        
        if self.peek() == TokenType::Equal {
            op = Some(AssignmentOp::Default);
        }
        
        if let Some(op) = op {
            let op_len: usize = match op {
                AssignmentOp::Default => 1,
                AssignmentOp::NullishCoalescence => 3,
                _ => 2
            };
            
            for _i in 0..op_len {
                self.consume();
            }
            
            self.consume_whitespace();
            
            let value = self.expression()?;
            
            return Ok(AstExpression::Assignment {
                op,
                target: Box::new(expr),
                value: Box::new(value),
                range: Range { start, end: self.get_last_end() }
            })
        }
        
        Ok(expr)
    }
    fn equality(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        let mut expr = self.comparison()?;
        
        self.consume_whitespace();
        
        while [TokenType::Equal, TokenType::ExclamationMark].contains(&self.peek().token_type) && self.peek_amount(2) == TokenType::Equal {
            let token = self.consume();
            self.consume();
            
            let op = match token.token_type {
                TokenType::Equal => BinaryOp::Eql,
                TokenType::ExclamationMark => BinaryOp::NotEql,
                _ => panic!()
            };
            
            self.consume_whitespace();
            
            let right = self.comparison()?;
            
            expr = AstExpression::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                op,
                range: Range { start: start.clone(), end: self.get_last_end() }
            }
        }
        
        Ok(expr)
    }
    fn comparison(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        let mut expr = self.term()?;
        
        self.consume_whitespace();
        
        while self.peek().token_type == TokenType::LeftArrow || self.peek().token_type == TokenType::RightArrow {
            let token = self.consume();
            
            let equal = self.peek() == TokenType::Equal;
            if equal {
                self.consume();
            }
            
            let op = match (token.token_type, equal) {
                (TokenType::LeftArrow, false) => BinaryOp::Smaller,
                (TokenType::LeftArrow, true) => BinaryOp::SmallerEql,
                (TokenType::RightArrow, false) => BinaryOp::Bigger,
                (TokenType::RightArrow, true) => BinaryOp::BiggerEql,
                _ => panic!()
            };
            
            self.consume_whitespace();
            
            let right = self.term()?;
            
            expr = AstExpression::Binary {
                left: Box::new(expr),
                right: Box::new(right),
                op,
                range: Range { start: start.clone(), end: self.get_last_end() }
            }
        }
        
        Ok(expr)
    }
    fn term(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        let mut expr = self.factor()?;
        
        self.consume_whitespace();
        
        while matches!(self.peek().token_type, TokenType::Plus | TokenType::Minus) && self.peek_amount(2) != TokenType::Equal {
            let op = match self.consume().token_type {
                TokenType::Plus =>
                    BinaryOp::Add,
                TokenType::Minus =>
                    BinaryOp::Sub,
                _ => panic!()
            };
            
            self.consume_whitespace();
            
            expr = AstExpression::Binary {
                left: Box::new(expr),
                right: Box::new(self.factor()?),
                op,
                range: Range { start: start.clone(), end: self.get_last_end() }
            }
        }
        
        Ok(expr)
    }
    fn factor(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        let mut expr = self.other_binary()?;
        
        self.consume_whitespace();
        
        while matches!(self.peek().token_type, TokenType::Star | TokenType::Slash) && self.peek_amount(2) != TokenType::Equal {
            let op = match self.consume().token_type {
                TokenType::Star =>
                    BinaryOp::Mul,
                TokenType::Slash =>
                    BinaryOp::Div,
                _ => panic!()
            };
            
            self.consume_whitespace();
            
            expr = AstExpression::Binary {
                left: Box::new(expr),
                right: Box::new(self.other_binary()?),
                op,
                range: Range { start: start.clone(), end: self.get_last_end() }
            }
        }
        
        Ok(expr)
    }
    fn other_binary(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        let mut expr = self.coalescence()?;
        
        self.consume_whitespace();
        
        while matches!(self.peek().token_type, TokenType::Mod | TokenType::Pow) && self.peek_amount(2) != TokenType::Equal {
            let op = match self.consume().token_type {
                TokenType::Mod =>
                    BinaryOp::Mod,
                TokenType::Pow =>
                    BinaryOp::Pow,
                _ => panic!()
            };
            
            self.consume_whitespace();
            
            expr = AstExpression::Binary {
                left: Box::new(expr),
                right: Box::new(self.coalescence()?),
                op,
                range: Range { start: start.clone(), end: self.get_last_end() }
            }
        }
        
        Ok(expr)
    }
    fn coalescence(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        let mut expr = self.unary()?;
        
        self.consume_whitespace();
        
        while self.peek() == TokenType::QuestionMark && self.peek_amount(2) == TokenType::QuestionMark && self.peek_amount(3) != TokenType::Equal {
            self.consume();
            self.consume();
            
            self.consume_whitespace();
            
            expr = AstExpression::Binary {
                left: Box::new(expr),
                right: Box::new(self.coalescence()?),
                op: BinaryOp::NullishCoalescence,
                range: Range { start: start.clone(), end: self.get_last_end() }
            }
        }
        
        Ok(expr)
    }
    fn unary(&mut self) -> Maybe<AstExpression> {
        if self.peek_amount(2) != TokenType::Equal
            && matches!(self.peek().token_type, TokenType::Minus | TokenType::Plus | TokenType::ExclamationMark | TokenType::QuestionMark) {
                let start = self.get_next_start();
                return Ok(AstExpression::Unary {
                    op: match self.consume().token_type {
                        TokenType::Minus =>
                            UnaryOp::Minus,
                        TokenType::Plus =>
                            UnaryOp::Number,
                        TokenType::ExclamationMark =>
                            UnaryOp::Invert,
                        TokenType::QuestionMark =>
                            UnaryOp::Boolify,
                        _ => panic!()
                    },
                    value: Box::new(self.unary()?),
                    range: Range { start, end: self.get_last_end() }
                })
            }
        
        self.call()
    }
    fn call(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        
        if let TokenType::Text(txt) = self.peek().token_type
            && txt.as_str() == "call" {
                self.consume();
                self.consume_whitespace();
                self.expect(TokenType::OpenParen)?;
                self.consume_whitespace();
                let target = self.expect_text()?;
                self.consume_whitespace();
                self.expect(TokenType::CloseParen)?;
                return Ok(AstExpression::CallEvent {
                    target
                })
            }
        
        let mut expr = self.func()?;
        self.consume_whitespace();
        if self.peek() == TokenType::OpenParen {
            self.consume();
            
            let mut args = Vec::new();
            
            while !(self.peek() == TokenType::CloseParen || self.at_end()) {
                let expression = self.expression();
                self.consume_whitespace();
                
                match expression {
                    Ok(expression) => {
                        args.push(expression);
                    }
                    Err(err) => {
                        return Err(err);
                    }
                }
                
                if self.peek() == TokenType::CloseParen {
                    break;
                }
                self.expect(TokenType::Comma)?;
                self.consume_whitespace();
            }
            self.expect(TokenType::CloseParen)?;
            
            expr = AstExpression::Call {
                args,
                func: Box::new(expr),
                range: Range { start, end: self.get_last_end() }
            };
        }
        
        Ok(expr)
    }
    fn func(&mut self) -> Maybe<AstExpression> {
        let start_ptr = self.pointer;
        let mut is = false;
        if self.peek() == TokenType::OpenParen {
            let mut depth = 0;
            while !self.at_end() {
                let tkn = self.consume();
                if tkn == TokenType::OpenParen {
                    depth += 1;
                }
                if tkn == TokenType::CloseParen {
                    depth -= 1;
                    if depth == 0 {
                        is = true;
                        break;
                    }
                }
            }
        }
        self.consume_whitespace();
        if is && self.peek() == TokenType::Tilde {
            self.pointer = start_ptr;
            
            let start = self.get_next_start();
            
            let mut params = Vec::new();
            
            self.expect(TokenType::OpenParen)?;
            while !(self.peek() == TokenType::CloseParen || self.at_end()) {
                let name = self.expect_text()?;
                params.push(name);
                self.consume_whitespace();
                
                if self.peek() != TokenType::CloseParen {
                    self.expect_multiple(vec![TokenType::Comma, TokenType::CloseParen])?;
                }
            }
            self.expect(TokenType::CloseParen)?;
            
            self.consume_whitespace();
            self.expect(TokenType::Tilde)?;
            self.consume_whitespace();
            
            let body = self.statement()?;
            
            return Ok(AstExpression::Func {
                params,
                body: Box::new(body),
                range: Range { start, end: self.get_last_end() }
            })
        }
        
        self.pointer = start_ptr;
        
        self.property()
    }
    fn property(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        let mut expr = self.primary()?;
        
        self.consume_whitespace();
        
        while self.peek() == TokenType::Period || self.peek() == TokenType::OpenSquare {
            let key = match self.consume().token_type {
                TokenType::Period => {
                    PropertyKey::Str(self.expect_text()?)
                }
                TokenType::OpenSquare => {
                    let expr = self.expression()?;
                    self.expect(TokenType::CloseSquare)?;
                    PropertyKey::Expr(Box::new(expr))
                }
                
                _ => panic!()
            };
            
            expr = AstExpression::Property {
                obj: Box::new(expr),
                key,
                range: Range { start: start.clone(), end: self.get_last_end() }
            };
        }
        
        Ok(expr)
    }
    fn primary(&mut self) -> Maybe<AstExpression> {
        if self.peek() == TokenType::OpenParen {
            self.consume();
            let expr = self.expression()?;
            self.expect(TokenType::CloseParen)?;
            return Ok(expr);
        }
        if self.peek() == TokenType::OpenSquare {
            return self.arr();
        }
        if self.peek() == TokenType::OpenCurly {
            return self.obj();
        }
        if self.peek() == TokenType::Hash {
            return self.color();
        }
        if [TokenType::Quote, TokenType::DoubleQuote, TokenType::BackQuote].contains(&self.peek().token_type) {
            return self.str();
        }
        if let TokenType::Text(name) = self.peek().token_type {
            if is_alpha(&name) {
                let range = self.consume().range;
                return Ok(AstExpression::Variable { name, range });
            } else if is_numeric(&name) {
                return self.num();
            }
        }
        
        Err(
            Error::UnexpectedToken {
                token: Box::new(self.peek()),
                range: Box::new(self.peek().range)
            }
        )
    }
    fn arr(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        
        self.expect(TokenType::OpenSquare)?;
        self.consume_whitespace();
        
        let mut items = Vec::new();
        while !(self.peek() == TokenType::CloseSquare || self.at_end()) {
            items.push(self.expression()?);
            
            if self.peek() != TokenType::CloseSquare {
                self.expect_multiple(vec![
                    TokenType::Comma,
                    TokenType::CloseSquare
                ])?;
            }
            
            self.consume_whitespace();
        }
        
        self.consume_whitespace();
        self.expect(TokenType::CloseSquare)?;
        
        Ok(AstExpression::Array {
            items,
            range: Range { start, end: self.get_last_end() }
        })
    }
    fn obj(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        
        self.expect(TokenType::OpenCurly)?;
        self.consume_whitespace();
        
        let mut pairs = Vec::new();
        while !(self.peek() == TokenType::CloseCurly || self.at_end()) {
            let name = self.expect_text()?;
            self.consume_whitespace();
            self.expect(TokenType::Colon)?;
            self.consume_whitespace();
            pairs.push((name, self.expression()?));
            
            if self.peek() != TokenType::CloseCurly {
                self.expect_multiple(vec![
                    TokenType::Comma,
                    TokenType::CloseCurly
                ])?;
            }
            
            self.consume_whitespace();
        }
        
        self.consume_whitespace();
        self.expect(TokenType::CloseCurly)?;
        
        Ok(AstExpression::Object {
            pairs,
            range: Range { start, end: self.get_last_end() }
        })
    }
    fn color(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        self.expect(TokenType::Hash)?;
        let txt = self.expect_text()?;
        
        let data_start = self.get_next_start();
        let content = parse_hex_color(&txt);
        if let Err(_err) = content {
            return Err(Error::InvalidColor {
                range: Range { start: data_start, end: self.get_last_end() }
            })
        }
        
        Ok(AstExpression::Color {
            content: content.unwrap(),
            range: Range { start, end: self.get_last_end() }
        })
    }
    fn str(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        
        let quote = self.expect_multiple(vec![
            TokenType::Quote,
            TokenType::DoubleQuote,
            TokenType::BackQuote
        ])?.token_type;
        let mut content = String::new();
        while !(self.peek() == quote || self.at_end()) {
            content = format!("{}{}", content, self.consume());
        }
        self.expect(quote)?;
        
        Ok(AstExpression::String {
            content,
            range: Range { start, end: self.get_last_end() }
        })
    }
    fn num(&mut self) -> Maybe<AstExpression> {
        let start = self.get_next_start();
        
        let mut val = self.expect_num()?;
        
        if self.peek() == TokenType::Period {
            self.consume();
            val = format!("{}.{}", val, self.expect_num()?);
        }
        
        
        let mut is_percentage = false;
        if self.peek() == TokenType::Mod {
            self.consume();
            is_percentage = true;
        }
        
        Ok(if is_percentage {
            AstExpression::Percentage {
                content: val.parse().unwrap(),
                range: Range { start, end: self.get_last_end() }
            }
        } else {
            AstExpression::Number {
                content: val.parse().unwrap(),
                range: Range { start, end: self.get_last_end() }
            }
        })
    }
}
