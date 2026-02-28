use crate::shared::range::Range;

#[derive(Debug, Clone)]
pub struct AstProgram {
    pub events: Vec<AstTopLevelStatement>
}

#[derive(Debug, Clone)]
pub enum AstTopLevelStatement {
    Event {
        body: Vec<AstStatement>,
        target: EventTarget
    }
}

#[derive(Debug, Clone)]
pub enum AstStatement {
    Expression(AstExpression)
}

#[derive(Debug, Clone)]
pub enum EventTarget {
    Global {
        name: String
    },
    Property {
        target: Target,
        event_name: String
    }
}

#[derive(Debug, Clone)]
pub enum Target {
    Any,
    Id(String),
    Element(String)
}

#[derive(Debug, Clone)]
pub enum AstExpression {
    // operations
    Call {
        args: Vec<AstExpression>,
        func: Box<AstExpression>,
        range: Range
    },
    Declare {
        name: String,
        value: Box<AstExpression>,
        range: Range
    },
    Unary {
        value: Box<AstExpression>,
        op: UnaryOp,
        range: Range
    },
    Binary {
        left: Box<AstExpression>,
        right: Box<AstExpression>,
        op: BinaryOp,
        range: Range
    },
    
    // accessing
    Variable {
        name: String,
        range: Range
    },
    
    // values
    String {
        content: String,
        range: Range
    },
    Number {
        content: f32,
        range: Range
    },
    Percentage {
        content: f32,
        range: Range
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Minus,
    Number,
    
    Invert,
    Boolify
}
#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    
    Eql,
    NotEql,
    
    Bigger,
    Smaller,
    BiggerEql,
    SmallerEql,
    
    NullishCoalescence
}
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String
}
