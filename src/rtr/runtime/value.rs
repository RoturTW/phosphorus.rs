use std::fmt::{Debug, Display, Formatter};
use crate::rtr::error::Error;
use crate::rtr::{IndexKey};
use crate::rtr::log::{RTRLog};
use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::rtr::runtime::values::arr_value::RTRArr;
use crate::rtr::runtime::values::bool_value::RTRBool;
use crate::rtr::runtime::values::color_value::RTRColor;
use crate::rtr::runtime::values::function_value::RTRFunction;
use crate::rtr::runtime::values::null_value::RTRNull;
use crate::rtr::runtime::values::num_value::RTRNum;
use crate::rtr::runtime::values::obj_value::RTRObj;
use crate::rtr::runtime::values::percentage_value::RTRPercentage;
use crate::rtr::runtime::values::str_value::RTRStr;
use crate::rtr::runtime::values::type_value::RTRType;

pub trait RTRValue: Debug {
    fn clone_box(&self) -> Box<dyn RTRValue>;
    // memory management
    fn free(&self, memory: &mut Memory) {}
    fn dupe(&self, memory: &mut Memory) -> Box<dyn RTRValue>;
    //fn as_any(&self) -> &dyn std::any::Any;
    //fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    
    // general methods
    fn get_type(&self) -> TypeValue;
    fn call(&self, logs: &mut Vec<RTRLog>, memory: &mut Memory, args: &[MemPointer]) -> Result<MemPointer, Error> {
        Err(Error::CannotCall {
            func: self.stringify(memory)
        })
    }
    fn length(&self, memory: &Memory) -> usize { 0 }
    
    // conversion methods
    fn stringify(&self, memory: &Memory) -> String { format!("<{}>", self.get_type()) }
    fn stringify_format(&self, memory: &Memory) -> String { self.stringify(memory) }
    fn boolify(&self) -> bool { true }
    fn numbify(&self) -> f32 { f32::NAN }
    fn arrify(&self, memory: &mut Memory) -> Vec<MemPointer> { Vec::new() }
    
    // property stuff
    fn get_item(&self, memory: &mut Memory, key: &dyn RTRValue) -> MemPointer { memory.alloc(Box::new(RTRNull)) }
    fn set_item(&mut self, key: IndexKey, value: MemPointer) -> Option<MemPointer> { None }
    
    // object stuff
    fn keys(&self, memory: &mut Memory) -> Vec<MemPointer> { Vec::new() }
    fn values(&self, memory: &mut Memory) -> Vec<MemPointer> { Vec::new() }
    
    // comparisons
    fn has(&self, memory: &mut Memory, key: &dyn RTRValue) -> bool { false }
    fn equal(&self, other: &dyn RTRValue) -> bool { self.get_type() == other.get_type() }
    fn is_null(&self) -> bool { false }
    
