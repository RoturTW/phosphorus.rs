use crate::shared::position::Position;
use crate::shared::range::Range;
use crate::shared::token::Token;

pub mod parser;
pub mod node;

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

pub fn tokenise(text: &str) -> Vec<Token> {
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
    
    let mut pos: Position = Position {
        ln: 1,
        col: 1,
        i: 0,
        script: text.to_string()
    };
    
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf: String = String::new();
    let mut buf_start = pos.clone();
    
    let mut comment_type: crate::rtr::ast::CommentType = crate::rtr::ast::CommentType::None;
    let mut i: usize = 0;
    let vec_chars = text.chars().collect::<Vec<char>>();
    let chars = text.chars();
    for char in chars {
        if char == '\n' {
            pos.col = 0;
            pos.ln += 1;
        }
        
        match comment_type {
            crate::rtr::ast::CommentType::None => (),
            crate::rtr::ast::CommentType::Multiline => {
                if i > 0 && vec_chars[i - 1] == '*' && char == '/' {
                    comment_type = crate::rtr::ast::CommentType::None;
                }
                pos += 1;
                i += 1;
                continue;
            }
            crate::rtr::ast::CommentType::Singleline => {
                if char == '\n' {
                    comment_type = crate::rtr::ast::CommentType::None;
                } else {
                    pos += 1;
                    i += 1;
                }
                continue;
            }
        }
        
        if char == '/' && i < text.len() - 1 && vec_chars[i + 1] == '*' {
            comment_type = crate::rtr::ast::CommentType::Multiline;
            pos += 1;
            i += 1;
            continue;
        }
        if char == '/' && i < text.len() - 1 && vec_chars[i + 1] == '/' {
            comment_type = crate::rtr::ast::CommentType::Singleline;
            pos += 1;
            i += 1;
            continue;
        }
        
        if SPLIT_CHARS.contains(&char) {
            if !buf.is_empty() {
                tokens.push(Token {
                    token_type: buf.clone().into(),
                    range: Range {
                        start: buf_start.clone(),
                        end: pos.clone(),
                    }
                });
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
            if buf.is_empty() {
                buf_start = pos.clone();
            }
            buf.push(char);
            pos += 1;
        }
        
        i += 1;
    }
    
    if !buf.is_empty() {
        tokens.push(Token {
            token_type: buf.clone().into(),
            range: Range {
                start: buf_start.clone(),
                end: pos.clone(),
            }
        });
    }
    
    tokens
}
