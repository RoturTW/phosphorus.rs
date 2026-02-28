use std::collections::HashMap;
use crate::rwl::ast::node::{AstHeader, AstHeaderItem, AstNode, AstValue};
use crate::rwl::ast::parser::Parser;
use crate::rwl::ast::tokenise;
use crate::rwl::element::{NodeWrapper, Node, Header, UpdateCtx, ContainerContext};
use crate::rwl::value::Value;
use crate::shared::area::Area;
use crate::shared::graphics::GLDrawHandle;

pub mod ast;
pub mod element;
pub mod error;
pub mod value;

#[derive(Debug)]
pub struct RWLInstance {
    pub ast: AstNode,
    pub root: NodeWrapper
}

impl RWLInstance {
    pub fn new() -> RWLInstance {
        RWLInstance {
            ast: AstNode::Empty,
            root: NodeWrapper::new(Node::Document { children: Vec::new() }),
        }
    }
    
    pub fn parse(&mut self, src: &str) {
        let mut parser = Parser {
            pointer: 0,
            tokens: tokenise(src)
        };
        
        let out = parser.parse();
        
        match out {
            Err(err) => {
                eprintln!("{err}");
            }
            Ok(ast) => {
                self.ast = ast;
            }
        }
    }
    
    pub fn instance(&mut self) {
        self.root = Self::instance_node(&self.ast);
    }
    fn instance_nodes(nodes: &[AstNode]) -> Vec<NodeWrapper> {
        nodes
            .iter()
            .map(RWLInstance::instance_node)
            .collect()
    }
    fn instance_node(node: &AstNode) -> NodeWrapper {
        match node {
            AstNode::Empty => NodeWrapper::new(
                Node::new_empty()
            ),
            
            AstNode::Document(children) => NodeWrapper::new(
                Node::new_document(
                    Self::instance_nodes(&children.clone())
                )
            ),
            AstNode::Block(block_type, header, children) => NodeWrapper::new(
                Node::new_block(
                    block_type.clone(),
                    Self::instance_nodes(&children.clone()),
                    Self::instance_header(header)
                )
            ),
            
            AstNode::Element(value, header) => NodeWrapper::new(
                Node::new_element(
                    Self::instance_value(value),
                    Self::instance_header(header)
                )
            ),
        }
    }
    fn instance_header(header: &AstHeader) -> Header {
        let mut pairs: HashMap<String, Value> = HashMap::new();
        let mut flags: Vec<String> = Vec::new();
        
        for attr in &header.attributes[0..] {
            match attr {
                AstHeaderItem::Pair(name, value) => {
                    pairs.insert(
                        name.clone(),
                        Self::instance_value(value)
                    );
                }
                AstHeaderItem::Flag(name) => {
                    flags.push(name.clone());
                }
            }
        }
        
        Header {
            pairs,
            flags
        }
    }
    fn instance_value(value: &AstValue) -> Value {
        match value {
            AstValue::Str(data) =>
                Value::Str(data.clone()),
            AstValue::Num(data) =>
                Value::Num(*data),
            AstValue::Percentage(data) =>
                Value::Percentage(*data),
            AstValue::Color(data) =>
                Value::Color(*data),
            AstValue::Property(path) =>
                Value::Property(path.clone())
        }
    }
    
    pub fn render(&mut self, d: &mut GLDrawHandle) {
        self.root.render(d);
    }
    pub fn update(&mut self, update_ctx: UpdateCtx, area: &Area) {
        let out = self.root.update(update_ctx, area, &mut ContainerContext::new());
        
        if let Err(err) = out {
            eprintln!("{err}");
        }
    }
}
