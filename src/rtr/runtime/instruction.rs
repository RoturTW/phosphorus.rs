use crate::rtr::ast::node::{AssignmentOp, BinaryOp, Parameter, UnaryOp};
use crate::shared::color::Color;

#[derive(Debug, Clone)]
pub enum VmInstruction {
    // program flow
    Label(String),
    Jump(String),
    JumpIf(String),
    JumpNotIf(String),
    
    // operations
    Call(usize),
    CallEv(String),
    Unary(UnaryOp),
    Binary(BinaryOp),
    Prop,
    PropNoFree, // (used in for)
    Len,
    LenNoFree,  // (used in for)
    Arrify,
    
    // values
    Null,
    Str(String),
    Num(f32),
    Percentage(f32),
    Bool(bool),
    Func {
        body: Vec<VmInstruction>,
        args: Vec<Parameter>
    },
    Arr {
        len: usize
    },
    Obj {
        keys: Vec<String>
    },
    Color(Color),
    
    // scope
    Get(String),
    Decl(String),
    NewScope,
    PopScope,
    
    // assignments
    AsiVar(String, AssignmentOp),
    AsiProp(AssignmentOp),
    
    // stack
    Pop,
    Dupe(usize),
    DupePtr(usize),
    
    // memory
    AddRef,
    RmRef
}