    // to methods (e.g. RTRStr.to_str() == Some("data"))
    fn to_type(&self) -> Option<RTRType> { None }
    fn to_null(&self) -> Option<RTRNull> { None }
    fn to_str(&self) -> Option<RTRStr> { None }
    fn to_num(&self) -> Option<RTRNum> { None }
    fn to_percentage(&self) -> Option<RTRPercentage> { None }
    fn to_bool(&self) -> Option<RTRBool> { None }
    fn to_function(&self) -> Option<RTRFunction> { None }
    fn to_arr(&self) -> Option<RTRArr> { None }
    fn to_obj(&self) -> Option<RTRObj> { None }
    fn to_color(&self) -> Option<RTRColor> { None }
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

/*
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
        items: Vec<MemPointer>
    },
    Obj {
        data: HashMap<String, MemPointer>
    },
    Color { data: Color }
}

impl Value {
    pub fn free(&self, memory: &mut Memory) {
        match self {
            Value::Arr { items } => {
                for item in items {
                    memory.free(*item);
                }
            }
            Value::Obj { data } => {
                for item in data.values() {
                    memory.free(*item);
                }
            }
            
            _ => ()
        }
    }
    pub fn dupe(&self, memory: &mut Memory) -> Value {
        match self {
            Value::Arr { items } => {
                let mut new_items = Vec::new();
                for item in items {
                    let val = memory.get(*item).clone().dupe(memory);
                    new_items.push(memory.alloc(val));
                }
                Value::Arr {
                    items: new_items
                }
            }
            Value::Obj { data } => {
                let mut new_map = HashMap::new();
                for item in data {
                    let val = memory.get(*item.1).clone().dupe(memory);
                    new_map.insert(item.0.clone(), memory.alloc(val));
                }
                Value::Obj {
                    data: new_map
                }
            }
            _ => self.clone()
        }
    }
    
    pub fn call(&self, logs: &mut Vec<RTRLog>, memory: &mut Memory, args: &[MemPointer]) -> Result<MemPointer, Error> {
        match self {
            Value::Function(Function::Builtin(builtin)) => {
                builtin.call(logs, memory, args)
            }
            // Vm functions are handled in the call instruction
            
            _ => {
                Err(Error::CannotCall {
                    func: self.stringify(memory)
                })
            }
        }
        
        //Ok(memory.alloc(Value::Null))
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
    
    pub fn stringify(&self, memory: &Memory) -> String {
        match self {
            Value::Type { data } =>
                format!("<type:{data}>"),
            
            Value::Null =>
                String::from("null"),
            Value::Str { data } =>
                data.clone(),
            Value::Num { data } =>
                data.to_string(),
            Value::Percentage { data } =>
                format!("{data}%"),
            Value::Bool { data } =>
                data.to_string(),
            Value::Arr { items } => {
                let mut str = String::new();
                for item in items.iter().enumerate() {
                    str = format!("{str}{}", memory.get(*item.1).stringify_format(memory));
                    if item.0 < items.len() - 1 {
                        str = format!("{str}, ");
                    }
                }
                format!("[{str}]")
            }
            Value::Obj { data } => {
                let mut str = String::new();
                for (i, (key, ptr)) in data.iter().enumerate() {
                    str = format!("{str}{key}: {}", memory.get(*ptr).stringify_format(memory));
                    if i < data.len() - 1 {
                        str = format!("{str}, ");
                    }
                }
                format!("{{{str}}}")
            }
            Value::Color { data } => {
                data.to_hex_rgb()
            }
            
            _ => format!("<{}>", self.get_type())
        }
    }
    pub fn stringify_format(&self, memory: &Memory) -> String {
        match self {
            Value::Str { data } =>
                format!("{data:?}"),
            
            _ => self.stringify(memory)
        }
    }
    pub fn boolify(&self) -> bool {
        match self {
            Value::Null =>
                false,
            Value::Str { data } =>
                !data.is_empty(),
            Value::Num { data }
            | Value::Percentage { data } =>
                *data > 0.0,
            Value::Bool { data } =>
                *data,
            Value::Arr { items } =>
                !items.is_empty(),
            Value::Obj { data } =>
                !data.is_empty(),
            Value::Color { data } =>
                data.above_zero(),
            
            _ => true
        }
    }
    pub fn numbify(&self) -> f32 {
        match self {
            Value::Str { data } =>
                data.parse().unwrap_or(f32::NAN),
            Value::Num { data } =>
                *data,
            Value::Percentage { data } =>
                *data / 100.0,
            Value::Bool { data } =>
                if *data { 1.0 } else { 0.0 },
            
             _ => f32::NAN
        }
    }
    pub fn arrify(&self, memory: &mut Memory) -> Vec<MemPointer> {
        match self {
            Value::Str { data } => {
                data
                    .chars()
                    .map(|c| {
                        memory.alloc(Value::Str {
                            data: c.to_string()
                        })
                    })
                    .collect()
            }
            Value::Num { .. }
            | Value::Percentage { .. } => {
                self.stringify(memory)
                    .chars()
                    .map(|c| {
                        memory.alloc(Value::Str {
                            data: c.to_string()
                        })
                    })
                    .collect()
            }
            Value::Arr { items } => {
                items.clone()
            }
            Value::Obj { data } => {
                data
                    .keys()
                    .map(|key| {
                        memory.alloc(Value::Str {
                            data: key.clone()
                        })
                    })
                    .collect()
            }
            
            _ => Vec::new()
        }
    }
    pub fn length(&self) -> usize {
        match self {
            Value::Str { data } =>
                data.len(),
            Value::Num { data } | Value::Percentage { data } =>
                data.to_string().len(),
            Value::Arr { items } =>
                items.len(),
            Value::Obj { data } =>
                data.len(),
        
            _ => 0
        }
    }
    pub fn get_item(&self, memory: &mut Memory, key: &Value) -> MemPointer {
        // TODO: allow for negative indexes
        match self {
            Value::Str { data } =>
                // TODO: handle out of range
                memory.alloc(Value::Str {
                    data: data[key.numbify().trunc() as usize..].to_string()
                }),
            Value::Num { data } | Value::Percentage { data } =>
                // TODO: handle out of range
                memory.alloc(Value::Str {
                    data: data.to_string()[key.numbify().trunc() as usize..].to_string()
                }),
            Value::Arr { items } =>
                // TODO: handle out of range
                *items.get(key.numbify().trunc() as usize)
                    .unwrap_or(&memory.alloc(Value::Null)),
            Value::Obj { data } =>
                *data.get(&key.stringify(memory))
                    .unwrap_or(&memory.alloc(Value::Null)),
            
            _ => memory.alloc(Value::Null)
        }
    }
    pub fn set_item(&mut self, key: IndexKey, value: MemPointer) -> Option<MemPointer> {
        match self {
            Value::Obj { data } => {
                if let IndexKey::Str(key) = key {
                    data.insert(key, value)
                } else {
                    None
                }
            }
            
            _ => None
        }
    }
    pub fn keys(&self, memory: &mut Memory) -> Vec<MemPointer> {
        match self {
            Value::Arr { items } => {
                (0..items.len())
                    .map(|n| memory.alloc(Value::Num {
                        data: n as f32
                    }))
                    .collect()
            }
            Value::Obj { data } => {
                data.keys()
                    .map(|k| memory.alloc(Value::Str { data: k.clone() }))
                    .collect()
            }
            
            _ => Vec::new()
        }
    }
    pub fn values(&self, _memory: &mut Memory) -> Vec<MemPointer> {
        match self {
            Value::Arr { items } => {
                items.clone()
            }
            Value::Obj { data } => {
                data.values()
                    .copied()
                    .collect()
            }
            
            _ => Vec::new()
        }
    }
    pub fn has(&self, memory: &mut Memory, key: &Value) -> bool {
        match self {
            Value::Arr { items } => {
                for item in items {
                    if memory.get(*item).equal(key) {
                        return true;
                    }
                }
                false
            }
            Value::Obj { data } => {
                for item in data.keys() {
                    if key.equal(&Value::Str { data: item.clone() }) {
                        return true;
                    }
                }
                false
            }
            
            _ => false
        }
    }
    pub fn equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Type { data: a_data }, Value::Type { data: b_data }) =>
                a_data == b_data,
            
            (Value::Null, Value::Null) =>
                true,
            (Value::Str { data: a_data }, Value::Str { data: b_data }) =>
                a_data == b_data,
            (Value::Num { data: a_data }, Value::Num { data: b_data })
            | (Value::Percentage { data: a_data }, Value::Percentage { data: b_data }) =>
                (a_data - b_data).abs() < 0.000_000_1,
            (Value::Bool { data: a_data }, Value::Bool { data: b_data }) =>
                a_data == b_data,
            (Value::Color { data: a_data }, Value::Color { data: b_data }) =>
                a_data == b_data,
            
            _ => false
        }
    }
}

impl From<Value> for f32 {
    fn from(value: Value) -> Self {
        match value {
            Value::Str { data } =>
                data.parse().unwrap_or(Self::NAN),
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

#[derive(Debug, Clone)]
pub enum Function {
    Builtin(BuiltinFunction),
    Rust(fn(logs: &mut Vec<RTRLog>, memory: &mut Memory, args: &[MemPointer]) -> Result<MemPointer, Error>),
    Vm {
        body: Vec<VmInstruction>,
        params: Vec<Parameter>
    },
}


 */