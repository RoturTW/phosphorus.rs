use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::{print_raw, print_warn, Log, LogKind, print_log, print_error};
use crate::rtr::ast::node::Parameter;
use crate::rtr::error::Error;
use crate::rtr::{IndexKey};
use crate::rtr::log::{RTRLog, RTRLogKind};
use crate::rtr::runtime::instruction::VmInstruction;
use crate::rtr::runtime::memory::{MemPointer, Memory};
use crate::shared::color::Color;
use crate::shared::logging::LogSource;
use crate::shared::utils::{chr, ord};

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
    Rust(fn(logs: &mut Vec<RTRLog>, memory: &mut Memory, args: &[MemPointer]) -> Result<MemPointer, Error>),
    Vm {
        body: Vec<VmInstruction>,
        params: Vec<Parameter>
    },
}

#[derive(Debug, Clone)]
pub enum BuiltinFunction {
    Log,
    Error,
    Return,
    Typeof,
    Length,
    
    // mathematical
    Min,
    Max,
    
    Abs,
    Sqrt,
    
    Round,
    Floor,
    Ceil,
    
    // string
    Join,
    Split,
    
    Chr,
    Ord,
    
    ToUpper,
    ToLower,
    ToTitle,
    
    // array
    Item,
    Range,
    
    // object
    Keys,
    Values,
    Has,
    Obj,
    
    // logical
    All,
    Any,
    Not
}

