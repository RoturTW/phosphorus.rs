use crate::shared::position::Position;
use crate::shared::range::Range;
use crate::shared::token::Token;

const SPLIT_CHARS: [char; 30] = [
    '(',')',
    
    ':','$',
    
    ' ','\n',
    
    // rtr
    '[',']',
    '{','}',
    ',',';','=','.','#','~',
    '+','-','*','/','%','^',
    '\\','<','>',
    '\'','"','`',
    '!','?'
];

pub fn tokenise(text: &str) -> Vec<Token> {
    let mut pos: Position = Position {
        ln: 1,
        col: 1,
        i: 0,
        script: text.to_string()
    };
    
    let mut tokens: Vec<Token> = Vec::new();
    let mut buf: String = String::new();
    let mut buf_start = pos.clone();
    
    for (i, char) in text.chars().enumerate() {
        if char == '\n' {
            pos.col = 0;
            pos.ln += 1;
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
