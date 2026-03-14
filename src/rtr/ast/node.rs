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
    Block {
        body: Vec<AstStatement>
    },
    Branch {
        cond: AstExpression,
        body: Box<AstStatement>,
        elifs: Vec<(AstExpression, AstStatement)>,
        else_body: Option<Box<AstStatement>>,
        range: Range
    },
    ConditionalLoop {
        kind: AstConditionalType,
        cond: AstExpression,
        body: Box<AstStatement>,
        range: Range
    },
    Repeat {
        amount: AstExpression,
        body: Box<AstStatement>,
        range: Range
    },
    For {
        var: String,
        iterator: AstExpression,
        body: Box<AstStatement>,
        range: Range
    },
    
    Expression(AstExpression)
}

#[derive(Debug, Clone)]
pub enum AstConditionalType {
    While,
    Until
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
    Assignment {
        op: AssignmentOp,
        target: Box<AstExpression>,
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
    Property {
        obj: Box<AstExpression>,
        key: PropertyKey,
        range: Range
    },
    
    // values
    Array {
        items: Vec<AstExpression>,
        range: Range
    },
    Object {
        pairs: Vec<(String, AstExpression)>,
        range: Range
    },
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
    },
    Func {
        params: Vec<String>,
        body: Box<AstStatement>,
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
pub enum AssignmentOp {
    Default,
    
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    
    NullishCoalescence
}
impl From<AssignmentOp> for BinaryOp {
    fn from(value: AssignmentOp) -> Self {
        match value {
            AssignmentOp::Default => panic!("cannot convert default to binary op"),
            AssignmentOp::Add => BinaryOp::Add,
            AssignmentOp::Sub => BinaryOp::Sub,
            AssignmentOp::Mul => BinaryOp::Mul,
            AssignmentOp::Div => BinaryOp::Div,
            AssignmentOp::Mod => BinaryOp::Mod,
            AssignmentOp::Pow => BinaryOp::Pow,
            
            AssignmentOp::NullishCoalescence => BinaryOp::NullishCoalescence
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String
}
#[derive(Debug, Clone)]
pub enum PropertyKey {
    Str(String),
    Expr(Box<AstExpression>)
}
