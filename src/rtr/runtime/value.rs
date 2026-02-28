use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::rtr::ast::node::Parameter;
use crate::rtr::error::Error;
use crate::rtr::runtime::instruction::VmInstruction;
use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::shared::color::Color;

#[derive(Debug, Clone)]
pub enum Value {
    Type { data: TypeValue },
    
    Null,
    Str { data: String },
    Num { data: f32 },
    Percentage { data: f32 },
    Bool { data: bool },
    Function(Function),
    Arr {
        elems: Vec<MemPointer>
    },
    Obj {
        data: HashMap<String, MemPointer>
    },
    Color { data: Color }
}

impl Value {
    #[allow(clippy::unused_self)]
    pub fn free(&self, _memory: &mut Memory) {
        // TODO: handle arrays
        // TODO: handle objects
    }
    
    pub fn call(&self, memory: &mut Memory, args: &[MemPointer]) -> Result<MemPointer, Error> {
        match self {
            Value::Function(Function::Builtin(builtin)) => {
                match builtin {
                    BuiltinFunction::Log => {
                        println!("{}",
                            args
                                .iter()
                                .map(|ptr| memory.get(*ptr))
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(" ")
                        );
                    }
                }
            }
            
            _ => {
                return Err(Error::CannotCall {
                    func: self.to_string()
                })
            }
        }
        
        Ok(memory.alloc(Value::Null))
    }
    
    pub fn get_type(&self) -> TypeValue {
        match self {
            Value::Type { .. } =>
                TypeValue::Type,
            
            Value::Null =>
                TypeValue::Null,
            Value::Str { .. } =>
                TypeValue::Str,
            Value::Num { .. } =>
                TypeValue::Num,
            Value::Percentage { .. } =>
                TypeValue::Percentage,
            Value::Bool { .. } =>
                TypeValue::Bool,
            Value::Function { .. } =>
                TypeValue::Function,
            Value::Arr { .. } =>
                TypeValue::Arr,
            Value::Obj { .. } =>
                TypeValue::Obj,
            Value::Color { .. } =>
                TypeValue::Color {},
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null =>
                write!(f, "null"),
            Value::Str { data } =>
                write!(f, "{data}"),
            Value::Num { data } =>
                write!(f, "{data}"),
            Value::Percentage { data } =>
                write!(f, "{data}%"),
            Value::Bool { data } =>
                write!(f, "{data}"),
            // TODO: arr & obj & color
            
            _ => write!(f, "<{}>", self.get_type())
        }
    }
}

impl From<Value> for f32 {
    fn from(value: Value) -> Self {
        match value {
            // TODO: str -> num
            Value::Num { data } | Value::Percentage { data } =>
                data,
            Value::Bool { data } =>
                data.into(),
            
            _ => Self::NAN
        }
    }
}

impl From<Value> for bool {
    fn from(value: Value) -> Self {
        match value {
            Value::Null =>
                false,
            Value::Str { data } =>
                !data.is_empty(),
            Value::Num { data } | Value::Percentage { data } =>
                data > 0.0,
            Value::Bool { data } =>
                data,
            
            _ => true
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeValue {
    Type,
    
    Null,
    Str,
    Num,
    Percentage,
    Bool,
    Function,
    Arr,
    Obj,
    Color
}

impl Display for TypeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeValue::Type => write!(f, "type"),
            
            TypeValue::Null => write!(f, "null"),
            TypeValue::Str => write!(f, "str"),
            TypeValue::Num => write!(f, "num"),
            TypeValue::Percentage => write!(f, "percentage"),
            TypeValue::Bool => write!(f, "bool"),
            TypeValue::Function => write!(f, "function"),
            TypeValue::Arr => write!(f, "arr"),
            TypeValue::Obj => write!(f, "obj"),
            TypeValue::Color => write!(f, "color"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Function {
    Builtin(BuiltinFunction),
    Vm {
        body: Vec<VmInstruction>,
        args: Vec<Parameter>
    },
}

#[derive(Debug, Clone)]
pub enum BuiltinFunction {
    Log
}
