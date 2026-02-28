use crate::rwl::value::PropertyPath;
use crate::shared::color::Color;

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