impl BuiltinFunction {
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::unnecessary_wraps)]
    pub fn call(&self, logs: &mut Vec<RTRLog>, memory: &mut Memory, args: &[MemPointer]) -> Result<MemPointer, Error> {
        match self {
            BuiltinFunction::Log => {
                let text = args
                    .iter()
                    .map(|ptr| memory.get(*ptr))
                    .map(|v| v.stringify(memory))
                    .collect::<Vec<_>>()
                    .join(" ");
                
                print_log!(LogSource::Rtr, "{text}");
                
                logs.push(RTRLog {
                    kind: RTRLogKind::Log,
                    text
                });
            },
            BuiltinFunction::Error => {
                // TODO: have this error and then return and call the event
                let text = args
                    .iter()
                    .map(|ptr| memory.get(*ptr))
                    .map(|v| v.stringify(memory))
                    .collect::<Vec<_>>()
                    .join(" ");
                
                print_error!(LogSource::Rtr, "{text}");
                
                logs.push(RTRLog {
                    kind: RTRLogKind::Error,
                    text
                });
            },
            BuiltinFunction::Return => {
                // return is handled in call instruction
            }
            BuiltinFunction::Typeof => {
                return Ok(
                    memory.alloc(Value::Type {
                        data: memory.get(args[0]).get_type()
                    })
                )
            }
            BuiltinFunction::Length => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Value::Num {
                        data: memory.get(args[0]).length() as f32
                    })
                );
            }
            
            // mathematical
            BuiltinFunction::Min => {
                // TODO: check amount of args
                let mut iter = args.iter();
                let mut val = memory.get(*iter.next().unwrap()).numbify();
                
                for arg in iter {
                    let arg = memory.get(*arg).numbify();
                    val = val.min(arg);
                }
                
                return Ok(
                    memory.alloc(Value::Num {
                        data: val
                    })
                );
            }
            BuiltinFunction::Max => {
                // TODO: check amount of args
                let mut iter = args.iter();
                let mut val = memory.get(*iter.next().unwrap()).numbify();
                
                for arg in iter {
                    let arg = memory.get(*arg).numbify();
                    val = val.max(arg);
                }
                
                return Ok(
                    memory.alloc(Value::Num {
                        data: val
                    })
                );
            }
            
            BuiltinFunction::Abs => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Value::Num {
                        data: memory.get(args[0]).numbify().abs()
                    })
                );
            }
            BuiltinFunction::Sqrt => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Value::Num {
                        data: memory.get(args[0]).numbify().sqrt()
                    })
                );
            }
            
            // Round
            BuiltinFunction::Round => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Value::Num {
                        data: memory.get(args[0]).numbify().round()
                    })
                );
            }
            BuiltinFunction::Floor => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Value::Num {
                        data: memory.get(args[0]).numbify().floor()
                    })
                );
            }
            BuiltinFunction::Ceil => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Value::Num {
                        data: memory.get(args[0]).numbify().ceil()
                    })
                );
            }
            
            // string
            BuiltinFunction::Join => {
                // TODO: check amount of args
                return Ok(
                    memory.alloc(Value::Str {
                        data: args
                            .iter()
                            .map(|ptr| memory.get(*ptr).stringify(memory))
                            .reduce(|a, b| {
                                format!("{a}{b}")
                            }).unwrap_or(String::new())
                    })
                );
            }
            BuiltinFunction::Split => {
                // TODO: check amount of args
                let str = memory.get(args[0]).stringify(memory);
                let sep = memory.get(args[1]).stringify(memory);
                
                let val = Value::Arr {
                    items: str.split(sep.as_str())
                        .map(|s| memory.alloc(
                            Value::Str { data: s.to_string() }
                        ))
                        .collect()
                };
                
                return Ok(
                    memory.alloc(val)
                );
            }
            BuiltinFunction::Chr => {
                // TODO: check amount of args
                let num = memory.get(args[0]).numbify();
                return Ok(
                    memory.alloc(
                        Value::Str {
                            data: chr(num)
                        }
                    )
                )
            }
            BuiltinFunction::Ord => {
                // TODO: check amount of args
                let str = memory.get(args[0]).stringify(memory);
                return Ok(
                    memory.alloc(
                        Value::Num {
                            data: ord(&str)
                        }
                    )
                )
            }
            
            BuiltinFunction::ToUpper => {
                // TODO: handle incorrect amount of args
                let str = memory.get(args[0]).stringify(memory);
                return Ok(
                    memory.alloc(
                        Value::Str {
                            data: str.to_uppercase()
                        }
                    )
                )
            }
            BuiltinFunction::ToLower => {
                // TODO: handle incorrect amount of args
                let str = memory.get(args[0]).stringify(memory);
                return Ok(
                    memory.alloc(
                        Value::Str {
                            data: str.to_lowercase()
                        }
                    )
                )
            }
            BuiltinFunction::ToTitle => {
                // TODO: handle incorrect amount of args
                let str = memory.get(args[0]).stringify(memory);
                let words: Vec<String> = str.split(' ')
                    .map(|w| w[0..1].to_uppercase() + &w[1..].to_lowercase())
                    .collect();
                return Ok(
                    memory.alloc(
                        Value::Str {
                            data: words.join(" ")
                        }
                    )
                )
            }
            
            // array
            BuiltinFunction::Item => {
                // TODO: handle incorrect amount of args
                let value = memory.get(args[0]).clone();
                let idx = memory.get(args[1]).clone();
                return Ok(
                    value.get_item(memory, &idx)
                );
            }
            BuiltinFunction::Range => {
                let mut items = Vec::new();
                
                // TODO: handle incorrect amount of args
                let a = memory.get(args[0]).numbify().trunc() as usize;
                let b = memory.get(args[1]).numbify().trunc() as usize;
                
                for i in a..=b {
                    items.push(memory.alloc(
                        Value::Num { data: i as f32 }
                    ));
                }
                
                return Ok(memory.alloc(
                    Value::Arr { items }
                ))
            }
            
            // object
            BuiltinFunction::Keys => {
                // TODO: handle incorrect amount of args
                let keys = memory.get(args[0])
                    .clone()
                    .keys(memory);
                return Ok(memory.alloc(Value::Arr {
                    items: keys
                }));
            }
            BuiltinFunction::Values => {
                // TODO: handle incorrect amount of args
                let keys = memory.get(args[0])
                    .clone()
                    .values(memory);
                return Ok(memory.alloc(Value::Arr {
                    items: keys
                }));
            }
            BuiltinFunction::Has => {
                // TODO: handle incorrect amount of args
                let value = memory.get(args[0]).clone();
                let key = memory.get(args[1]).clone();
                let data = value.has(memory, &key);
                return Ok(memory.alloc(Value::Bool {
                    data
                }));
            }
            BuiltinFunction::Obj => {
                return Ok(memory.alloc(Value::Obj {
                    data: HashMap::new()
                }));
            }
            
            // logical
            BuiltinFunction::All => {
                return Ok(memory.alloc(Value::Bool {
                    data: args
                        .iter()
                        .all(|v| memory.get(*v).boolify())
                }))
            }
            BuiltinFunction::Any => {
                return Ok(memory.alloc(Value::Bool {
                    data: args
                        .iter()
                        .any(|v| memory.get(*v).boolify())
                }))
            }
            BuiltinFunction::Not => {
                // TODO: handle incorrect amount of args
                return Ok(memory.alloc(Value::Bool {
                    data: !memory.get(args[0]).boolify()
                }))
            }
        }
        
        Ok(memory.alloc(Value::Null))
    }
}
